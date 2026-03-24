use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use ethers::types::U256;
use paradapp_core::{
    btc::btc_service::{
        BitcoinMerkleProofPayload, check_confirmation_and_build_proof,
        send_to_user_program, btc_tip_height
    },
    consts::{
        supported_network_enum::SupportedNetwork,
        transaction_phase::TransactionPhase, transaction_type::TransactionType,
    },
    dependencies::context::CoreContext,
    models::conversion::Conversion,
    traits::{
        chain_provider_adapter::{ChainProviderAdapter, TxIdFilter},
        converting_adapter::ConvertingAdapter,
    },
};
use tracing::{error, info, warn};

use crate::{bindings::paradapp_convert, dependencies::context::EvmContext};
use anyhow::Result;

pub struct EvmConvertingAdapter {
    pub ctx: Arc<EvmContext>,
    pub core_ctx: Arc<CoreContext>,
    pub chain_provider: Arc<dyn ChainProviderAdapter>,
}

impl EvmConvertingAdapter {
    fn map_to_core(evm: paradapp_convert::Conversion) -> Conversion {
        Conversion {
            user: evm.user,
            is_native_to_bitcoin: evm.is_native_to_bitcoin,
            slippage: evm.slippage,
            user_program: evm.user_program,
            paradapp_receive_program: evm.paradapp_receive_program,
            network_address: evm.network_address,
            network_id: evm.network_id,
            native_amount: evm.native_amount,
            bitcoin_amount: evm.bitcoin_amount,
            created_at: evm.created_at,
            approved_at: evm.approved_at,
            deposited_at: evm.deposited_at,
            commit_fee: evm.commit_fee,
            approved: evm.approved,
            deposited: evm.deposited,
            completed: evm.completed,
            refunded: evm.refunded,
            reserved_native: evm.reserved_native,
            operator_duty_expires_at: evm.operator_duty_expires_at,
        }
    }
}

#[async_trait]
impl ConvertingAdapter for EvmConvertingAdapter {
    async fn mark_processed(
        &self,
        tx_id: U256,
        btc_tx_id: Option<String>,
    ) -> Result<()> {
        let tx_id_str = tx_id.to_string();
        let btc_id = btc_tx_id.unwrap_or_else(|| "unknown".to_string());
        let network = self.ctx.cfg.network.string_identifier();

        info!(tx_id = %tx_id, btc_tx_id = %btc_id, "Marking transaction as processed in storage");

        if let Err(e) = self
            .core_ctx
            .redis_storage
            .set_conversion_processed(network, &tx_id_str, &btc_id)
            .await
        {
            error!(tx_id = %tx_id, error = %e, "Failed to mark transaction as processed");
            return Err(e);
        }

        info!(tx_id = %tx_id, "Transaction marked successfully");
        Ok(())
    }

    async fn find_native_to_btc_ready(
        &self,
        to_tx_id: U256,
        dest_network: Option<SupportedNetwork>,
    ) -> Result<Vec<(U256, Conversion)>> {
        let mut ready = Vec::new();
        let tx_types = [
            // TransactionType::NATIVE_TO_BITCOIN, //Temporary turn off not to send BTC to user
            TransactionType::NATIVE_TO_NATIVE_OUT,
        ];

        for tx_type in tx_types {
            let active_ids: Vec<U256> = self
                .chain_provider
                .get_tx_ids_by_filter(TxIdFilter {
                    type_filter: tx_type,
                    phase_filter: TransactionPhase::ACTIVE_WAITING_PROOF,
                    dest_network,
                    to_tx_id,
                    ..Default::default()
                })
                .await?;

            if active_ids.is_empty() {
                continue;
            }

            info!(
                tx_type = ?tx_type,
                count = active_ids.len(),
                "Processing active IDs from contract"
            );

            for tx_id in active_ids {
                // 1. Fetch the EVM-specific struct
                let (evm_conv, _) = self
                    .ctx
                    .c_op
                    .get_conversion_with_phase(tx_id)
                    .call()
                    .await?;

                // 2. Perform business logic checks
                if tx_type == TransactionType::NATIVE_TO_BITCOIN
                    && !evm_conv.is_native_to_bitcoin
                {
                    continue;
                }

                if !evm_conv.approved || evm_conv.completed || evm_conv.refunded
                {
                    continue;
                }

                if !evm_conv.deposited {
                    continue;
                }

                let core_conv = Self::map_to_core(evm_conv);
                ready.push((tx_id, core_conv));
            }
        }

        if !ready.is_empty() {
            info!(
                to_tx_id = %to_tx_id,
                count = ready.len(),
                tx_ids = ?ready.iter().map(|r| r.0).collect::<Vec<_>>(),
                "Found ready conversions awaiting payout (mapped to Core)"
            );
        }

        Ok(ready)
    }

    async fn find_btc_to_native_completed(
        &self,
        to_tx_id: U256,
        dest_network: Option<SupportedNetwork>,
    ) -> Result<Vec<(U256, Conversion)>> {
        let mut completed = Vec::new();
        let network = self.ctx.cfg.network.string_identifier();

        for tx_type in [
            TransactionType::BITCOIN_TO_NATIVE,
            TransactionType::NATIVE_TO_NATIVE_IN,
        ] {
            let mut ids: Vec<U256> = self
                .chain_provider
                .get_tx_ids_by_filter(TxIdFilter {
                    type_filter: tx_type,
                    phase_filter: TransactionPhase::COMPLETED,
                    dest_network,
                    to_tx_id,
                    ..Default::default()
                })
                .await?;

            completed.append(&mut ids);
        }

        info!(
            to_tx_id = %to_tx_id,
            count = completed.len(),
            tx_ids = ?completed,
            "Contract returned COMPLETED BTC→Native txs"
        );

        let mut ready = Vec::new();

        // Check for processed in storage
        // Convert completed U256 IDs to strings for Redis
        let completed_id_strs: Vec<String> =
            completed.iter().map(|tx_id| tx_id.to_string()).collect();

        // Perform batch check in Redis
        let processed_flags = self
            .core_ctx
            .redis_storage
            .filter_processed_ids(network, &completed_id_strs)
            .await?;

        for (idx, tx_id) in completed.iter().enumerate() {
            // Check processed flag from Redis
            if processed_flags[idx] {
                continue;
            }

            let evm_conv =
                self.ctx.c_op.get_conversion_with_phase(*tx_id).call().await?.0;

            if evm_conv.is_native_to_bitcoin {
                continue;
            }

            if !evm_conv.completed || evm_conv.refunded {
                continue;
            }

            ready.push((*tx_id, Self::map_to_core(evm_conv)));
        }

        if !ready.is_empty() {
            info!(
                to_tx_id = %to_tx_id,
                count = ready.len(),
                tx_ids = ?ready.iter().map(|r| r.0).collect::<Vec<_>>(),
                "Found BTC→Native conversions completed (user got Native)"
            );
        }

        Ok(ready)
    }

    async fn handle_native_to_btc_conversion(
        &self,
        tx_id: U256,
        conv: Conversion,
    ) -> Result<()> {
        // Convert Bytes -> Vec<u8> for user program
        let user_program: Vec<u8> = if conv.user_program.0.is_empty() {
            vec![]
        } else {
            conv.user_program.0.to_vec()
        };

        // Program status for logging
        let program_status =
            if user_program.is_empty() { "empty" } else { "non-empty" };
        info!(
            amount_sats = %conv.bitcoin_amount,
            program_status, "Sending BTC to user's script program"
        );

        // Safely downcast bitcoin_amount to u64 for sats
        let amount_sats: u64 =
            conv.bitcoin_amount.try_into().map_err(|_| {
                anyhow::anyhow!("btc_amount overflow: {}", conv.bitcoin_amount)
            })?;

        // Fetch anchor info and BTC tip height for validation
        let anchor = self.chain_provider.anchor_info(tx_id).await?;
        let btc_tip = btc_tip_height(&self.core_ctx).await?;
        let btc_tip_u256 = U256::from(btc_tip);
        let limit = anchor.anchor_height + U256::from(20);

        // Only send_to_user_program if btc tip height is less than 20 + anchor height
        if btc_tip_u256 < limit {
            // Attempt to send BTC to user program
            match send_to_user_program(
                &self.core_ctx,
                &user_program,
                amount_sats,
            )
            .await
            {
                Ok(result) => {
                    info!(
                        tx_id = %tx_id,
                        btc_txid = %result,
                        "BTC sent successfully to user program"
                    );
                    self.mark_processed(tx_id, Some(result)).await?;
                },
                Err(e) => {
                    let err_msg = e.to_string();

                    if err_msg.contains("Not enough funds") {
                        warn!(tx_id = %tx_id, "Insufficient BTC. Triggering provider-level emergency sweep.");

                        if let Err(e) =
                            self.chain_provider.trigger_btc_sweep().await
                        {
                            error!(tx_id = %tx_id, error = %e, "Emergency sweep execution failed");
                        } else {
                            info!(tx_id = %tx_id, "Emergency sweep completed successfully.");
                        }
                    }
                    error!(error = %e, tx_id = %tx_id, "Failed to send BTC to user program");
                    return Err(e);
                },
            }
        } else {
            warn!(
                %tx_id,
                btc_tip,
                anchor_height = %anchor.anchor_height,
                "BTC tip height too high relative to anchor; skipping send"
            );
        }

        Ok(())
    }
    
    async fn handle_btc_to_native_conversion(&self, tx_id: U256) -> Result<()> {
        // Log before the action so you know which ID is being targeted
        info!(%tx_id, "Marking BTC→NATIVE conversion as processed in storage.");

        // Immediate mark as processed since no on-chain action is needed
        let status_str = "processed_btc_to_native".to_string();

        self.mark_processed(tx_id, Some(status_str.clone())).await?;

        // Success confirmation log
        info!(%tx_id, status = %status_str, "Successfully marked conversion.");

        Ok(())
    }

    async fn check_confirmation_and_build_proof(
        &self,
        tx_id: U256,
        btc_txid: &str,
    ) -> Result<Option<BitcoinMerkleProofPayload>> {
        check_confirmation_and_build_proof(&self.core_ctx, tx_id, btc_txid)
            .await
    }

    async fn submit_merkle_proof(
        &self,
        tx_id: U256,
        proof: BitcoinMerkleProofPayload,
    ) -> Result<()> {
        let call = self.ctx.c_op.submit_bitcoin_merkle_proof_with_tx(
            tx_id,
            proof.legacy_tx,
            proof.vout_index,
            proof.block_hash_le,
            proof.block_height,
            proof.branch,
            proof.index,
        );

        let pending = call.send().await?;
        let tx_hash = pending.tx_hash();

        info!(
            "Merkle proof submitted txId={} | contract_tx_hash={:?}",
            proof.tx_id, tx_hash
        );

        Ok(())
    }

    async fn get_processed_native_to_btc(
        &self,
        tx_ids: &[U256],
    ) -> Result<HashMap<U256, String>> {
        // Short-circuit: nothing to check
        if tx_ids.is_empty() {
            return Ok(HashMap::new());
        }

        // Convert U256 -> String
        let tx_id_strs: Vec<String> =
            tx_ids.iter().map(|id| id.to_string()).collect();

        let network = self.ctx.cfg.network.string_identifier();

        // Use the redis_storage from core_ctx
        let btc_tx_ids = self
            .core_ctx
            .redis_storage
            .get_btc_tx_ids(network, &tx_id_strs)
            .await?;

        let mut result = HashMap::new();

        for (i, btc_tx_id) in btc_tx_ids.into_iter().enumerate() {
            if let Some(btc_id) = btc_tx_id {
                // Mimicking "btc_tx_id IS NOT NULL"
                if btc_id != "unknown" {
                    result.insert(tx_ids[i], btc_id);
                }
            }
        }

        Ok(result)
    }
}
