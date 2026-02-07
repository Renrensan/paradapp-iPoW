use crate::Engine;
use ethers_core::types::U256;
use futures::future::join_all;
use paradapp_core::{
    btc::btc_service::{btc_tip_height, maybe_rebalance_btc_wallets, rbf_send_to_user_program},
    consts::{
        supported_network_enum::SupportedNetwork, transaction_phase::TransactionPhase,
        transaction_type::TransactionType,
    },
    context::CoreContext,
    traits::{
        chain_provider_adapter::{
            BitcoinProgramType, ChainProviderAdapter, GlobalChainState, TxIdFilter,
        },
        chain_stack::ChainStack,
    },
};
use std::sync::Arc;
use tokio::try_join;
use tracing::{debug, error, info, warn};

pub struct ChainOperator;

pub struct BridgeIntent {
    pub stack: Arc<dyn ChainStack>,
    pub tx_ids: Vec<U256>,
}

impl ChainOperator {
    pub async fn run(
        stack: Arc<dyn ChainStack>,
        watch_sources: Vec<String>,
        engine: Engine,
    ) -> anyhow::Result<()> {
        let network_id = stack.network_id().to_string();
        info!(
            network = %network_id,
            watching = ?watch_sources,
            engine = ?engine,
            "Launching Operator Task(s)"
        );

        let has_watch_targets = !watch_sources.is_empty();
        let watch_sources = Arc::new(watch_sources);

        // We use a Vec to track handles so we can monitor whichever ones we actually start
        let mut handles = Vec::new();

        // 1. Approving Loop
        if engine == Engine::Approver || engine == Engine::All {
            let s = stack.clone();
            let ws = watch_sources.clone();
            handles.push(tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(13));
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                loop {
                    interval.tick().await;
                    if let Err(e) = Self::tick_approving(s.clone(), &ws).await {
                        warn!(network = %s.network_id(), error = %e, "Approving task failed");
                    }
                }
            }));
        }

        // 2. Converting Loop
        if engine == Engine::Converter || engine == Engine::All {
            let s = stack.clone();
            handles.push(tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(17));
                tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                loop {
                    interval.tick().await;
                    if let Err(e) = Self::tick_converting(s.clone()).await {
                        warn!(network = %s.network_id(), error = %e, "Converting task failed");
                    }
                }
            }));
        }

        // 3. Tunneling Loop (only runs if one or more watch target is specified)
        if (engine == Engine::Approver || engine == Engine::All) && has_watch_targets {
            let s = stack.clone();
            handles.push(tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(23));
                tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
                loop {
                    interval.tick().await;
                    if let Err(e) = Self::tick_tunneling(s.clone()).await {
                        warn!(network = %s.network_id(), error = %e, "Tunneling task failed");
                    }
                }
            }));
        }

        // 4. Streaming Loop
        if engine == Engine::Streamer || engine == Engine::All {
            let s = stack.clone();
            handles.push(tokio::spawn(async move {
                let mut interval = tokio::time::interval(std::time::Duration::from_secs(31));
                tokio::time::sleep(std::time::Duration::from_millis(3500)).await;
                loop {
                    interval.tick().await;
                    if let Err(e) = Self::tick_streaming(s.clone()).await {
                        warn!(network = %s.network_id(), error = %e, "Streaming task failed");
                    }
                }
            }));
        }

        // Monitor whichever handles were created
        if handles.is_empty() {
            return Err(anyhow::anyhow!("No engines selected to run"));
        }

        // Wait for a crash or a shutdown
        tokio::select! {
            res = futures::future::select_all(handles) => {
                error!(result = ?res.0, "One of the operator tasks exited unexpectedly");
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Shutdown signal received, stopping operator...");
            }
        }

        Ok(())
    }

    #[tracing::instrument(
        name = "operator_approving",
        skip(stack),
        fields(network = %stack.network_id())
    )]
    async fn tick_approving(
        stack: Arc<dyn ChainStack>,
        watch_sources: &[String],
    ) -> anyhow::Result<()> {
        let duty_seconds = 24 * 60 * 60;
        let provider = stack.chain_provider();
        let current_network = provider.network();
        let core_ctx = stack.core_context();

        let next_tx_id = provider.next_tx_id().await?;
        let to_tx_id = next_tx_id.saturating_sub(U256::one());

        // Fetch pending tx ids in parallel
        let (n2b, b2n, n2n_out) = try_join!(
            provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_BITCOIN,
                phase_filter: TransactionPhase::WAITING_OPERATOR_APPROVAL,
                to_tx_id,
                ..Default::default()
            }),
            provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::BITCOIN_TO_NATIVE,
                phase_filter: TransactionPhase::WAITING_OPERATOR_APPROVAL,
                to_tx_id,
                ..Default::default()
            }),
            provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                phase_filter: TransactionPhase::WAITING_OPERATOR_APPROVAL,
                to_tx_id,
                ..Default::default()
            })
        )?;

        // Log pending tx counts
        let pending_count = n2b.len() + b2n.len() + n2n_out.len();
        if pending_count > 0 {
            info!(
                n2b = n2b.len(),
                b2n = b2n.len(),
                n2n = n2n_out.len(),
                "Processing approvals"
            );
        }

        // Check RPC health, skip tick if unhealthy
        if (provider.check_rpc_health().await).is_err() {
            warn!("RPC unhealthy, skipping approval tick");
            return Ok(());
        }

        // Get this network's global chain state
        let state = provider.get_global_chain_state().await?;

        // Handle operator timeouts for this network tx's
        Self::handle_operator_timeouts(&stack, &state).await;

        // Determine if any bridge intents defined by watch_sources from other networks to this network are active
        // 1. Get the list of active intents
        let active_intents =
            Self::check_bridge_intent(core_ctx.clone(), watch_sources, current_network).await;
        let bridge_intent_active = !active_intents.is_empty();

        // 2. Handle sync/jump logic
        if bridge_intent_active || pending_count > 0 {
            // Collect network names for better logging
            let source_names: Vec<String> = active_intents
                .iter()
                .map(|i| i.stack.network_id().to_string())
                .collect();

            info!(
                remote_sources = ?source_names,
                local_pending = pending_count,
                "Bridge intents or pending txs detected; triggering sync logic"
            );

            // Pass the boolean to the sync logic handler
            Self::handle_sync_logic(stack.clone(), state, &active_intents).await?;
        } else {
            info!("No active intents or pending txs; skipping sync logic");
        }

        // Approve local N2B and B2N txs
        for tx_id in n2b.into_iter().chain(b2n) {
            let _ = stack.approving().approve_one_tx(tx_id, duty_seconds).await;
        }

        // Approve N2N_OUT txs if destination anchor height sufficient
        for tx_id in n2n_out {
            // Get conversion info to determine destination network
            let info = provider.get_conversion_info(tx_id).await?;
            let net_val = info.network_id.as_u32() as u8;

            // Map net_val to SupportedNetwork
            let dest_network = match SupportedNetwork::from_u8(net_val) {
                Some(net) => net,
                None => {
                    warn!(
                        val = net_val,
                        %tx_id,
                        "Received unknown network ID from conversion info"
                    );
                    continue;
                }
            };

            // If destination is in registry, attempt approval logic
            if let Ok(dest_stack) =
                crate::registry::Registry::get_stack(dest_network.as_str(), core_ctx.clone()).await
            {
                // Get min anchor height on destination
                let min_anchor_dest = dest_stack.chain_provider().min_anchor_height().await?;
                // Get current global tip on this network
                let current_global_tip = provider.global_tip_height().await?;

                // Only approve if min anchor on dest <= current global tip, else skip
                if min_anchor_dest <= current_global_tip {
                    info!(
                        %tx_id,
                        %dest_network,
                        %min_anchor_dest,
                        %current_global_tip,
                        "Anchor height sufficient, approving cross-chain tx"
                    );
                    let _ = stack.approving().approve_one_tx(tx_id, duty_seconds).await;
                } else {
                    warn!(
                        %tx_id,
                        %dest_network,
                        anchor = %min_anchor_dest,
                        global_tip = %current_global_tip,
                        "Anchor height not yet sufficient for cross-chain approval"
                    );
                }
            } else {
                warn!(%tx_id, %dest_network, "Could not find stack in registry for destination network");
            }
        }
        Ok(())
    }

    #[tracing::instrument(
        name = "operator_streaming", 
        skip(stack),
        fields(network = %stack.network_id())
    )]
    async fn tick_streaming(stack: Arc<dyn ChainStack>) -> anyhow::Result<()> {
        let provider = stack.chain_provider();
        let next_tx_id = provider.next_tx_id().await?;
        let to_tx_id = next_tx_id.saturating_sub(U256::one());

        // Separate fetches to track phase origin
        let active_ids = provider
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::ANY,
                phase_filter: TransactionPhase::ACTIVE_WAITING_PROOF,
                to_tx_id,
                max_results: U256::from(1000u64),
                ..Default::default()
            })
            .await?;

        let user_action_ids = provider
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::ANY,
                phase_filter: TransactionPhase::WAITING_USER_ACTION,
                to_tx_id,
                max_results: U256::from(1000u64),
                ..Default::default()
            })
            .await?;

        let mut max_target: u64 = 0;
        let mut trigger_reason = "none";

        // 1. Check Local Transactions
        // Combine with phase tagging for precise logging
        let local_checks = [
            (active_ids, TransactionPhase::ACTIVE_WAITING_PROOF),
            (user_action_ids, TransactionPhase::WAITING_USER_ACTION),
        ];

        let mut needed_tx_ids = Vec::new();
        for (ids, phase) in local_checks {
            for tx_id in ids {
                let stream_target = stack.streaming().compute_stream_target(tx_id).await?;
                if !stream_target.needed {
                    debug!(tx_id = %tx_id, ?phase, "Streaming not needed for tx");
                    continue;
                }

                needed_tx_ids.push(tx_id);

                if stream_target.target_height > max_target {
                    max_target = stream_target.target_height;
                    trigger_reason = "local_tx";
                    info!(
                        tx_id = %tx_id,
                        phase = ?phase,
                        new_target = max_target,
                        "Streaming triggered: Local transaction requires higher sync"
                    );
                }
            }
        }

        if max_target > 0 {
            info!(
                target_height = max_target,
                relevant_txs = needed_tx_ids.len(),
                reason = %trigger_reason,
                "Executing header push to global state"
            );

            stack
                .streaming()
                .push_headers_global(max_target, needed_tx_ids)
                .await?;
        }

        Ok(())
    }

    #[tracing::instrument(
        name = "operator_converting",
        skip(stack),
        fields(network = %stack.network_id())
    )]
    async fn tick_converting(stack: Arc<dyn ChainStack>) -> anyhow::Result<()> {
        // 1. Determine latest tx id
        let next_tx_id = stack.chain_provider().next_tx_id().await?;
        let to_tx_id = next_tx_id.saturating_sub(U256::one());

        if to_tx_id == U256::zero() {
            info!("No conversions exist yet.");
            return Ok(());
        }

        // 2. Get conversions needing processing
        let ready_h2b = stack
            .converting()
            .find_native_to_btc_ready(to_tx_id, None)
            .await?;
        let ready_b2h = stack
            .converting()
            .find_btc_to_native_completed(to_tx_id, None)
            .await?;

        if ready_h2b.is_empty() && ready_b2h.is_empty() {
            info!("No conversions requiring off-chain work this pass.");
        }

        // 3. Handle NATIVE → BTC
        let h2b_tx_ids: Vec<U256> = ready_h2b.iter().map(|(id, _)| *id).collect();
        let processed_map = stack
            .converting()
            .get_processed_native_to_btc(&h2b_tx_ids)
            .await?;

        for (tx_id, conv) in ready_h2b {
            // Case A: BTC already sent → check confirmation & submit proof
            if let Some(btc_txid) = processed_map.get(&tx_id) {
                match stack
                    .converting()
                    .check_confirmation_and_build_proof(tx_id, btc_txid)
                    .await?
                {
                    Some(proof) => {
                        if let Err(err) = stack.converting().submit_merkle_proof(tx_id, proof).await
                        {
                            warn!(%tx_id, ?err, "Error submitting merkle proof");
                        }
                    }
                    None => {
                        info!(%tx_id, "BTC transaction not yet confirmed, checking RBF criteria...");

                        // 1. Fetch Anchor Info from the contract
                        let anchor = stack.chain_provider().anchor_info(tx_id).await?;

                        // 2. Fetch current Bitcoin Tip Height
                        let tip_height = btc_tip_height(&stack.core_context()).await?;

                        // 3. Define thresholds (Configurable)
                        let anchor_threshold = anchor.anchor_height.as_u64() + 10;
                        let tip_lag_threshold = 1;

                        // Condition Check:
                        // Tip must be > (Anchor + 10) AND Tip must be > (some_reference_height + lag)
                        // For RBF, we usually check if tip_height - anchor_height > 10.
                        if tip_height > anchor_threshold
                            && tip_height >= (anchor.anchor_height.as_u64() + tip_lag_threshold)
                        {
                            info!(%tx_id, tip=%tip_height, anchor=%anchor.anchor_height, "RBF criteria met. Proceeding with fee bump.");

                            let amount_sats = conv.bitcoin_amount.as_u64();
                            let user_program = conv.user_program.0.to_vec();

                            match rbf_send_to_user_program(
                                &stack.core_context(),
                                btc_txid,
                                &user_program,
                                amount_sats,
                            )
                            .await
                            {
                                Ok(new_btc_txid) => {
                                    info!(%tx_id, old = %btc_txid, new = %new_btc_txid, "RBF Successful");

                                    // Update storage using your mark_processed (UPSERT) logic
                                    if let Err(e) = stack
                                        .converting()
                                        .mark_processed(tx_id, Some(new_btc_txid))
                                        .await
                                    {
                                        error!(%tx_id, "Failed to update RBF txid in storage: {:?}", e);
                                    }
                                }
                                Err(e) => {
                                    warn!(%tx_id, "RBF attempt failed or skipped by node: {:?}", e);
                                }
                            }
                        } else {
                            debug!(%tx_id, tip=%tip_height, anchor=%anchor.anchor_height, "RBF not needed yet.");
                        }
                    }
                }
                continue;
            }

            // Case B: Not processed yet → send BTC
            if let Err(err) = stack
                .converting()
                .handle_native_to_btc_conversion(tx_id, conv)
                .await
            {
                warn!(%tx_id, ?err, "Error handling NATIVE→BTC conversion");
            }
        }

        // 4. Handle BTC → NATIVE
        for (tx_id, conv) in ready_b2h {
            if let Err(err) = stack
                .converting()
                .handle_btc_to_native_conversion(tx_id, conv)
                .await
            {
                warn!(%tx_id, ?err, "Error handling BTC→NATIVE conversion");
            }
        }

        // 5. Liquidity Management
        let native_liq = stack.chain_provider().read_liquidity().await?;
        stack
            .chain_provider()
            .maybe_rebalance_contract_liquidity(native_liq)
            .await?;

        // 6. BTC hot wallet rebalance (uses core_ctx from stack)
        maybe_rebalance_btc_wallets(&stack.core_context()).await?;

        info!("Done conversion pass.");
        Ok(())
    }

    #[tracing::instrument(name = "operator_tunneling", skip(stack), fields(network = %stack.network_id()))]
    async fn tick_tunneling(stack: Arc<dyn ChainStack>) -> anyhow::Result<()> {
        let provider = stack.chain_provider();
        let core_ctx = stack.core_context();
        let next_tx_id = provider.next_tx_id().await?;
        let to_tx_id = next_tx_id.saturating_sub(U256::one());

        // Find outgoing bridge intents waiting for the "IN" side to be opened on the other chain
        // 1. Prepare the filters
        let user_action_filter = TxIdFilter {
            type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
            phase_filter: TransactionPhase::WAITING_USER_ACTION,
            to_tx_id,
            ..Default::default()
        };

        let mut waiting_proof_filter = user_action_filter.clone();
        waiting_proof_filter.phase_filter = TransactionPhase::ACTIVE_WAITING_PROOF;

        // 2. Parallel Execution
        let (user_action_res, proof_res) = tokio::try_join!(
            provider.get_tx_ids_by_filter(user_action_filter),
            provider.get_tx_ids_by_filter(waiting_proof_filter)
        )?;

        // 3. Combine results
        let mut intents = user_action_res;
        intents.extend(proof_res);

        for tx_id in intents {
            let info = provider.get_conversion_info(tx_id).await?;
            let net_val = info.network_id.as_u32() as u8;

            let dest_network = match SupportedNetwork::from_u8(net_val) {
                Some(net) => net,
                None => {
                    warn!(
                        val = net_val,
                        "Received unknown network ID from conversion info"
                    );
                    continue;
                }
            };

            // Get destination stack from registry
            if let Ok(dest_stack) =
                crate::registry::Registry::get_stack(dest_network.as_str(), core_ctx.clone()).await
            {
                // 1. Check if the tunnel is already open on the destination
                let dest_provider: Arc<dyn ChainProviderAdapter> = dest_stack.chain_provider();
                let filter = TxIdFilter {
                    type_filter: TransactionType::NATIVE_TO_NATIVE_IN,
                    bitcoin_program_filter: Some(info.user_program.clone()),
                    bitcoin_program_type: Some(BitcoinProgramType::Paradapp),
                    to_tx_id: dest_provider
                        .next_tx_id()
                        .await?
                        .saturating_sub(U256::one()),
                    max_results: U256::one(),
                    ..Default::default()
                };

                // 2. Prepare the specific filters for each phase
                let mut user_action_filter = filter.clone();
                user_action_filter.phase_filter = TransactionPhase::WAITING_USER_ACTION;

                let mut waiting_proof_filter = filter;
                waiting_proof_filter.phase_filter = TransactionPhase::ACTIVE_WAITING_PROOF;

                // 3. Run queries in parallel (Promise.all style)
                let already_opened = match tokio::try_join!(
                    dest_provider.get_tx_ids_by_filter(user_action_filter),
                    dest_provider.get_tx_ids_by_filter(waiting_proof_filter)
                ) {
                    Ok((user_list, proof_list)) => !user_list.is_empty() || !proof_list.is_empty(),
                    Err(e) => {
                        error!(error = %e, "Failed to check if intent is already opened");
                        false
                    }
                };

                if already_opened {
                    continue;
                }

                // // 2. Skip tick if destination not synced to source tip
                // let src_tip = provider.global_tip_height().await?.as_u64();
                // let dest_state = dest_provider.get_global_chain_state().await?;
                // if dest_state.global_tip < src_tip {
                //     warn!(
                //         %tx_id,
                //         %dest_network,
                //         dest_tip = dest_state.global_tip,
                //         needed = src_tip,
                //         "Destination not synced to source tip, skipping tunneling tick"
                //     );
                //     continue;
                // }

                // 3. Commit the "IN" side on destination (open tunnel)
                let source_anchor = provider.anchor_info(tx_id).await?;
                let btc_amount = provider
                    .estimate_bitcoin_from_native(info.native_amount)
                    .await?;

                info!(%tx_id, dest = %dest_network, "Opening bridge tunnel");
                dest_provider
                    .commit_bitcoin_to_native(
                        paradapp_core::traits::chain_provider_adapter::BitcoinToNativeCommitArgs {
                            bitcoin_amount: btc_amount,
                            network_id: U256::from(provider.network() as u8),
                            user_program: ethers::types::Bytes::new(),
                            dest_address: ethers::types::Address::from_slice(
                                &info.network_address.as_ref()[..20],
                            ),
                            network_address: ethers::types::Bytes::from(
                                info.user.as_bytes().to_vec(),
                            ),
                            duty_window_seconds: info.operator_duty_expires_at,
                            paradapp_receive_program: info.user_program,
                            locked_anchor_height: source_anchor.anchor_height,
                            slippage: info.slippage,
                        },
                    )
                    .await?;
            }
        }
        Ok(())
    }
}

// Helper
impl ChainOperator {
    async fn handle_operator_timeouts(stack: &Arc<dyn ChainStack>, state: &GlobalChainState) {
        if state.next_tx_id <= U256::one() {
            return;
        }
        let filter = TxIdFilter {
            to_tx_id: state.next_tx_id - U256::one(),
            ..Default::default()
        };
        let active = stack
            .chain_provider()
            .get_tx_ids_by_filter(TxIdFilter {
                phase_filter: TransactionPhase::ACTIVE_WAITING_PROOF,
                ..filter.clone()
            })
            .await
            .unwrap_or_default();
        let waiting = stack
            .chain_provider()
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::BITCOIN_TO_NATIVE,
                phase_filter: TransactionPhase::WAITING_USER_ACTION,
                ..filter
            })
            .await
            .unwrap_or_default();
        for tx_id in active.into_iter().chain(waiting) {
            let _ = stack
                .approving()
                .handle_operator_closes_for_active(tx_id, state.confirmations_required)
                .await;
        }
    }

    async fn handle_sync_logic(
        stack: Arc<dyn ChainStack>,
        state: GlobalChainState,
        active_intents: &[BridgeIntent],
    ) -> anyhow::Result<()> {
        let provider = stack.chain_provider();
        let network = stack.network_id();

        // 1. Determine the target height
        let mut target_height = state.safe_anchor;
        let mut target_reason = "local safe anchor";

        // Collect all anchor_info futures from all intents and all tx_ids
        let mut anchor_futures = Vec::new();
        for intent in active_intents {
            let remote_name = intent.stack.network_id();
            for &tx_id in &intent.tx_ids {
                let provider_clone = intent.stack.chain_provider().clone();
                anchor_futures.push(async move {
                    provider_clone
                        .anchor_info(tx_id)
                        .await
                        .map(|info| info.anchor_height.as_u64())
                        .map_err(|e| (tx_id, remote_name, e))
                });
            }
        }

        if !anchor_futures.is_empty() {
            debug!(%network, count = anchor_futures.len(), "Fetching remote anchor heights in parallel");
        }

        // Execute all requests in parallel
        let results = futures::future::join_all(anchor_futures).await;

        // Log errors for failed anchor lookups
        for res in results.iter() {
            if let Err((tx_id, r_network, err)) = res {
                warn!(%network, remote = %r_network, %tx_id, error = %err, "Failed to fetch remote anchor info");
            }
        }

        let min_remote_anchor = results.into_iter().filter_map(|r| r.ok()).min();

        // If we found remote intents, our sync target is the minimum anchor height required
        if let Some(remote_target) = min_remote_anchor {
            // We only update if the remote requirement is actually different/relevant
            if remote_target != state.safe_anchor {
                target_height = remote_target;
                target_reason = "lowest remote bridge intent anchor";
            }
        }

        let gap = target_height.saturating_sub(state.global_tip);
        if gap == 0 {
            debug!(%network, current = state.global_tip, target = target_height, "No sync gap found; chain is up to date");
            return Ok(());
        }

        info!(
            %network,
            current = state.global_tip,
            target = target_height,
            gap = gap,
            reason = %target_reason,
            active_txs = state.active_open,
            "Sync gap detected; initiating synchronization"
        );

        // 2. Sync Execution
        // Reensure state is fresh before deciding on jump vs stream
        // let state = provider.get_global_chain_state().await?;
        if state.active_open == 0 {
            info!(%network, %target_height, "No active conversions in source chain → jumping to target height");
            provider
                .jump_to_anchor_from_zero_active(state.global_tip, target_height)
                .await?;
        } else {
            let user_close_candidates = stack
                .approving()
                .discover_user_close_candidates(
                    state.next_tx_id - U256::one(),
                    state.confirmations_required,
                )
                .await
                .unwrap_or_default();

            let user_close_cost = user_close_candidates.len();

            info!(
                %network,
                stream_gap = gap,
                user_close_cost = user_close_cost,
                "Source chain user-close vs stream decision"
            );

            if user_close_cost > 0 && (gap as usize) > user_close_cost {
                info!(%network, "Cheaper to user-close than stream → executing");

                let candidates: Vec<(U256, &'static str)> = user_close_candidates
                    .into_iter()
                    .map(|(tx_id, kind)| {
                        if kind.contains("refundAfterNoProof") {
                            (tx_id, "refundAfterNoProof_NativeTokentoBTC")
                        } else {
                            (tx_id, "claimNative_AfterOperatorExpired")
                        }
                    })
                    .collect();

                stack.approving().execute_user_closes(candidates).await?;

                // REFRESH STATE: Check if we can jump now
                let refreshed = provider.get_global_chain_state().await?;
                if refreshed.active_open == 0 {
                    info!(%network, "All active closed → jumping to target height");
                    provider
                        .jump_to_anchor_from_zero_active(refreshed.global_tip, target_height)
                        .await?;
                }
            } else {
                info!(%network, "Streaming headers is cheaper or no candidates found");
                stack
                    .streaming()
                    .stream_headers_to_height(state.global_tip, state.safe_anchor, 200)
                    .await?;
            }
        }

        Ok(())
    }

    async fn check_bridge_intent(
        ctx: Arc<CoreContext>,
        sources: &[String],
        current: SupportedNetwork,
    ) -> Vec<BridgeIntent> {
        // Create a list of futures to execute in parallel
        let tasks = sources.iter().map(|source| {
            let ctx = ctx.clone();
            let source = source.clone();

            async move {
                if let Ok(remote_stack) = crate::registry::Registry::get_stack(&source, ctx).await {
                    let remote_provider = remote_stack.chain_provider();

                    // Run the two filter queries for this specific network
                    let (approval_res, user_action_res) = tokio::join!(
                        remote_provider.get_tx_ids_by_filter(TxIdFilter {
                            type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                            phase_filter: TransactionPhase::WAITING_OPERATOR_APPROVAL,
                            dest_network: Some(current),
                            max_results: U256::from(100u64),
                            ..Default::default()
                        }),
                        remote_provider.get_tx_ids_by_filter(TxIdFilter {
                            type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                            phase_filter: TransactionPhase::WAITING_USER_ACTION,
                            dest_network: Some(current),
                            max_results: U256::from(100u64),
                            ..Default::default()
                        })
                    );

                    let mut all_ids = approval_res.unwrap_or_default();
                    all_ids.extend(user_action_res.unwrap_or_default());

                    if !all_ids.is_empty() {
                        all_ids.sort();
                        all_ids.dedup();

                        info!(
                            from_network = %source,
                            count = all_ids.len(),
                            "Found active bridge intents (Approval + User Action)"
                        );

                        return Some(BridgeIntent {
                            stack: remote_stack,
                            tx_ids: all_ids,
                        });
                    }
                }
                None
            }
        });

        // Execute all network checks in parallel (Promise.all)
        let results = join_all(tasks).await;

        // Filter out the Nones (networks with no intents or errors)
        results.into_iter().flatten().collect()
    }
}
