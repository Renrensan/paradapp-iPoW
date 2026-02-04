use ethers_core::types::U256;
use paradapp_core::{
    btc::btc_service::maybe_rebalance_btc_wallets,
    consts::{
        supported_network_enum::SupportedNetwork, transaction_phase::TransactionPhase,
        transaction_type::TransactionType,
    },
    context::CoreContext,
    traits::{
        chain_provider_adapter::{GlobalChainState, TxIdFilter},
        chain_stack::ChainStack,
    },
};
use std::sync::Arc;
use tokio::{task::JoinHandle, try_join};
use tracing::{error, info, warn};

pub struct LocalOperator;

impl LocalOperator {
    pub async fn run(stack: Arc<dyn ChainStack>, watch_sources: Vec<String>) -> anyhow::Result<()> {
        let network_id = stack.network_id().to_string();
        info!(
            network = %network_id,
            watching = ?watch_sources,
            "Launching Operator with staggered prime-interval tasks"
        );

        let watch_sources = Arc::new(watch_sources);

        // 1. Approving Loop (Interval: 13s)
        // Checks for new transactions requiring operator signatures
        let approving_stack = stack.clone();
        let approving_sources = watch_sources.clone();
        let approving_handle: JoinHandle<()> = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(13));
            // Stagger start by 500ms
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            loop {
                interval.tick().await;
                if let Err(e) =
                    Self::tick_approving(approving_stack.clone(), &approving_sources).await
                {
                    warn!(network = %approving_stack.network_id(), error = %e, "Approving task failed");
                }
            }
        });

        // 2. Converting Loop (Interval: 17s)
        // Handles the logic of swapping/converting assets once approved
        let converting_stack = stack.clone();
        let converting_handle: JoinHandle<()> = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(17));
            // Stagger start by 1.5s
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
            loop {
                interval.tick().await;
                if let Err(e) = Self::tick_converting(converting_stack.clone()).await {
                    warn!(network = %converting_stack.network_id(), error = %e, "Converting task failed");
                }
            }
        });

        // 3. Tunneling Loop (Interval: 23s)
        // Forwards cross-chain intents to the destination chain registry
        let tunneling_stack = stack.clone();
        let tunneling_handle: JoinHandle<()> = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(23));
            // Stagger start by 2.5s
            tokio::time::sleep(std::time::Duration::from_millis(2500)).await;
            loop {
                interval.tick().await;
                if let Err(e) = Self::tick_tunneling(tunneling_stack.clone()).await {
                    warn!(network = %tunneling_stack.network_id(), error = %e, "Tunneling task failed");
                }
            }
        });

        // 4. Streaming Loop (Interval: 31s)
        // Watches for incoming events or state changes on the source chain
        let streaming_stack = stack.clone();
        let streaming_sources = watch_sources.clone();
        let streaming_handle: JoinHandle<()> = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(31));
            // Stagger start by 3.5s
            tokio::time::sleep(std::time::Duration::from_millis(3500)).await;
            loop {
                interval.tick().await;
                if let Err(e) =
                    Self::tick_streaming(streaming_stack.clone(), &streaming_sources).await
                {
                    warn!(network = %streaming_stack.network_id(), error = %e, "Streaming task failed");
                }
            }
        });

        // Monitor all tasks. If any task crashes or a shutdown signal is received, exit.
        tokio::select! {
            res = approving_handle => {
                error!(result = ?res, "Approving task exited unexpectedly");
            }
            res = converting_handle => {
                error!(result = ?res, "Converting task exited unexpectedly");
            }
            res = tunneling_handle => {
                error!(result = ?res, "Tunneling task exited unexpectedly");
            }
            res = streaming_handle => {
                error!(result = ?res, "Streaming task exited unexpectedly");
            }
            _ = tokio::signal::ctrl_c() => {
                info!("Shutdown signal (Ctrl+C) received, stopping operator...");
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

        if (n2b.len() + b2n.len() + n2n_out.len()) > 0 {
            info!(
                n2b = n2b.len(),
                b2n = b2n.len(),
                n2n = n2n_out.len(),
                "Processing approvals"
            );
        }

        if (provider.check_rpc_health().await).is_err() {
            warn!("RPC unhealthy, skipping approval tick");
            return Ok(());
        }

        let state = provider.get_global_chain_state().await?;
        Self::handle_operator_timeouts(&stack, &state).await;

        let bridge_intent_active =
            Self::check_bridge_intent(core_ctx.clone(), watch_sources, current_network).await;
        if bridge_intent_active {
            info!("Active bridge intent detected on remote source");
        }

        Self::handle_sync_logic(stack.clone(), state, bridge_intent_active).await?;

        for tx_id in n2b.into_iter().chain(b2n) {
            let _ = stack.approving().approve_one_tx(tx_id, duty_seconds).await;
        }

        for tx_id in n2n_out {
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

            if let Ok(dest_stack) =
                crate::registry::Registry::get_stack(dest_network.as_str(), core_ctx.clone()).await
            {
                let src_tip = provider.global_tip_height().await?.as_u64();
                Self::sync_remote_chain(dest_stack.clone(), src_tip).await?;

                let min_anchor_dest = dest_stack.chain_provider().min_anchor_height().await?;
                if min_anchor_dest <= provider.global_tip_height().await? {
                    let _ = stack.approving().approve_one_tx(tx_id, duty_seconds).await;
                } else {
                    warn!(%tx_id, %dest_network, "Anchor height not yet sufficient for cross-chain approval");
                }
            }
        }
        Ok(())
    }

    #[tracing::instrument(name = "operator_streaming", skip(stack), fields(network = %stack.network_id()))]
    async fn tick_streaming(
        stack: Arc<dyn ChainStack>,
        watch_sources: &[String],
    ) -> anyhow::Result<()> {
        let provider = stack.chain_provider();
        let next_tx_id = provider.next_tx_id().await?;
        let to_tx_id = next_tx_id.saturating_sub(U256::one());
        let current_network = provider.network();

        let mut active_ids = provider
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

        active_ids.extend(user_action_ids);
        active_ids.sort();
        active_ids.dedup();

        let mut max_target: u64 = 0;
        let core_ctx = stack.core_context();

        for source_name in watch_sources {
            if let Ok(remote_stack) =
                crate::registry::Registry::get_stack(source_name, core_ctx.clone()).await
            {
                let intents = remote_stack
                    .chain_provider()
                    .get_tx_ids_by_filter(TxIdFilter {
                        type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                        phase_filter: TransactionPhase::WAITING_USER_ACTION,
                        dest_network: Some(current_network),
                        max_results: U256::from(100u64),
                        ..Default::default()
                    })
                    .await
                    .unwrap_or_default();

                for tx_id in intents {
                    if let Ok(target) = remote_stack.streaming().compute_stream_target(tx_id).await
                        && target.needed
                        && target.target_height > max_target
                    {
                        max_target = target.target_height;
                        info!(from = %source_name, target = max_target, "Remote intent requires header sync");
                    }
                }
            }
        }

        let mut needed_tx_ids = Vec::new();
        for tx_id in active_ids {
            let stream_target = stack.streaming().compute_stream_target(tx_id).await?;
            if !stream_target.needed {
                continue;
            }
            needed_tx_ids.push(tx_id);
            if stream_target.target_height > max_target {
                max_target = stream_target.target_height;
            }
        }

        if max_target > 0 {
            info!(
                target = max_target,
                tx_count = needed_tx_ids.len(),
                "Streaming headers to global state"
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
                    None => { /* Not confirmed yet, retry later */ }
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
        let intents = provider
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                phase_filter: TransactionPhase::WAITING_USER_ACTION,
                to_tx_id,
                ..Default::default()
            })
            .await?;

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
                let dest_provider = dest_stack.chain_provider();
                let already_opened = dest_provider
                    .get_tx_ids_by_filter(TxIdFilter {
                        type_filter: TransactionType::NATIVE_TO_NATIVE_IN,
                        phase_filter: TransactionPhase::WAITING_USER_ACTION,
                        bitcoin_program_filter: Some(info.user_program.clone()),
                        to_tx_id: dest_provider
                            .next_tx_id()
                            .await?
                            .saturating_sub(U256::one()),
                        max_results: U256::one(),
                        ..Default::default()
                    })
                    .await
                    .map(|v| !v.is_empty())
                    .unwrap_or(false);

                if already_opened {
                    continue;
                }

                // 2. Sync destination to source tip
                let src_tip = provider.global_tip_height().await?.as_u64();
                Self::sync_remote_chain(dest_stack.clone(), src_tip).await?;

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
impl LocalOperator {
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
        bridge_intent_active: bool,
    ) -> anyhow::Result<()> {
        let provider = stack.chain_provider();
        let gap = state.safe_anchor.saturating_sub(state.global_tip);
        if gap == 0 {
            return Ok(());
        }

        if state.active_open == 0 && !bridge_intent_active {
            provider
                .jump_to_anchor_from_zero_active(state.global_tip, state.safe_anchor)
                .await?;
        } else {
            let candidates = stack
                .approving()
                .discover_user_close_candidates(
                    state.next_tx_id - U256::one(),
                    state.confirmations_required,
                )
                .await
                .unwrap_or_default();
            if !candidates.is_empty() && (gap as usize) > candidates.len() {
                let mapped = candidates
                    .into_iter()
                    .map(|(id, kind)| {
                        (
                            id,
                            if kind.contains("refund") {
                                "refundAfterNoProof_NativeTokentoBTC"
                            } else {
                                "claimNative_AfterOperatorExpired"
                            },
                        )
                    })
                    .collect();
                stack.approving().execute_user_closes(mapped).await?;
            } else {
                stack
                    .streaming()
                    .stream_headers_to_height(state.global_tip, state.safe_anchor, 200)
                    .await?;
            }
        }
        Ok(())
    }

    async fn sync_remote_chain(stack: Arc<dyn ChainStack>, target: u64) -> anyhow::Result<()> {
        let provider = stack.chain_provider();
        let state = provider.get_global_chain_state().await?;
        let gap = target.saturating_sub(state.global_tip);
        if gap == 0 {
            return Ok(());
        }

        if state.active_open == 0 {
            provider
                .jump_to_anchor_from_zero_active(state.global_tip, target)
                .await?;
        } else {
            stack
                .streaming()
                .stream_headers_to_height(state.global_tip, target, 200)
                .await?;
        }
        Ok(())
    }

    async fn check_bridge_intent(
        ctx: Arc<CoreContext>,
        sources: &[String],
        current: SupportedNetwork,
    ) -> bool {
        for source in sources {
            if let Ok(remote) = crate::registry::Registry::get_stack(source, ctx.clone()).await {
                let pending = remote
                    .chain_provider()
                    .get_tx_ids_by_filter(TxIdFilter {
                        type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                        phase_filter: TransactionPhase::WAITING_USER_ACTION,
                        dest_network: Some(current),
                        max_results: U256::one(),
                        ..Default::default()
                    })
                    .await
                    .unwrap_or_default();
                if !pending.is_empty() {
                    return true;
                }
            }
        }
        false
    }
}
