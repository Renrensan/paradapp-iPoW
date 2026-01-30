use ethers_core::types::U256;
use paradapp_chain_evm::stack::EvmStack;
use paradapp_core::{
    btc::btc_service::maybe_rebalance_btc_wallets,
    consts::{transaction_phase::TransactionPhase, transaction_type::TransactionType},
    traits::{
        approving_adapter::ApprovingAdapter,
        chain_helper_adapter::{ChainHelperAdapter, GlobalChainState},
        converting_adapter::ConvertingAdapter,
        streaming_adapter::StreamingAdapter,
    },
};
use std::sync::Arc;
use tracing::{error, info, warn};

pub struct LocalOperator;

impl LocalOperator {
    pub async fn run(stack: Arc<EvmStack>) -> anyhow::Result<()> {
        let network_id = stack.network_id.clone();
        info!(network = %network_id, "Launching Local Operator tasks in parallel...");

        // Approving Loop
        let approving_stack = stack.clone();
        let approving_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
            loop {
                interval.tick().await;
                if let Err(e) = Self::tick_approving(approving_stack.clone()).await {
                    warn!(network = %approving_stack.network_id, error = %e, "Approving task failed");
                }
            }
        });

        // Streaming Loop
        let streaming_stack = stack.clone();
        let streaming_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = Self::tick_streaming(streaming_stack.clone()).await {
                    warn!(network = %streaming_stack.network_id, error = %e, "Streaming task failed");
                }
            }
        });

        // Converting Loop
        let converting_stack = stack.clone();
        let converting_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));
            loop {
                interval.tick().await;
                if let Err(e) = Self::tick_converting(converting_stack.clone()).await {
                    warn!(network = %converting_stack.network_id, error = %e, "Converting task failed");
                }
            }
        });

        // Use select to wait for either a shutdown signal or a fatal crash in one of the tasks
        tokio::select! {
            _ = streaming_handle => error!("Streaming task exited unexpectedly"),
            _ = approving_handle => error!("Approving task exited unexpectedly"),
            _ = converting_handle => error!("Converting task exited unexpectedly"),
            _ = tokio::signal::ctrl_c() => {
                info!(network = %network_id, "Shutdown signal received, stopping all tasks");
            }
        }

        Ok(())
    }

    #[tracing::instrument(
        name = "operator_approving",
        skip(stack),
        fields(network = %stack.network_id)
    )]
    async fn tick_approving(stack: Arc<EvmStack>) -> anyhow::Result<()> {
        let duty_seconds = 24 * 60 * 60;

        // 1. Fetch pending approvals
        let pending_txids = match stack.approving.get_pending_txids(500, None).await {
            Ok(txids) if !txids.is_empty() => txids,
            Ok(_) => {
                info!("No pending conversions to approve this cycle.");
                return Ok(());
            }
            Err(e) => {
                warn!("Failed to fetch pending txids: {e}");
                return Ok(());
            }
        };

        info!(
            count = pending_txids.len(),
            "Found conversions waiting for operator approval"
        );

        // 2. Check RPC Health
        if let Err(e) = stack.helper.check_rpc_health().await {
            warn!(error = %e, "Skipping cycle — RPC health check failed");
            return Ok(());
        }

        // 3. Operator timeout closes
        let mut state = stack.helper.get_global_chain_state().await?;
        if state.next_tx_id > U256::one() {
            let to_tx_id = state.next_tx_id - U256::one();

            let active = stack
                .approving
                .get_tx_ids_by_phase(
                    TransactionPhase::ACTIVE_WAITING_PROOF,
                    TransactionType::ANY,
                    U256::one(),
                    to_tx_id,
                    U256::from(500u64),
                    None,
                )
                .await
                .unwrap_or_default();

            let waiting_user = stack
                .approving
                .get_tx_ids_by_phase(
                    TransactionPhase::WAITING_USER_ACTION,
                    TransactionType::BITCOIN_TO_NATIVE,
                    U256::one(),
                    to_tx_id,
                    U256::from(500u64),
                    None,
                )
                .await
                .unwrap_or_default();

            let mut seen = std::collections::HashSet::new();
            for tx_id in active.into_iter().chain(waiting_user) {
                if seen.insert(tx_id) {
                    let _ = stack
                        .approving
                        .handle_operator_closes_for_active(tx_id, state.confirmations_required)
                        .await;
                }
            }
        }

        // 4. Refresh and Sync
        /// Extracted sync decision logic
        async fn handle_sync_logic(
            stack: Arc<EvmStack>,
            state: GlobalChainState,
        ) -> anyhow::Result<()> {
            let stream_gap = state.safe_anchor.saturating_sub(state.global_tip);

            if stream_gap == 0 {
                info!("Global tip already at or beyond safe anchor");
                return Ok(());
            }

            if state.active_open == 0 {
                info!("No active conversions → jumping to safe anchor");
                stack
                    .helper
                    .jump_to_anchor_from_zero_active(state.global_tip, state.safe_anchor)
                    .await?;
            } else {
                let candidates = stack
                    .approving
                    .discover_user_close_candidates(
                        state.next_tx_id - U256::one(),
                        state.confirmations_required,
                        None,
                    )
                    .await
                    .unwrap_or_default();

                let user_close_cost = candidates.len();

                if user_close_cost > 0 && (stream_gap as usize) > user_close_cost {
                    info!(cost = user_close_cost, "Cheaper to user-close than stream");

                    let mapped_candidates: Vec<(U256, &'static str)> = candidates
                        .into_iter()
                        .map(|(id, kind)| {
                            if kind.contains("refundAfterNoProof") {
                                (id, "refundAfterNoProof_NativeTokentoBTC")
                            } else {
                                (id, "claimNative_AfterOperatorExpired")
                            }
                        })
                        .collect();

                    stack
                        .approving
                        .execute_user_closes(mapped_candidates)
                        .await?;

                    // Check if we can jump now
                    let refreshed = stack.helper.get_global_chain_state().await?;
                    if refreshed.active_open == 0 {
                        stack
                            .helper
                            .jump_to_anchor_from_zero_active(
                                refreshed.global_tip,
                                refreshed.safe_anchor,
                            )
                            .await?;
                    }
                } else {
                    info!(
                        gap = stream_gap,
                        "Streaming headers is cheaper or only option"
                    );
                    stack
                        .helper
                        .stream_headers_to_height(state.global_tip, state.safe_anchor, 200)
                        .await?;
                }
            }
            Ok(())
        }

        state = stack.helper.get_global_chain_state().await?;
        if let Err(e) = handle_sync_logic(stack.clone(), state).await {
            warn!(error = %e, "Sync / jump logic failed — skipping approvals");
            return Ok(());
        }

        // 5. Final Approvals
        for tx_id in pending_txids {
            if let Err(e) = stack.approving.approve_one_tx(tx_id, duty_seconds).await {
                warn!(tx_id = %tx_id, error = %e, "Failed to approve tx");
            }
        }

        info!("Approve / close / sync cycle completed");
        Ok(())
    }

    #[tracing::instrument(
        name = "operator_streaming",
        skip(stack),
        fields(network = %stack.network_id)
    )]
    async fn tick_streaming(stack: Arc<EvmStack>) -> anyhow::Result<()> {
        // 1. Get all currently active transaction IDs on this chain
        let active_ids = stack.streaming.get_active_tx_ids(1000, None).await?;
        if active_ids.is_empty() {
            info!("No active conversions found – nothing to stream this pass.");
            return Ok(());
        }

        info!("Found active conversions: {:?}", active_ids);

        let mut needed_tx_ids = Vec::new();
        let mut max_target: u64 = 0;

        // 2. Determine which transactions actually need header updates
        for tx_id in active_ids {
            let stream_target = stack.streaming.compute_stream_target(tx_id).await?;

            if !stream_target.needed {
                info!(tx_id = %tx_id, reason = %stream_target.reason, "txId does not need streaming.");
                continue;
            }

            needed_tx_ids.push(tx_id);

            // Track the furthest block height we need to reach to satisfy all txs
            if stream_target.target_height > max_target {
                max_target = stream_target.target_height;
            }
        }

        if needed_tx_ids.is_empty() {
            info!("No conversions require additional headers this pass.");
            return Ok(());
        }

        // 3. Format IDs for logging (can be simplified if using tracing natively)
        let ids_str = needed_tx_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        info!(
            needed_ids = %ids_str,
            max_target,
            "Need to stream headers up to target height"
        );

        // 4. Trigger the block header relay logic via the adapter
        stack
            .streaming
            .push_headers_global(max_target, needed_tx_ids)
            .await?;

        info!("Done streaming headers for this pass.");
        Ok(())
    }

    #[tracing::instrument(
        name = "operator_converting",
        skip(stack),
        fields(network = %stack.network_id)
    )]
    async fn tick_converting(stack: Arc<EvmStack>) -> anyhow::Result<()> {
        // 1. Determine latest tx id
        let next_tx_id = stack.converting.next_tx_id().await?;
        let to_tx_id = next_tx_id.saturating_sub(U256::one());

        if to_tx_id == U256::zero() {
            info!("No conversions exist yet.");
            return Ok(());
        }

        // 2. Get conversions needing processing
        let ready_h2b = stack
            .converting
            .find_native_to_btc_ready(to_tx_id, None)
            .await?;
        let ready_b2h = stack
            .converting
            .find_btc_to_native_completed(to_tx_id, None)
            .await?;

        if ready_h2b.is_empty() && ready_b2h.is_empty() {
            info!("No conversions requiring off-chain work this pass.");
        }

        // 3. Handle NATIVE → BTC
        let h2b_tx_ids: Vec<U256> = ready_h2b.iter().map(|(id, _)| *id).collect();
        let processed_map = stack
            .converting
            .get_processed_native_to_btc(&h2b_tx_ids)
            .await?;

        for (tx_id, conv) in ready_h2b {
            // Case A: BTC already sent → check confirmation & submit proof
            if let Some(btc_txid) = processed_map.get(&tx_id) {
                match stack
                    .converting
                    .check_confirmation_and_build_proof(tx_id, btc_txid)
                    .await?
                {
                    Some(proof) => {
                        if let Err(err) = stack.converting.submit_merkle_proof(tx_id, proof).await {
                            warn!(%tx_id, ?err, "Error submitting merkle proof");
                        }
                    }
                    None => { /* Not confirmed yet, retry later */ }
                }
                continue;
            }

            // Case B: Not processed yet → send BTC
            if let Err(err) = stack
                .converting
                .handle_native_to_btc_conversion(tx_id, conv)
                .await
            {
                warn!(%tx_id, ?err, "Error handling NATIVE→BTC conversion");
            }
        }

        // 4. Handle BTC → NATIVE
        for (tx_id, conv) in ready_b2h {
            if let Err(err) = stack
                .converting
                .handle_btc_to_native_conversion(tx_id, conv)
                .await
            {
                warn!(%tx_id, ?err, "Error handling BTC→NATIVE conversion");
            }
        }

        // 5. Liquidity Management
        let native_liq = stack.converting.read_liquidity().await?;
        stack
            .converting
            .maybe_rebalance_contract_liquidity(native_liq)
            .await?;

        // 6. BTC hot wallet rebalance (uses core_ctx from stack)
        // Note: Assuming maybe_rebalance_btc_wallets is a standalone utility
        // that takes &CoreContext
        maybe_rebalance_btc_wallets(&stack.converting.core_ctx).await?;

        info!("Done conversion pass.");
        Ok(())
    }
}
