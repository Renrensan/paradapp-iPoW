use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use ethers::types::{Address, Bytes, U256};
use paradapp_core::consts::supported_network_enum::SupportedNetwork;
use paradapp_core::consts::transaction_phase::TransactionPhase;
use paradapp_core::consts::transaction_type::TransactionType;
use paradapp_core::traits::chain_provider_adapter::{
    BitcoinProgramType, BitcoinToNativeCommitArgs, TxIdFilter,
};
use paradapp_core::traits::chain_stack::ChainStack;
use paradapp_core::traits::interop_resolver::InteropResolver as InteropResolverTrait;
use tracing::{info, warn};

pub struct InteropResolver {
    pub source: Arc<dyn ChainStack>,
    pub dest: Arc<dyn ChainStack>,
    pub dest_network: SupportedNetwork,
}

impl InteropResolver {
    pub fn new(
        source: Arc<dyn ChainStack>,
        dest: Arc<dyn ChainStack>,
        dest_network: SupportedNetwork,
    ) -> Self {
        Self {
            source,
            dest,
            dest_network,
        }
    }
}

#[async_trait]
impl InteropResolverTrait for InteropResolver {
    async fn run_once(&self, duty_seconds: u64) -> Result<()> {
        let next_tx_id: U256 = self.source.chain_provider().next_tx_id().await?;
        if next_tx_id <= U256::from(1u64) {
            return Ok(());
        }
        let to_tx_id = next_tx_id - U256::from(1u64);

        // 1. Process WAITING_OPERATOR_APPROVAL (Standard Flow)
        let pending_approval = self
            .source
            .chain_provider()
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                phase_filter: TransactionPhase::WAITING_OPERATOR_APPROVAL,
                to_tx_id,
                ..Default::default()
            })
            .await?;
        for tx_id in pending_approval {
            if let Err(e) = self.attempt_approve_one_tx(tx_id, duty_seconds).await {
                warn!(tx_id = %tx_id, error = %e, "Approval flow failed");
            }
        }

        // 2. Process WAITING_USER_ACTION (Tunnel Only Flow)
        let pending_user = self
            .source
            .chain_provider()
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                phase_filter: TransactionPhase::WAITING_USER_ACTION,
                to_tx_id,
                ..Default::default()
            })
            .await?;
        for tx_id in pending_user {
            if let Err(e) = self.attempt_open_tunnel(tx_id).await {
                warn!(tx_id = %tx_id, error = %e, "Tunnel-only flow failed");
            }
        }

        Ok(())
    }

    async fn attempt_open_tunnel(&self, tx_id: U256) -> Result<()> {
        let conv = self
            .source
            .chain_provider()
            .get_conversion_info(tx_id)
            .await?;

        // CHECK DESTINATION: Does this paradapp program already exist on the destination chain?
        let user_program_filter = conv.user_program.clone();
        let dest_next_id = self.dest.chain_provider().next_tx_id().await?;
        let existing_on_dest = self
            .dest
            .chain_provider()
            .get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_IN,
                phase_filter: TransactionPhase::WAITING_USER_ACTION,
                bitcoin_program_filter: Some(user_program_filter),
                bitcoin_program_type: Some(BitcoinProgramType::Paradapp),
                to_tx_id: dest_next_id.saturating_sub(U256::one()),
                max_results: U256::one(),
                ..Default::default()
            })
            .await?;

        if !existing_on_dest.is_empty() {
            info!(tx_id = %tx_id, "TX already exists on destination chain, skipping tunnel open");
            return Ok(());
        }

        let source_anchor = self.source.chain_provider().anchor_info(tx_id).await?;
        let source_anchor_height_u64: u64 = source_anchor
            .anchor_height
            .try_into()
            .map_err(|_| anyhow::anyhow!("source anchor height conversion error"))?;
        let min_anchor_dest = self.dest.chain_provider().min_anchor_height().await?;

        if min_anchor_dest <= source_anchor.anchor_height {
            // Check destination chain stream state (Sync logic)
            let dest_chain_state = self.dest.chain_provider().get_global_chain_state().await?;
            let dest_stream_gap =
                source_anchor_height_u64.saturating_sub(dest_chain_state.global_tip);

            if dest_stream_gap > 0 {
                if dest_chain_state.active_open == 0 {
                    info!(source_tx_id = %tx_id, "No active conversions → jumping to anchor");
                    self.dest
                        .chain_provider()
                        .jump_to_anchor_from_zero_active(
                            dest_chain_state.global_tip,
                            source_anchor_height_u64,
                        )
                        .await?;
                } 
                else {
                    // 1. Identify if we can clear the blockers (User-Close Candidates)
                    let candidates = self
                        .dest
                        .approving()
                        .discover_user_close_candidates(
                            dest_chain_state.next_tx_id - U256::one(),
                            dest_chain_state.confirmations_required,
                        )
                        .await
                        .unwrap_or_default();

                    let user_close_cost = candidates.len();

                    // 2. Optimization check: Is clearing blockers cheaper than streaming the gap?
                    if user_close_cost > 0 && (dest_stream_gap as usize) > user_close_cost {
                        info!(
                            cost = user_close_cost,
                            "Cheaper to user-close blockers than stream headers"
                        );

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

                        // 3. Clear the blockers
                        self.dest
                            .approving()
                            .execute_user_closes(mapped_candidates)
                            .await?;

                        // 4. Check if clearing them actually brought active_open to zero
                        let refreshed = self.dest.chain_provider().get_global_chain_state().await?;
                        if refreshed.active_open == 0 {
                            info!("Blockers cleared! Executing jump now.");
                            self.dest
                                .chain_provider()
                                .jump_to_anchor_from_zero_active(
                                    refreshed.global_tip,
                                    source_anchor_height_u64,
                                )
                                .await?;
                        }
                    } else {
                        // 5. Fallback: If blockers are too many, or cost is too high, just stream.
                        info!(
                            gap = dest_stream_gap,
                            "Streaming headers is cheaper or only option (active_open > 0)"
                        );
                        self.dest
                            .streaming()
                            .stream_headers_to_height(
                                dest_chain_state.global_tip,
                                source_anchor_height_u64,
                                200,
                            )
                            .await?;
                    }
                }
            }

            // Execute Open Tunnel
            let anchor = self.source.chain_provider().anchor_info(tx_id).await?;
            let dest_address = Address::from_slice(&conv.network_address.as_ref()[..20]);
            let network_address = Bytes::from(conv.user.as_bytes().to_vec());
            let native_amount = conv.native_amount;
            let estimated_bitcoin_amount = self
                .source
                .chain_provider()
                .estimate_bitcoin_from_native(native_amount)
                .await?;
            let network_id = U256::from(self.source.chain_provider().network() as u8);

            // Recall dest to re log
            let dest_chain_state = self.dest.chain_provider().get_global_chain_state().await?;
            info!(
                tx_id = %tx_id,
                bitcoin_amount = ?estimated_bitcoin_amount,
                native_amount = ?native_amount,
                network_id = %network_id,
                dest_address = ?dest_address,
                locked_anchor_height = %anchor.anchor_height,
                dest_global_tip = dest_chain_state.global_tip,
                dest_active_open = dest_chain_state.active_open,
                "calling commit_bitcoin_to_native on dest"
            );
            self.dest
                .chain_provider()
                .commit_bitcoin_to_native(BitcoinToNativeCommitArgs {
                    bitcoin_amount: estimated_bitcoin_amount,
                    network_id,
                    user_program: Bytes::new(),
                    dest_address,
                    network_address,
                    duty_window_seconds: conv.operator_duty_expires_at,
                    paradapp_receive_program: conv.user_program.clone(),
                    locked_anchor_height: anchor.anchor_height,
                    slippage: conv.slippage,
                })
                .await?;

            info!(source_tx_id = %tx_id, "Tunnel opened for WAITING_USER_ACTION tx");
        }

        Ok(())
    }

    async fn attempt_approve_one_tx(&self, tx_id: U256, duty_seconds: u64) -> Result<()> {
        let source_chain_sync_result: Result<()> = (async {
            let source_chain_state = self.source.chain_provider().get_global_chain_state().await?;
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
                let next_tx_id = self.source.chain_provider().next_tx_id().await?;
                let to_tx_id = next_tx_id.saturating_sub(U256::one());

                let active_ids = self
                    .source
                    .chain_provider()
                    .get_tx_ids_by_filter(TxIdFilter {
                        type_filter: TransactionType::ANY,
                        phase_filter: TransactionPhase::ACTIVE_WAITING_PROOF,
                        dest_network: Some(self.dest_network),
                        to_tx_id,
                        max_results: U256::from(1000u64),
                        ..Default::default()
                    })
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
                    self.source.chain_provider()
                        .jump_to_anchor_from_zero_active(
                            source_chain_state.global_tip,
                            source_chain_state.safe_anchor,
                        )
                        .await?;
                } else {
                    let user_close_candidates = self
                        .source.approving()
                        .discover_user_close_candidates(
                            source_chain_state.next_tx_id - U256::one(),
                            source_chain_state.confirmations_required,
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
                        info!(source_tx_id = %tx_id,"Cheaper to user-close than stream → executing on source chain");

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

                        self.source.approving().execute_user_closes(candidates).await?;

                        let refreshed = self.source.chain_provider().get_global_chain_state().await?;
                        if refreshed.active_open == 0 {
                            info!(source_tx_id = %tx_id,"All active closed → jumping to safe anchor on source chain");
                            self.source.chain_provider()
                                .jump_to_anchor_from_zero_active(
                                    refreshed.global_tip,
                                    refreshed.safe_anchor,
                                )
                                .await?;
                        }
                    } else {
                        info!(source_tx_id = %tx_id,"Streaming headers is cheaper on source chain");
                        self.source.streaming()
                            .stream_headers_to_height(
                                source_chain_state.global_tip,
                                source_chain_state.safe_anchor,
                                200,
                            )
                            .await?;
                    }
                }
            }else {
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
        let min_anchor_dest = self.dest.chain_provider().min_anchor_height().await?;
        let global_tip_src = self.source.chain_provider().global_tip_height().await?;

        if min_anchor_dest <= global_tip_src {
            self.source
                .approving()
                .approve_one_tx(tx_id, duty_seconds)
                .await?;
        } else {
            warn!(
                %tx_id,
                %min_anchor_dest,
                %global_tip_src,
                "Skipping approval, anchor condition not met"
            );
        }

        Ok(())
    }
}
