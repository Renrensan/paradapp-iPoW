use std::sync::Arc;

use ethers_core::types::U256;
use paradapp_core::{
    consts::{transaction_phase::TransactionPhase, transaction_type::TransactionType},
    traits::approving_adapter::ApprovingAdapter,
};

use anyhow::Result;
use tracing::{info, warn};

pub struct ApprovingOrchestrator {
    adapter: Arc<dyn ApprovingAdapter>,
    network: &'static str,
}

impl ApprovingOrchestrator {
    pub fn new(adapter: Arc<dyn ApprovingAdapter>, network: &'static str) -> Self {
        Self { adapter, network }
    }

    #[tracing::instrument(
        name = "operator_approving",
        skip(self),
        fields(network = %self.network)
    )]
    pub async fn run_once(&self) -> Result<()> {
        // === Fetch pending approvals ===
        let pending_txids = match self.adapter.get_pending_txids(500).await {
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
            tx_ids = ?pending_txids,
            "Found conversions waiting for operator approval"
        );

        // === Check RPC Health ===
        if let Err(e) = self.adapter.check_rpc_health().await {
            warn!(error = %e, "Skipping this cycle — RPC health check failed");
            return Ok(());
        }

        // === Fetch chain state ===
        let mut state = self.adapter.get_global_chain_state().await?;

        info!(
            btc_tip = state.btc_tip,
            global_tip = state.global_tip,
            active_open = state.active_open,
            "Before operator closes"
        );

        // === Operator timeout closes ===
        if state.next_tx_id > U256::one() {
            let from = U256::one();
            let to = state.next_tx_id - U256::one();
            let max = U256::from(500u64);

            let active = self
                .adapter
                .get_tx_ids_by_phase(
                    TransactionPhase::ACTIVE_WAITING_PROOF,
                    TransactionType::ANY,
                    from,
                    to,
                    max,
                )
                .await
                .unwrap_or_default();

            let waiting_user = self
                .adapter
                .get_tx_ids_by_phase(
                    TransactionPhase::WAITING_USER_ACTION,
                    TransactionType::BITCOIN_TO_NATIVE,
                    from,
                    to,
                    max,
                )
                .await
                .unwrap_or_default();

            let mut seen = std::collections::HashSet::new();
            for tx_id in active.into_iter().chain(waiting_user) {
                if seen.insert(tx_id) {
                    let _ = self
                        .adapter
                        .handle_operator_closes_for_active(tx_id, state.confirmations_required)
                        .await;
                }
            }
        }

        // === Refresh state after closes ===
        state = self.adapter.get_global_chain_state().await?;

        info!(
            global_tip = state.global_tip,
            active_open = state.active_open,
            "After operator closes"
        );

        // === Sync decision ===
        let stream_gap = state.safe_anchor.saturating_sub(state.global_tip);

        info!(
            btc_tip = state.btc_tip,
            safe_anchor = state.safe_anchor,
            global_tip = state.global_tip,
            stream_gap = stream_gap,
            active_open = state.active_open,
            "Sync decision point"
        );

        if stream_gap > 0 {
            if state.active_open == 0 {
                info!("No active conversions → jumping to safe anchor");
                self.adapter
                    .jump_to_anchor_from_zero_active(state.global_tip, state.safe_anchor)
                    .await?;
            } else {
                let user_close_candidates = self
                    .adapter
                    .discover_user_close_candidates(
                        state.next_tx_id - U256::one(),
                        state.confirmations_required,
                    )
                    .await
                    .unwrap_or_default();

                let user_close_cost = user_close_candidates.len();

                info!(
                    stream_gap = stream_gap,
                    user_close_cost = user_close_cost,
                    "User-close vs stream decision"
                );

                if user_close_cost > 0 && (stream_gap as usize) > user_close_cost {
                    info!("Cheaper to user-close than stream → executing");

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

                    self.adapter.execute_user_closes(candidates).await?;

                    let refreshed = self.adapter.get_global_chain_state().await?;
                    if refreshed.active_open == 0 {
                        info!("All active closed → jumping to safe anchor");
                        self.adapter
                            .jump_to_anchor_from_zero_active(
                                refreshed.global_tip,
                                refreshed.safe_anchor,
                            )
                            .await?;
                    }
                } else {
                    info!("Streaming headers is cheaper");
                    self.adapter
                        .stream_headers_to_height(state.global_tip, state.safe_anchor, 200)
                        .await?;
                }
            }
        } else {
            info!("Global tip already at or beyond safe anchor");
        }

        // === Approvals ===
        let duty_seconds = 24 * 60 * 60;
        for tx_id in pending_txids {
            if let Err(e) = self.adapter.approve_one_tx(tx_id, duty_seconds).await {
                warn!(
                    tx_id = %tx_id,
                    error = %e,
                    "Failed to approve tx"
                );
            }
        }

        info!("approve / close / sync cycle completed");
        Ok(())
    }
}
