use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use ethers::types::U256;
use paradapp_core::{
    btc::btc_service::{
        BitcoinMerkleProofPayload, check_confirmation_and_build_proof,
        derive_address_from_mnemonic, send_all_btc_from_account_to_dev, send_to_user_program,
    },
    consts::{
        supported_network_enum::SupportedNetwork, transaction_phase::TransactionPhase,
        transaction_type::TransactionType,
    },
    context::CoreContext,
    models::conversion::Conversion,
    traits::{
        chain_provider_adapter::{ChainProviderAdapter, TxIdFilter},
        converting_adapter::ConvertingAdapter,
    },
};
use sqlx::{Row, SqlitePool};
use tracing::{error, info, warn};

use crate::{bindings::paradapp_convert, dependencies::context::EvmContext};
use anyhow::Result;

pub struct EvmConvertingAdapter {
    pub ctx: Arc<EvmContext>,
    pub core_ctx: Arc<CoreContext>,
    pub sqlite_storage: SqlitePool,
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
    async fn mark_processed(&self, tx_id: U256, btc_tx_id: Option<String>) -> Result<()> {
        let id_i64: i64 = tx_id.to_string().parse().unwrap();

        info!(tx_id = %tx_id, btc_tx_id = ?btc_tx_id, "Marking transaction as processed");

        let query = r#"
            INSERT INTO processed_conversions (tx_id, processed, btc_tx_id)
            VALUES (?1, 1, ?2)
            ON CONFLICT(tx_id) DO UPDATE SET processed=1, btc_tx_id=excluded.btc_tx_id
        "#;

        if let Err(e) = sqlx::query(query)
            .bind(id_i64)
            .bind(btc_tx_id.clone())
            .execute(&self.sqlite_storage)
            .await
        {
            error!(tx_id = %tx_id, btc_tx_id = ?btc_tx_id, error = %e, "Failed to mark transaction as processed");
            return Err(e.into());
        }

        info!(tx_id = %tx_id, btc_tx_id = ?btc_tx_id, "Transaction marked as processed successfully");
        Ok(())
    }

    async fn find_native_to_btc_ready(
        &self,
        to_tx_id: U256,
        dest_network: Option<SupportedNetwork>,
    ) -> Result<Vec<(U256, Conversion)>> {
        let mut ready = Vec::new();
        let tx_types = [
            TransactionType::NATIVE_TO_BITCOIN,
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
                if tx_type == TransactionType::NATIVE_TO_BITCOIN && !evm_conv.is_native_to_bitcoin {
                    continue;
                }

                if !evm_conv.approved || evm_conv.completed || evm_conv.refunded {
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

        // Check for processed in sqlite
        // Convert completed U256 IDs to i64
        let completed_ids: Vec<i64> = completed
            .iter()
            .map(|tx_id| tx_id.low_u64() as i64)
            .collect();

        let placeholders = (0..completed_ids.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            "SELECT tx_id FROM processed_conversions WHERE tx_id IN ({}) AND processed = 1",
            placeholders
        );
        let mut query = sqlx::query(&sql);
        for id in &completed_ids {
            query = query.bind(id);
        }

        let processed_ids: Vec<i64> = query
            .fetch_all(&self.sqlite_storage)
            .await?
            .into_iter()
            .map(|row| row.get::<i64, _>("tx_id"))
            .collect();

        let processed_set: std::collections::HashSet<i64> = processed_ids.into_iter().collect();

        for (idx, tx_id) in completed.iter().enumerate() {
            if processed_set.contains(&completed_ids[idx]) {
                continue;
            }

            let evm_conv = self
                .ctx
                .c_op
                .get_conversion_with_phase(*tx_id)
                .call()
                .await?
                .0;

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

    async fn handle_native_to_btc_conversion(&self, tx_id: U256, conv: Conversion) -> Result<()> {
        // Convert Bytes -> Vec<u8> for user program
        let user_program: Vec<u8> = if conv.user_program.0.is_empty() {
            vec![]
        } else {
            conv.user_program.0.to_vec()
        };

        info!(
            "Sending {} sats BTC to user's script program: {}",
            conv.bitcoin_amount,
            if user_program.is_empty() {
                "empty"
            } else {
                "non-empty"
            }
        );

        // Safely downcast bitcoin_amount to u64 for sats
        let amount_sats: u64 = conv
            .bitcoin_amount
            .try_into()
            .map_err(|_| anyhow::anyhow!("btc_amount overflow: {}", conv.bitcoin_amount))?;

        match send_to_user_program(&self.core_ctx, &user_program, amount_sats).await {
            Ok(result) => {
                info!("Sent BTC to user for txId={} (btc_txid={})", tx_id, result);
                self.mark_processed(tx_id, Some(result)).await?;
            }
            Err(e) => {
                error!(error=%e, "Failed to send BTC to user for txId={}", tx_id);
                return Err(e);
            }
        }

        Ok(())
    }

    async fn handle_btc_to_native_conversion(&self, tx_id: U256, conv: Conversion) -> Result<()> {
        let btc_human =
            ethers::utils::format_units(conv.bitcoin_amount, 8).unwrap_or_else(|_| "0".into());
        let native_human =
            ethers::utils::format_units(conv.native_amount, 8).unwrap_or_else(|_| "0".into());
        let sats_str = conv.bitcoin_amount.to_string();

        let user_program = if conv.user_program.0.is_empty() {
            "0x".to_string()
        } else {
            format!("0x{}", hex::encode(&conv.user_program.0))
        };

        info!(
            "\n[BTC→Native] txId={}\n   User received ≈ {} Native from contract.\n   BTC amount committed for this conversion ≈ {} BTC ({} sats).",
            tx_id, native_human, btc_human, sats_str
        );

        info!(
            "SIMULATE: sell {} BTC (received on operator BTC address) -> {} Native on exchange.",
            btc_human, native_human
        );
        info!(
            "SIMULATE: use resulting Native to keep operator side hedged and/or refill contract liquidity if needed."
        );
        info!(
            "userProgram (unused for BTC→Native but shown for parity): {}",
            user_program
        );

        // For DB lookups, downcast safely
        let tx_id_str = tx_id.to_string();

        let row = sqlx::query(
            "SELECT idx
         FROM receive_state
         WHERE tx_id = ?
           AND tx_id != '__next_index__'",
        )
        .bind(&tx_id_str)
        .fetch_optional(&self.sqlite_storage)
        .await?;

        let Some(row) = row else {
            warn!("No receive_state entry for BTC→Native tx_id={}", tx_id);
            return Ok(());
        };

        let idx: u32 = row.get::<i64, _>("idx") as u32;

        let derived = derive_address_from_mnemonic(&self.core_ctx, vec![idx]).await?;
        let info = &derived[0];

        match send_all_btc_from_account_to_dev(&self.core_ctx, &info.address, &info.wif).await {
            Ok(sent_txid) => {
                let txid_opt = if sent_txid.is_empty() {
                    None
                } else {
                    Some(sent_txid)
                };
                self.mark_processed(tx_id, txid_opt).await?;
            }
            Err(err) => {
                warn!(
                    "Failed sending BTC back to main for tx_id={}: {:?}",
                    tx_id, err
                );
            }
        }

        Ok(())
    }

    async fn check_confirmation_and_build_proof(
        &self,
        tx_id: U256,
        btc_txid: &str,
    ) -> Result<Option<BitcoinMerkleProofPayload>> {
        check_confirmation_and_build_proof(&self.core_ctx, tx_id, btc_txid).await
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

    async fn get_processed_native_to_btc(&self, tx_ids: &[U256]) -> Result<HashMap<U256, String>> {
        // Short-circuit: nothing to check
        if tx_ids.is_empty() {
            return Ok(HashMap::new());
        }

        // Convert U256 -> i64 (same as old logic)
        let tx_ids_i64: Vec<i64> = tx_ids
            .iter()
            .map(|id| id.to_string().parse::<i64>())
            .collect::<Result<_, _>>()?;

        // Build dynamic IN (...) placeholders
        let placeholders = std::iter::repeat_n("?", tx_ids_i64.len())
            .collect::<Vec<_>>()
            .join(", ");

        let sql = format!(
            r#"
            SELECT tx_id, btc_tx_id
            FROM processed_conversions
            WHERE tx_id IN ({})
              AND processed = 1
              AND btc_tx_id IS NOT NULL
            "#,
            placeholders
        );

        let mut query = sqlx::query(&sql);

        for id in &tx_ids_i64 {
            query = query.bind(id);
        }

        let rows = query.fetch_all(&self.sqlite_storage).await?;

        let mut result = HashMap::new();

        for row in rows {
            let tx_id_i64: i64 = row.get("tx_id");
            let btc_tx_id: String = row.get("btc_tx_id");

            result.insert(U256::from(tx_id_i64), btc_tx_id);
        }

        Ok(result)
    }
}
