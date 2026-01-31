use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers::types::{Address, Bytes, H160, U256};

use paradapp_core::consts::supported_network_enum::SupportedNetwork;
use paradapp_core::consts::transaction_phase::TransactionPhase;
use paradapp_core::consts::transaction_type::TransactionType;
use paradapp_core::traits::approving_adapter::ApprovingAdapter;
use paradapp_core::traits::chain_helper_adapter::{
    BitcoinToNativeCommitArgs, ChainHelperAdapter, TxIdFilter,
};
use paradapp_core::traits::interop_resolver::InteropResolver as InteropResolverTrait;
use paradapp_core::traits::streaming_adapter::StreamingAdapter;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct Conversion {
    pub user: ::ethers::core::types::Address,
    pub is_native_to_bitcoin: bool,
    pub slippage: u16,
    pub user_program: ::ethers::core::types::Bytes,
    pub paradapp_receive_program: ::ethers::core::types::Bytes,
    pub network_address: ::ethers::core::types::Bytes,
    pub network_id: ::ethers::core::types::U256,
    pub native_amount: ::ethers::core::types::U256,
    pub bitcoin_amount: ::ethers::core::types::U256,
    pub created_at: ::ethers::core::types::U256,
    pub approved_at: ::ethers::core::types::U256,
    pub deposited_at: ::ethers::core::types::U256,
    pub commit_fee: ::ethers::core::types::U256,
    pub approved: bool,
    pub deposited: bool,
    pub completed: bool,
    pub refunded: bool,
    pub reserved_native: ::ethers::core::types::U256,
    pub operator_duty_expires_at: ::ethers::core::types::U256,
}
pub struct InteropResolver {
    pub source_helper: Arc<dyn ChainHelperAdapter>,
    pub dest_helper: Arc<dyn ChainHelperAdapter>,

    pub source_approver: Arc<dyn ApprovingAdapter>,
    pub dest_approver: Arc<dyn ApprovingAdapter>,

    pub source_streaming: Arc<dyn StreamingAdapter>,
    pub dest_streaming: Arc<dyn StreamingAdapter>,

    pub dest_network: SupportedNetwork,
}

#[async_trait]
impl InteropResolverTrait for InteropResolver {
    async fn run_once(&self, duty_seconds: u64) -> Result<()> {
        let next_tx_id: U256 = self.source_helper.next_tx_id().await?;
        if next_tx_id <= U256::from(1u64) {
            return Ok(());
        }
        let to_tx_id = next_tx_id - U256::from(1u64);

        // 1. Process WAITING_OPERATOR_APPROVAL (Standard Flow)
        let pending_approval = self
            .get_txs_by_phase(to_tx_id, TransactionPhase::WAITING_OPERATOR_APPROVAL, 500)
            .await?;
        for tx_id in pending_approval {
            if let Err(e) = self
                .attempt_approve_one_tx_and_open_tunnel(tx_id, duty_seconds)
                .await
            {
                warn!(tx_id = %tx_id, error = %e, "Approval flow failed");
            }
        }

        // 2. Process WAITING_USER_ACTION (Tunnel Only Flow)
        let pending_user = self
            .get_txs_by_phase(to_tx_id, TransactionPhase::WAITING_USER_ACTION, 500)
            .await?;
        for tx_id in pending_user {
            if let Err(e) = self.attempt_open_tunnel_only(tx_id).await {
                warn!(tx_id = %tx_id, error = %e, "Tunnel-only flow failed");
            }
        }

        Ok(())
    }

    // Helper to fetch IDs based on phase
    async fn get_txs_by_phase(&self, to_tx_id: U256, phase: u8, max: u64) -> Result<Vec<U256>> {
        self.source_helper
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                phase_filter: phase,
                user_filter: H160::zero(),
                user_program_filter: Bytes::new(),
                dest_network: None,
                from_tx_id: U256::from(1u64),
                to_tx_id,
                max_results: U256::from(max),
            })
            .await
    }

    async fn attempt_open_tunnel_only(&self, tx_id: U256) -> Result<()> {
        let conv = self.source_helper.get_conversion_info(tx_id).await?;

        // CHECK DESTINATION: Does this user_program already exist on the destination chain?
        // We look for NATIVE_TO_NATIVE_IN in WAITING_USER_ACTION phase
        let dest_next_id = self.dest_helper.next_tx_id().await?;
        let existing_on_dest = self
            .dest_helper
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_IN,
                phase_filter: TransactionPhase::WAITING_USER_ACTION,
                user_filter: H160::zero(),
                user_program_filter: conv.user_program.clone(),
                dest_network: None,
                from_tx_id: U256::from(1u64),
                to_tx_id: dest_next_id.saturating_sub(U256::one()),
                max_results: U256::one(),
            })
            .await?;

        if !existing_on_dest.is_empty() {
            info!(tx_id = %tx_id, "TX already exists on destination chain, skipping tunnel open");
            return Ok(());
        }

        // Logic mirrors the destination chain logic in attempt_approve_one_tx_and_open_tunnel
        let min_anchor_dest = self.dest_helper.min_anchor_height().await?;
        let global_tip_src = self.source_helper.global_tip_height().await?;

        if min_anchor_dest <= global_tip_src {
            // Check destination chain stream state (Sync logic)
            let dest_chain_state = self.dest_helper.get_global_chain_state().await?;
            let dest_stream_gap = dest_chain_state
                .safe_anchor
                .saturating_sub(dest_chain_state.global_tip);

            if dest_stream_gap > 0 {
                let active_ids = self
                    .dest_streaming
                    .get_active_tx_ids(1000, Some(self.dest_network))
                    .await?;
                if !active_ids.is_empty() {
                    return Ok(());
                }

                if dest_chain_state.active_open == 0 {
                    self.dest_helper
                        .jump_to_anchor_from_zero_active(
                            dest_chain_state.global_tip,
                            dest_chain_state.safe_anchor,
                        )
                        .await?;
                } else {
                    // Discovery/Stream logic (Skipped for brevity, same as your prototype)
                    self.dest_helper
                        .stream_headers_to_height(
                            dest_chain_state.global_tip,
                            dest_chain_state.safe_anchor,
                            200,
                        )
                        .await?;
                }
            }

            // Execute Open Tunnel
            let anchor = self.source_helper.anchor_info(tx_id).await?;
            let dest_address = Address::from_slice(&conv.network_address.as_ref()[..20]);
            let network_address = Bytes::from(conv.user.as_bytes().to_vec());

            self.dest_helper
                .commit_bitcoin_to_native(BitcoinToNativeCommitArgs {
                    bitcoin_amount: conv.bitcoin_amount,
                    network_id: conv.network_id,
                    user_program: conv.user_program.clone(),
                    dest_address,
                    network_address,
                    duty_window_seconds: conv.operator_duty_expires_at,
                    paradapp_receive_program: Bytes::new(),
                    locked_anchor_height: anchor.anchor_height,
                    slippage: conv.slippage,
                })
                .await?;

            info!(tx_id = %tx_id, "Tunnel opened for WAITING_USER_ACTION tx");
        }

        Ok(())
    }

    async fn attempt_approve_one_tx_and_open_tunnel(
        &self,
        tx_id: U256,
        duty_seconds: u64,
    ) -> Result<()> {
        let source_chain_sync_result: Result<()> = (async {
            let source_chain_state = self.source_helper.get_global_chain_state().await?;
            info!(
                btc_tip = source_chain_state.btc_tip,
                global_tip = source_chain_state.global_tip,
                active_open = source_chain_state.active_open,
                "Source chain current state"
            );

            let source_stream_gap = source_chain_state
                .safe_anchor
                .saturating_sub(source_chain_state.global_tip);

            if source_stream_gap > 0 {
                let active_ids = self
                    .source_streaming
                    .get_active_tx_ids(1000, Some(self.dest_network))
                    .await?;
                if !active_ids.is_empty() {
                    info!(
                        active_count = active_ids.len(),
                        "Active source conversions present → skipping this tick"
                    );
                    return Err(anyhow::anyhow!("active source TXs present, skipping tick"));
                }

                info!("No active source conversions → continue to resolve fresh header");

                if source_chain_state.active_open == 0 {
                    info!("No active conversions in source chain → jumping to safe anchor");
                    self.source_helper
                        .jump_to_anchor_from_zero_active(
                            source_chain_state.global_tip,
                            source_chain_state.safe_anchor,
                        )
                        .await?;
                } else {
                    let user_close_candidates = self
                        .source_approver
                        .discover_user_close_candidates(
                            source_chain_state.next_tx_id - U256::one(),
                            source_chain_state.confirmations_required,
                            Some(self.dest_network),
                        )
                        .await
                        .unwrap_or_default();

                    let user_close_cost = user_close_candidates.len();

                    info!(
                        stream_gap = source_stream_gap,
                        user_close_cost = user_close_cost,
                        "Source chain user-close vs stream decision"
                    );

                    if user_close_cost > 0 && (source_stream_gap as usize) > user_close_cost {
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

                        self.source_approver.execute_user_closes(candidates).await?;

                        let refreshed = self.source_helper.get_global_chain_state().await?;
                        if refreshed.active_open == 0 {
                            info!("All active closed → jumping to safe anchor");
                            self.source_helper
                                .jump_to_anchor_from_zero_active(
                                    refreshed.global_tip,
                                    refreshed.safe_anchor,
                                )
                                .await?;
                        }
                    } else {
                        info!("Streaming headers is cheaper");
                        self.source_helper
                            .stream_headers_to_height(
                                source_chain_state.global_tip,
                                source_chain_state.safe_anchor,
                                200,
                            )
                            .await?;
                    }
                }
            } else {
                info!("Source chain global tip already at or beyond safe anchor");
            }
            Ok(())
        })
        .await;

        if let Err(e) = source_chain_sync_result {
            warn!(
                error = %e,
                "Source chain fresh block / jump logic failed — skipping approval this cycle"
            );
            return Ok(());
        }

        // === Only approve if min_anchor_dest <= global_tip_src ===
        let min_anchor_dest = self.dest_helper.min_anchor_height().await?;
        let global_tip_src = self.source_helper.global_tip_height().await?;

        if min_anchor_dest <= global_tip_src {
            self.source_approver
                .approve_one_tx(tx_id, duty_seconds)
                .await?;
            info!(tx_id = %tx_id, "Source chain TX approved");
        } else {
            warn!(
                %tx_id,
                %min_anchor_dest,
                %global_tip_src,
                "Skipping approval, anchor condition not met"
            );
        }

        // === Destination chain logic (open tunnel if anchor ready) ===
        let min_anchor_dest = self.dest_helper.min_anchor_height().await?;
        let global_tip_src = self.source_helper.global_tip_height().await?;

        if min_anchor_dest <= global_tip_src {
            // Resolve destination state
            let dest_chain_state = self.dest_helper.get_global_chain_state().await?;
            info!(
                btc_tip = dest_chain_state.btc_tip,
                global_tip = dest_chain_state.global_tip,
                active_open = dest_chain_state.active_open,
                "Destination chain state"
            );

            let dest_stream_gap = dest_chain_state
                .safe_anchor
                .saturating_sub(dest_chain_state.global_tip);
            if dest_stream_gap > 0 {
                let active_ids = self
                    .dest_streaming
                    .get_active_tx_ids(1000, Some(self.dest_network))
                    .await?;
                if !active_ids.is_empty() {
                    info!(
                        active_count = active_ids.len(),
                        "Active destination conversions present → skipping this tick"
                    );
                    return Ok(());
                }

                if dest_chain_state.active_open == 0 {
                    self.dest_helper
                        .jump_to_anchor_from_zero_active(
                            dest_chain_state.global_tip,
                            dest_chain_state.safe_anchor,
                        )
                        .await?;
                } else {
                    let user_close_candidates = self
                        .dest_approver
                        .discover_user_close_candidates(
                            dest_chain_state.next_tx_id - U256::one(),
                            dest_chain_state.confirmations_required,
                            Some(self.dest_network),
                        )
                        .await
                        .unwrap_or_default();

                    let user_close_cost = user_close_candidates.len();
                    if user_close_cost > 0 && (dest_stream_gap as usize) > user_close_cost {
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
                        self.dest_approver.execute_user_closes(candidates).await?;
                    } else {
                        self.dest_helper
                            .stream_headers_to_height(
                                dest_chain_state.global_tip,
                                dest_chain_state.safe_anchor,
                                200,
                            )
                            .await?;
                    }
                }
            }

            // Open tunnel
            let conv = self.source_helper.get_conversion_info(tx_id).await?;
            let anchor = self.source_helper.anchor_info(tx_id).await?;

            let dest_address = Address::from_slice(&conv.network_address.as_ref()[..20]);
            let network_address = Bytes::from(conv.user.as_bytes().to_vec());
            let estimated_bitcoin_amount = self
                .dest_helper
                .estimate_bitcoin_from_native(conv.native_amount)
                .await?;

            self.dest_helper
                .commit_bitcoin_to_native(BitcoinToNativeCommitArgs {
                    bitcoin_amount: estimated_bitcoin_amount,
                    network_id: conv.network_id,
                    user_program: conv.user_program.clone(),
                    dest_address,
                    network_address,
                    duty_window_seconds: conv.operator_duty_expires_at,
                    paradapp_receive_program: Bytes::new(),
                    locked_anchor_height: anchor.anchor_height,
                    slippage: conv.slippage,
                })
                .await?;

            info!(tx_id = %tx_id, "Tunnel opened on destination chain");
        } else {
            warn!(
                %tx_id,
                %min_anchor_dest,
                %global_tip_src,
                "Skipping open tunnel, anchor condition not met"
            );
        }

        Ok(())
    }
}
