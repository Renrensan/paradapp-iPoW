use crate::Engine;
use ethers_core::types::U256;
use futures::future::join_all;
use paradapp_core::{
    btc::btc_service::{btc_tip_height}, consts::{
        supported_network_enum::SupportedNetwork,
        transaction_phase::TransactionPhase, transaction_type::TransactionType,
    }, dependencies::context::CoreContext, supra::supra_service, traits::{
        chain_provider_adapter::{
            BitcoinProgramType, GlobalChainState, TxIdFilter,
        },
        chain_stack::ChainStack,
    }
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
        let parsed_watch_sources: Vec<String> = watch_sources
            .into_iter()
            .flat_map(|s| {
                s.split(',')
                    .map(|part| part.trim().to_lowercase())
                    .filter(|part| !part.is_empty())
                    .collect::<Vec<_>>()
            })
            .collect();

        let network_id = stack.network_id().to_string();
        info!(
            network = %network_id,
            watching = ?parsed_watch_sources,
            engine = ?engine,
            "Launching Operator Task(s)"
        );

        let has_watch_targets = !parsed_watch_sources.is_empty();
        let watch_sources_arc = Arc::new(parsed_watch_sources);

        let mut handles = Vec::new();

        if engine == Engine::Approver || engine == Engine::All {
            let s = stack.clone();
            let ws = watch_sources_arc.clone();
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

        if (engine == Engine::Approver || engine == Engine::All)
            && has_watch_targets
        {
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

        if engine == Engine::Rebalance || engine == Engine::All {
            let s = stack.clone();
            handles.push(tokio::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                info!("Running initial startup BTC sweep check");
                let _ = Self::tick_sweeping(s.clone()).await;

                let one_day = std::time::Duration::from_secs(24 * 60 * 60);

                let mut interval = tokio::time::interval_at(
                    tokio::time::Instant::now() + one_day,
                    one_day,
                );

                loop {
                    interval.tick().await;
                    info!("Starting scheduled 1-day BTC sweep check");
                    let _ = Self::tick_sweeping(s.clone()).await;
                }
            }));
        }

        if handles.is_empty() {
            return Err(anyhow::anyhow!("No engines selected to run"));
        }

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

        let (n2b_raw, b2n_raw, n2n_out_raw) = try_join!(
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

        let min_limit = U256::from(provider.min_transaction_limit());
        let max_limit = U256::from(provider.max_transaction_limit());

        let filter_by_limits = |ids: Vec<U256>| async {
            let mut filtered = Vec::new();
            for id in ids {
                if let Ok(conv) = provider.get_conversion_info(id).await {
                    let amount_to_check = conv.native_amount;

                    if amount_to_check >= min_limit
                        && amount_to_check <= max_limit
                    {
                        filtered.push(id);
                    } else {
                        info!(
                            tx_id = %id,
                            amount = %amount_to_check,
                            is_n2b = %conv.is_native_to_bitcoin,
                            "Transaction skipped: native amount outside configured limits"
                        );
                    }
                }
            }
            filtered
        };

        let (n2b, b2n, n2n_out) = tokio::join!(
            filter_by_limits(n2b_raw),
            filter_by_limits(b2n_raw),
            filter_by_limits(n2n_out_raw)
        );

        let pending_count = n2b.len() + b2n.len() + n2n_out.len();
        if pending_count > 0 {
            if (provider.check_rpc_health().await).is_err() {
                warn!("RPC unhealthy, skipping approval tick");
                return Ok(());
            }
            info!(
                n2b = n2b.len(),
                b2n = b2n.len(),
                n2n = n2n_out.len(),
                "Processing approvals"
            );
        }

        let state = provider.get_global_chain_state().await?;

        Self::handle_operator_timeouts(&stack, &state).await;

        let active_intents = Self::check_bridge_intent(
            core_ctx.clone(),
            watch_sources,
            current_network,
        )
        .await;
        let bridge_intent_active = !active_intents.is_empty();

        if bridge_intent_active || pending_count > 0 {
            let source_names: Vec<String> = active_intents
                .iter()
                .map(|i| i.stack.network_id().to_string())
                .collect();

            info!(
                remote_sources = ?source_names,
                local_pending = pending_count,
                "Bridge intents or pending txs detected; triggering sync logic"
            );

            Self::handle_sync_logic(stack.clone(), state, &active_intents)
                .await?;
        } else {
            info!("No active intents or pending txs; skipping sync logic");
        }

        // Attempt approve local tx
        let current_tip_height = paradapp_core::btc::btc_service::btc_tip_height(&stack.core_context()).await? as u64;
        let global_tip_height = provider.global_tip_height().await?;

        info!(
            current_tip_height = %current_tip_height,
            global_tip_height = %global_tip_height,
            "Current Bitcoin tip height and global tip height fetched for approval checks"
        );

        if current_tip_height == global_tip_height.as_u64() {
            for tx_id in n2b.into_iter().chain(b2n) {
                let _ = stack.approving().approve_one_tx(tx_id, duty_seconds).await;
            }

            // Attempt approve cross chain tx
            for tx_id in n2n_out {
                let info = provider.get_conversion_info(tx_id).await?;
                let net_val = info.network_id.as_u32() as u8;

                let dest_network = match SupportedNetwork::from_u8(net_val) {
                    Some(net) => net,
                    None => {
                        warn!(
                            val = net_val,
                            %tx_id,
                            "Received unknown network ID from conversion info"
                        );
                        continue;
                    },
                };

                if let Ok(dest_stack) = crate::registry::Registry::get_stack(
                    dest_network.as_str(),
                    core_ctx.clone(),
                )
                .await
                {
                    let min_anchor_dest =
                        dest_stack.chain_provider().min_anchor_height().await?;
                    let current_global_tip = provider.global_tip_height().await?;

                    if min_anchor_dest <= current_global_tip {
                        info!(
                            %tx_id,
                            %dest_network,
                            %min_anchor_dest,
                            %current_global_tip,
                            "Anchor height sufficient, approving cross-chain tx"
                        );
                        let _ = stack
                            .approving()
                            .approve_one_tx(tx_id, duty_seconds)
                            .await;
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

        let local_checks = [
            (active_ids, TransactionPhase::ACTIVE_WAITING_PROOF),
            (user_action_ids, TransactionPhase::WAITING_USER_ACTION),
        ];

        let mut needed_tx_ids = Vec::new();
        for (ids, phase) in local_checks {
            if !ids.is_empty() {
                info!(
                    count = ids.len(),
                    ?phase,
                    "Found potential transactions for streaming"
                );
            }

            for tx_id in ids {
                let stream_target =
                    stack.streaming().compute_stream_target(tx_id).await?;

                // Log the computation result for every tx found
                info!(
                    tx_id = %tx_id,
                    ?phase,
                    needed = %stream_target.needed,
                    target = %stream_target.target_height,
                    "Computed stream target"
                );

                if !stream_target.needed {
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
                        "Streaming threshold updated: Local transaction sets new max target"
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
        } else {
            // Useful to see in logs when the operator is idle
            debug!("Streaming tick complete: No headers need pushing.");
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
        let ready_h2b =
            stack.converting().find_native_to_btc_ready(to_tx_id, None).await?;
        let ready_b2h = stack
            .converting()
            .find_btc_to_native_completed(to_tx_id, None)
            .await?;

        // Extract just the IDs from the tuples for the log
        let h2b_ids: Vec<U256> = ready_h2b.iter().map(|(id, _)| *id).collect();
        let b2h_ids: Vec<U256> = ready_b2h.iter().map(|(id, _)| *id).collect();

        if !ready_h2b.is_empty() || !ready_b2h.is_empty() {
            info!(
                h2b_count = ready_h2b.len(),
                b2h_count = ready_b2h.len(),
                h2b_members = ?h2b_ids,
                b2h_members = ?b2h_ids,
                "Conversion status updated"
            );
        } else {
            info!("No conversions found.");
        }

        // 3. Handle NATIVE → BTC
        let h2b_tx_ids: Vec<U256> =
            ready_h2b.iter().map(|(id, _)| *id).collect();
        let processed_map =
            stack.converting().get_processed_native_to_btc(&h2b_tx_ids).await?;

        for (tx_id, conv) in ready_h2b {
            // Case A: BTC already sent → check confirmation & submit proof
            if let Some(btc_txid) = processed_map.get(&tx_id) {
                // --- CACHE CHECK START ---
                // Fetch submitted proof info from contract
                let info = stack.chain_provider().proof_info(tx_id).await?;
                info!(
                    %tx_id,
                    proof_h = %info.block_height,
                    attempts = %info.attempts,
                    "Fetched existing proof info for transaction"
                );

                // Check if cached: attempts > 0 and current_tip < proof_block_height
                if info.attempts > 0 {
                    let chain_height =
                        stack.chain_provider().global_tip_height().await?;
                        info!(
                            %tx_id,
                            proof_h = %info.block_height,
                            tip_h = %chain_height,
                            attempts = %info.attempts,
                            "Transaction has existing proof attempts. Checking if chain height is below proof block height."
                        );

                    if chain_height < info.block_height {
                        info!(
                            %tx_id,
                            proof_h = %info.block_height,
                            tip_h = %chain_height,
                            attempts = %info.attempts,
                            "Merkle proof already cached. Skipping height check."
                        );
                        continue;
                    }
                }
                
                // --- CACHE CHECK END ---

                match stack
                    .converting()
                    .check_confirmation_and_build_proof(tx_id, btc_txid)
                    .await?
                {
                    Some(proof) => {
                        // If block_height is > 0, it is confirmed on Bitcoin
                        if !proof.block_height.is_zero() {
                            if let Err(err) = stack
                                .converting()
                                .submit_merkle_proof(tx_id, proof)
                                .await
                            {
                                warn!(%tx_id, ?err, "Error submitting merkle proof");
                            }
                        } else {
                            // Transaction is still in mempool, use the fetched estimated_confirmation_height for RBF
                            info!(%tx_id, "BTC transaction not yet confirmed, evaluating RBF eligibility...");

                            // 1. Fetch Anchor Info from the contract
                            let anchor = stack
                                .chain_provider()
                                .anchor_info(tx_id)
                                .await?;

                            // 2. Fetch current Bitcoin Tip Height
                            let current_tip_height =
                                btc_tip_height(&stack.core_context()).await?;

                            // 3. Define thresholds (Configurable)
                            let anchor_block_threshold = stack
                                .core_context()
                                .cfg
                                .rbf_blocks_since_anchor; // Min blocks since anchor tx before RBF allowed

                            let confirmation_delay_threshold_blocks = stack
                                .core_context()
                                .cfg
                                .rbf_blocks_from_tip_to_unconfirmed; // Max acceptable delay before RBF

                            let anchor_height = anchor.anchor_height.as_u64();
                            let anchor_rbf_eligible_height =
                                anchor_height + anchor_block_threshold;

                            let is_past_anchor_threshold = current_tip_height
                                >= anchor_rbf_eligible_height;

                            // Use the estimated confirmation height returned in the proof payload
                            let estimated_confirmation_height = proof
                                .mempool_height
                                .unwrap_or(current_tip_height);

                            let estimated_blocks_until_confirmation =
                                estimated_confirmation_height
                                    .saturating_sub(current_tip_height);

                            let is_confirmation_delay_over_threshold =
                                estimated_blocks_until_confirmation
                                    > confirmation_delay_threshold_blocks;

                            info!(
                                tx_id = %tx_id,
                                current_tip_height = %current_tip_height,
                                anchor_height = %anchor_height,
                                anchor_block_threshold = %anchor_block_threshold,
                                anchor_rbf_eligible_height = %anchor_rbf_eligible_height,
                                is_past_anchor_threshold = %is_past_anchor_threshold,
                                estimated_confirmation_height = %estimated_confirmation_height,
                                estimated_blocks_until_confirmation = %estimated_blocks_until_confirmation,
                                confirmation_delay_threshold_blocks = %confirmation_delay_threshold_blocks,
                                is_confirmation_delay_over_threshold = %is_confirmation_delay_over_threshold,
                                "RBF eligibility parameters evaluated"
                            );
                            if is_past_anchor_threshold
                                && is_confirmation_delay_over_threshold
                            {
                                info!(
                                    %tx_id,
                                    current_tip_height = %current_tip_height,
                                    anchor_height = %anchor_height,
                                    estimated_confirmation_height = %estimated_confirmation_height,
                                    estimated_blocks_until_confirmation = %estimated_blocks_until_confirmation,
                                    "RBF criteria met. Proceeding with fee bump."
                                );

                                // let amount_sats = conv.bitcoin_amount.as_u64();
                                // let user_program = conv.user_program.0.to_vec();

                                // match rbf_send_to_user_program(
                                //     &stack.core_context(),
                                //     btc_txid,
                                //     &user_program,
                                //     amount_sats,
                                // )
                                // .await
                                // {
                                //     Ok(new_btc_txid) => {
                                //         info!(
                                //             %tx_id,
                                //             old_btc_txid = %btc_txid,
                                //             new_btc_txid = %new_btc_txid,
                                //             "RBF successful"
                                //         );

                                //         // Update storage
                                //         if let Err(e) = stack
                                //             .converting()
                                //             .mark_processed(
                                //                 tx_id,
                                //                 Some(new_btc_txid),
                                //             )
                                //             .await
                                //         {
                                //             error!(
                                //                 %tx_id,
                                //                 "Failed to update RBF txid in storage: {:?}",
                                //                 e
                                //             );
                                //         }
                                //     },
                                //     Err(e) => {
                                //         warn!(
                                //             %tx_id,
                                //             current_tip_height = %current_tip_height,
                                //             estimated_confirmation_height = %estimated_confirmation_height,
                                //             estimated_blocks_until_confirmation = %estimated_blocks_until_confirmation,
                                //             "RBF attempt failed or skipped by node: {:?}",
                                //             e
                                //         );
                                //     },
                                // }
                            } else {
                                debug!(
                                    %tx_id,
                                    current_tip_height = %current_tip_height,
                                    anchor_height = %anchor_height,
                                    anchor_rbf_eligible_height = %anchor_rbf_eligible_height,
                                    is_past_anchor_threshold = %is_past_anchor_threshold,
                                    estimated_confirmation_height = %estimated_confirmation_height,
                                    estimated_blocks_until_confirmation = %estimated_blocks_until_confirmation,
                                    confirmation_delay_threshold_blocks = %confirmation_delay_threshold_blocks,
                                    is_confirmation_delay_over_threshold = %is_confirmation_delay_over_threshold,
                                    "RBF not needed yet"
                                );
                            }
                        }
                    },
                    None => {
                        // This handles the API error/retry case from check_confirmation_and_build_proof
                    },
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
        for (tx_id, _conv) in ready_b2h {
            if let Err(err) =
                stack.converting().handle_btc_to_native_conversion(tx_id).await
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
        // maybe_rebalance_btc_wallets(&stack.core_context()).await?;

        info!("Done conversion pass.");
        Ok(())
    }

    #[tracing::instrument(name = "operator_tunneling", skip(stack), fields(network = %stack.network_id()))]
    async fn tick_tunneling(stack: Arc<dyn ChainStack>) -> anyhow::Result<()> {
        let provider = stack.chain_provider();
        let core_ctx = stack.core_context();
        let next_tx_id = provider.next_tx_id().await?;
        let to_tx_id = next_tx_id.saturating_sub(U256::one());

        let user_action_filter = TxIdFilter {
            type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
            phase_filter: TransactionPhase::WAITING_USER_ACTION,
            to_tx_id,
            ..Default::default()
        };

        let mut waiting_proof_filter = user_action_filter.clone();
        waiting_proof_filter.phase_filter = TransactionPhase::ACTIVE_WAITING_PROOF;

        let (user_action_res, proof_res) = tokio::try_join!(
            provider.get_tx_ids_by_filter(user_action_filter),
            provider.get_tx_ids_by_filter(waiting_proof_filter)
        )?;

        let mut intents = user_action_res;
        intents.extend(proof_res);

        for tx_id in intents {
            if let Err(e) = async {
                let info = provider.get_conversion_info(tx_id).await?;
                let net_val = info.network_id.as_u32() as u8;

                if net_val == 0 {
                    return Ok::<(), anyhow::Error>(());
                }

                let dest_network = SupportedNetwork::from_u8(net_val)
                    .ok_or_else(|| anyhow::anyhow!("Unknown network ID: {}", net_val))?;

                let dest_stack =
                    crate::registry::Registry::get_stack(dest_network.as_str(), core_ctx.clone())
                        .await
                        .map_err(|e| {
                            anyhow::anyhow!("Registry failed for {}: {}", dest_network, e)
                        })?;

                let dest_provider = dest_stack.chain_provider();

                let dest_next_id = dest_provider.next_tx_id().await?;
                let filter = TxIdFilter {
                    type_filter: TransactionType::NATIVE_TO_NATIVE_IN,
                    bitcoin_program_filter: Some(info.user_program.clone()),
                    bitcoin_program_type: Some(BitcoinProgramType::Paradapp),
                    to_tx_id: dest_next_id.saturating_sub(U256::one()),
                    max_results: U256::one(),
                    ..Default::default()
                };

                let mut ua_filter = filter.clone();
                ua_filter.phase_filter = TransactionPhase::WAITING_USER_ACTION;

                let mut wp_filter = filter;
                wp_filter.phase_filter = TransactionPhase::ACTIVE_WAITING_PROOF;

                let (user_list, proof_list) = tokio::try_join!(
                    dest_provider.get_tx_ids_by_filter(ua_filter),
                    dest_provider.get_tx_ids_by_filter(wp_filter)
                )?;

                if !user_list.is_empty() || !proof_list.is_empty() {
                    return Ok(()); 
                }

                let market_ratio = supra_service::get_token_price_vs_btc(
                    &core_ctx,
                    dest_network,
                    Some(supra_service::SupraNetwork::Testnet),
                )
                .await?;

                let native_decimals = match dest_network {
                    SupportedNetwork::HEDERA => 8,
                    SupportedNetwork::SOLANA => 9,
                    _ => 18, 
                };
                let btc_float = info.bitcoin_amount.as_u64() as f64 / 100_000_000.0;
                let market_native_float = btc_float / market_ratio;
                let safe_native_float = market_native_float * 0.995;
                let safe_native_amount = U256::from((safe_native_float * 10f64.powi(native_decimals)) as u128);

                let mut raw_user_address_32 = vec![0u8; 32];
                raw_user_address_32[..20].copy_from_slice(info.user.as_bytes());

                let mut dest_addr_bytes = info.network_address.to_vec();
                if dest_addr_bytes.starts_with(b"0x") || dest_addr_bytes.starts_with(b"0X") {
                    let hex_str = String::from_utf8_lossy(&dest_addr_bytes);
                    let clean_hex = hex_str.trim_start_matches("0x").trim_start_matches("0X");
                    
                    if let Ok(decoded) = (0..clean_hex.len())
                        .step_by(2)
                        .map(|i| u8::from_str_radix(&clean_hex[i..std::cmp::min(i + 2, clean_hex.len())], 16))
                        .collect::<Result<Vec<u8>, _>>()
                    {
                        dest_addr_bytes = decoded;
                    }
                }

                let network_id_u8 = provider.network() as u8;
                let source_anchor = provider.anchor_info(tx_id).await?;
                let now = chrono::Utc::now().timestamp();
                let now_u256 = U256::from(now as u64);
                let duty_window = info.operator_duty_expires_at.saturating_sub(now_u256);

                if dest_network == SupportedNetwork::SOLANA {
                    let network_address_bytes = ethers::types::Bytes::from(info.user.as_bytes().to_vec());
                    
                    info!(
                        %tx_id, 
                        dest = %dest_network, 
                        requested = %info.native_amount, 
                        calculated_payout = %safe_native_amount, 
                        dest_address = ?dest_addr_bytes,
                        network_address = %network_address_bytes,
                        "Opening bridge tunnel with market-adjusted native amount (Solana Dest)"
                    );

                    dest_provider
                        .commit_bitcoin_to_native(
                            paradapp_core::traits::chain_provider_adapter::BitcoinToNativeCommitArgs {
                                bitcoin_amount: info.bitcoin_amount,
                                native_amount: safe_native_amount,
                                network_id: U256::from(network_id_u8),
                                user_program: ethers::types::Bytes::new(), 
                                dest_address: ethers::types::Bytes::from(dest_addr_bytes.clone()),
                                network_address: network_address_bytes, 
                                duty_window_seconds: duty_window,
                                paradapp_receive_program: info.user_program.clone(),
                                locked_anchor_height: source_anchor.anchor_height,
                            },
                        )
                        .await
                        .map_err(|e| anyhow::anyhow!("Commit failed: {}", e))?;
                } else {
                    let mut final_dest_address = [0u8; 20];
                    let copy_len = std::cmp::min(dest_addr_bytes.len(), 20);
                    final_dest_address[..copy_len].copy_from_slice(&dest_addr_bytes[..copy_len]);

                    // We keep this purely for clean logging
                    let dest_address_evm = ethers::types::Address::from(final_dest_address);
                    
                    let dest_address_bytes_param = ethers::types::Bytes::from(final_dest_address.to_vec());
                    
                    let network_address_bytes = ethers::types::Bytes::from(raw_user_address_32); 

                    info!(
                        %tx_id, 
                        dest = %dest_network, 
                        requested = %info.native_amount, 
                        calculated_payout = %safe_native_amount, 
                        dest_address = %dest_address_evm,
                        network_address = %network_address_bytes,
                        "Opening bridge tunnel with market-adjusted native amount (EVM Dest)"
                    );

                    info!(
                        tx_id = %tx_id,
                        dest_network_id = %network_id_u8,
                        bitcoin_amount = %info.bitcoin_amount,
                        native_amount = %safe_native_amount,
                        dest_address = %dest_address_evm,
                        network_address = %network_address_bytes,
                        duty_window_seconds = %duty_window,
                        receive_program_len = %info.user_program.len(),
                        locked_anchor = %source_anchor.anchor_height,
                        "[TUNNEL] Sending commit_bitcoin_to_native to destination provider"
                    );

                    dest_provider
                        .commit_bitcoin_to_native(
                            paradapp_core::traits::chain_provider_adapter::BitcoinToNativeCommitArgs {
                                bitcoin_amount: info.bitcoin_amount,
                                native_amount: safe_native_amount,
                                network_id: U256::from(network_id_u8),
                                user_program: ethers::types::Bytes::new(),
                                dest_address: dest_address_bytes_param,
                                network_address: network_address_bytes,
                                duty_window_seconds: duty_window,
                                paradapp_receive_program: info.user_program.clone(),
                                locked_anchor_height: source_anchor.anchor_height,
                            },
                        )
                        .await
                        .map_err(|e| anyhow::anyhow!("Commit failed: {}", e))?;
                }

                Ok::<(), anyhow::Error>(())
            }
            .await
            {
                tracing::error!(%tx_id, error = %e, "Failed to process tunneling intent");
                continue; 
            }
        }
        Ok(())
    }
    #[tracing::instrument(
    name = "scheduled_btc_sweep",
    skip(stack),
    fields(network = %stack.network_id())
)]
    async fn tick_sweeping(stack: Arc<dyn ChainStack>) -> anyhow::Result<()> {
        let provider = stack.chain_provider();

        info!("Executing scheduled BTC sweep check...");

        if let Err(e) = provider.trigger_btc_sweep().await {
            error!(error = %e, "Failed to execute BTC sweep logic");
            return Err(e);
        }

        Ok(())
    }
}

// Helper
impl ChainOperator {
    async fn handle_operator_timeouts(
        stack: &Arc<dyn ChainStack>,
        state: &GlobalChainState,
    ) {
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
                .handle_operator_closes_for_active(
                    tx_id,
                    state.confirmations_required,
                )
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
        info!(%network, target_height = %target_height, "Initial sync target height determined");
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

        // Execute all requests in parallel
        let results = futures::future::join_all(anchor_futures).await;

        // Log errors for failed anchor lookups
        for res in results.iter() {
            if let Err((tx_id, r_network, err)) = res {
                warn!(%network, remote = %r_network, %tx_id, error = %err, "Failed to fetch remote anchor info");
            }
        }

        let min_remote_anchor =
            results.into_iter().filter_map(|r| r.ok()).min();

        // If we found remote intents, our sync target is the minimum anchor height required
        if let Some(remote_target) = min_remote_anchor {
            // We only update if the remote requirement is actually different/relevant
            if remote_target != state.safe_anchor {
                target_height = remote_target;
                info!(%network, target_height = %target_height, "Updated sync target height based on remote anchor requirements");
                target_reason = "lowest remote bridge intent anchor";
            }
        }

        let gap = target_height.saturating_sub(state.global_tip);
        if gap == 0 {
            warn!(%network, current = state.global_tip, target = target_height, "No sync gap found; chain is up to date");
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
                .jump_to_anchor_from_zero_active(
                    state.global_tip,
                    target_height,
                )
                .await?;
            info!(%network, "Jump execution completed");
        } else {
            let user_close_candidates = stack
                .approving()
                .discover_user_close_candidates(state.next_tx_id - U256::one())
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

                let candidates: Vec<(U256, &'static str)> =
                    user_close_candidates
                        .into_iter()
                        .map(|(tx_id, kind)| {
                            let static_kind = match kind.as_str() {
                                "timeoutNoDeposit_NativeToBitcoin" => {
                                    "timeoutNoDeposit_NativeToBitcoin"
                                },
                                "closeNoBitcoin_BitcoinToNative" => {
                                    "closeNoBitcoin_BitcoinToNative"
                                },
                                "refundAfterNoProof_NativeTokentoBTC" => {
                                    "refundAfterNoProof_NativeTokentoBTC"
                                },
                                "claimNative_AfterOperatorExpired" => {
                                    "claimNative_AfterOperatorExpired"
                                },
                                _ => "claimNative_AfterOperatorExpired",
                            };
                            (tx_id, static_kind)
                        })
                        .collect();

                stack.approving().execute_user_closes(candidates).await?;
                info!(%network, "User closes executed");

                // REFRESH STATE: Check if we can jump now
                let refreshed = provider.get_global_chain_state().await?;
                if refreshed.active_open == 0 {
                    info!(%network, "All active closed → jumping to target height");
                    provider
                        .jump_to_anchor_from_zero_active(
                            refreshed.global_tip,
                            target_height,
                        )
                        .await?;
                    info!(%network, "Post-close jump execution completed");
                } else {
                    info!(
                        %network,
                        active_open = %refreshed.active_open,
                        global_tip = %refreshed.global_tip,
                        "Jump skipped: active operations still pending"
                    );
                }
            } else {
                info!(%network, "Streaming headers is cheaper or no candidates found");
                stack
                    .streaming()
                    .stream_headers_to_height(
                        state.global_tip,
                        state.safe_anchor,
                        200,
                    )
                    .await?;
                info!(%network, "Header streaming sequence completed");
            }
        }

        Ok(())
    }
    
    async fn check_bridge_intent(
        ctx: Arc<CoreContext>,
        sources: &[String],
        current: SupportedNetwork,
    ) -> Vec<BridgeIntent> {
        let tasks =
            sources.iter().map(|source| {
                let ctx = ctx.clone();
                let source = source.clone();

                async move {
                    if let Ok(remote_stack) =
                        crate::registry::Registry::get_stack(&source, ctx).await
                    {
                        let remote_provider = remote_stack.chain_provider();

                        // 1. Get remote limits
                        let min_limit =
                            U256::from(remote_provider.min_transaction_limit());
                        let max_limit =
                            U256::from(remote_provider.max_transaction_limit());

                        let (approval_res, user_action_res, proof_res) =
                            tokio::join!(
                    remote_provider.get_tx_ids_by_filter(TxIdFilter {
                        type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                        phase_filter:
                            TransactionPhase::WAITING_OPERATOR_APPROVAL,
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
                    }),
                    remote_provider.get_tx_ids_by_filter(TxIdFilter {
                        type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                        phase_filter:
                            TransactionPhase::ACTIVE_WAITING_PROOF,
                        dest_network: Some(current),
                        max_results: U256::from(100u64),
                        ..Default::default()
                    })
                );

                        let mut raw_ids = approval_res.unwrap_or_default();
                        raw_ids.extend(user_action_res.unwrap_or_default());
                        raw_ids.extend(proof_res.unwrap_or_default());

                        if raw_ids.is_empty() {
                            return None;
                        }

                        // 2. Filter IDs by remote conversion info and limits
                        let mut filtered_ids = Vec::new();
                        for id in raw_ids {
                            if let Ok(conv) =
                                remote_provider.get_conversion_info(id).await
                            {
                                if conv.native_amount >= min_limit
                                    && conv.native_amount <= max_limit
                                {
                                    filtered_ids.push(id);
                                } else {
                                    info!(
                                        tx_id = %id,
                                        from_net = %source,
                                        amount = %conv.native_amount,
                                        "Intent ignored: outside limits"
                                    );
                                }
                            }
                        }

                        if !filtered_ids.is_empty() {
                            filtered_ids.sort();
                            filtered_ids.dedup();

                            return Some(BridgeIntent {
                                stack: remote_stack,
                                tx_ids: filtered_ids,
                            });
                        }
                    }
                    None
                }
            });

        let results = join_all(tasks).await;
        results.into_iter().flatten().collect()
    }
}
