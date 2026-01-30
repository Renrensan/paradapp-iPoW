use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use ethers::types::{Address, Bytes, U256};
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
    traits::{chain_helper_adapter::ChainHelperAdapter, converting_adapter::ConvertingAdapter},
};
use sqlx::{Row, SqlitePool};
use tracing::{error, info, warn};

use crate::{
    bindings::paradapp_convert,
    common::{consts::liquidity::Liquidity, helpers::parse_native_token::parse_human_native_token},
    dependencies::context::EvmContext,
};
use anyhow::Result;

pub struct EvmConvertingAdapter {
    pub ctx: Arc<EvmContext>,
    pub core_ctx: Arc<CoreContext>,
    pub sqlite_storage: SqlitePool,
    pub helper: Arc<dyn ChainHelperAdapter>,
}

#[async_trait]
impl ConvertingAdapter for EvmConvertingAdapter {
    type Conversion = paradapp_convert::Conversion;

    async fn check_rpc_health(&self) -> Result<()> {
        // EVM RPC ===
        let evm_ok = match self
            .ctx
            .provider
            .request::<(), ethers::types::U64>("eth_blockNumber", ())
            .await
        {
            Ok(bn) => {
                info!(block_number = %bn, "EVM RPC alive");
                true
            }
            Err(e) => {
                error!(error = %e, "EVM RPC health check failed");
                false
            }
        };

        // === Bitcoin Esplora ===
        let btc_ok = {
            let url = format!("{}/blocks/tip/height", self.core_ctx.cfg.esplora_base);

            match self.core_ctx.http.get(&url).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => match text.parse::<u32>() {
                        Ok(height) => {
                            info!(
                                url = %url,
                                height,
                                "Bitcoin Esplora health check OK"
                            );
                            true
                        }
                        Err(e) => {
                            warn!(
                                url = %url,
                                response = %text,
                                error = %e,
                                "Bitcoin Esplora returned invalid height"
                            );
                            false
                        }
                    },
                    Err(e) => {
                        error!(
                            url = %url,
                            error = %e,
                            "Bitcoin Esplora failed to read response body"
                        );
                        false
                    }
                },
                Err(e) => {
                    error!(
                        url = %url,
                        error = %e,
                        "Bitcoin Esplora request failed"
                    );
                    false
                }
            }
        };

        if !evm_ok || !btc_ok {
            anyhow::bail!("one or more upstream RPCs are down");
        }

        Ok(())
    }

    async fn next_tx_id(&self) -> Result<U256> {
        // ---- Determine latest tx id ----
        let next_tx_id: U256 = self
            .ctx
            .c_op
            .next_tx_id()
            .call()
            .await
            .map_err(anyhow::Error::from)?;
        Ok(next_tx_id)
    }

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

    async fn read_liquidity(&self) -> Result<U256> {
        let contract = self.ctx.contract.clone();

        let mut native_liq = U256::zero();
        {
            let call = contract.native_liquidity();
            match call.call().await {
                Ok(v) => {
                    native_liq = v;
                }
                Err(e) => {
                    info!(
                        error = %e,
                        "nativeLiquidity() view not found or failed; treating as 0."
                    );
                }
            }
        }

        // Format logs with tracing
        let native_fmt = ethers::utils::format_ether(native_liq);

        info!(
            native = %native_fmt,
            raw_native = ?native_liq,
            "On-chain liquidity"
        );

        Ok(native_liq)
    }

    async fn maybe_rebalance_contract_liquidity(&self, native_liq: U256) -> Result<()> {
        let c_op = self.ctx.c_op.clone();
        let low_native = parse_human_native_token(Liquidity::HBAR_LIQ_LOW)?;
        let high_native = parse_human_native_token(Liquidity::HBAR_LIQ_HIGH)?;
        let enable_topup: bool = self.ctx.cfg.enable_onchain_lp_topup.to_lowercase() == "true";

        if native_liq < low_native {
            let need_native = low_native - native_liq;

            info!(
                needed = %ethers::utils::format_ether(need_native),
                "Native liquidity below low threshold."
            );

            if enable_topup {
                info!("addNativeLiquidity: operator wallet → contract");

                let call = c_op.add_native_liquidity().value(need_native);
                match call.send().await {
                    Ok(pending) => {
                        info!(
                            tx_hash = ?pending.tx_hash(),
                        "addNativeLiquidity tx broadcasted.")
                    }
                    Err(e) => error!(error=%e,"addNativeLiquidity failed"),
                }
            } else {
                info!(
                    need = %ethers::utils::format_ether(need_native),
                    "   (SIMULATION ONLY) Withdraw Native Token from exchange → operator wallet → addNativeLiquidity(needNative)."
                );
            }
        } else if native_liq > high_native {
            let excess = native_liq - high_native;

            info!(
                excess = %ethers::utils::format_ether(excess),
                "Native liquidity above high threshold."
            );
            info!("   TODO: call withdrawNativeLiquidity() → deposit to exchange.");
        } else {
            info!("Native liquidity within range – no rebalance needed.");
        }

        Ok(())
    }

    async fn find_native_to_btc_ready(
        &self,
        to_tx_id: U256,
        dest_network: Option<SupportedNetwork>,
    ) -> Result<Vec<(U256, Self::Conversion)>> {
        let user_filter = Address::zero();
        let mut ready = Vec::new();

        let (dest_network_u256, use_network_filter): (U256, bool) = match dest_network {
            Some(net) => (U256::from(net as u8), true),
            None => (U256::zero(), false), // ignored when use_network_filter == false
        };
        // ---------- Native → BTC ----------
        let active_native_to_btc: Vec<U256> = self
            .ctx
            .contract
            .get_tx_ids_by_filter(
                TransactionType::NATIVE_TO_BITCOIN,
                TransactionPhase::ACTIVE_WAITING_PROOF,
                user_filter,
                Bytes::new(),
                dest_network_u256,
                use_network_filter,
                U256::one(),
                to_tx_id,
                U256::from(500),
            )
            .call()
            .await
            .map_err(anyhow::Error::from)?;

        info!(
            to_tx_id = %to_tx_id,
            count = active_native_to_btc.len(),
            tx_ids = ?active_native_to_btc,
            "Contract returned ACTIVE_WAITING_PROOF Native→BTC txs"
        );

        for tx_id in active_native_to_btc {
            let conv = self
                .ctx
                .c_op
                .get_conversion_with_phase(tx_id)
                .call()
                .await?
                .0;

            if !conv.is_native_to_bitcoin {
                continue;
            }

            if !conv.approved || conv.completed || conv.refunded {
                continue;
            }

            if !conv.deposited {
                continue;
            }

            ready.push((tx_id, conv));
        }

        // ---------- Native → Native Out ----------
        let active_native_to_native_out: Vec<U256> = self
            .ctx
            .contract
            .get_tx_ids_by_filter(
                TransactionType::NATIVE_TO_NATIVE_OUT,
                TransactionPhase::ACTIVE_WAITING_PROOF,
                user_filter,
                Bytes::new(),
                dest_network_u256,
                use_network_filter,
                U256::one(),
                to_tx_id,
                U256::from(500),
            )
            .call()
            .await
            .map_err(anyhow::Error::from)?;

        info!(
            to_tx_id = %to_tx_id,
            count = active_native_to_native_out.len(),
            tx_ids = ?active_native_to_native_out,
            "Contract returned ACTIVE_WAITING_PROOF Native→NativeOut txs"
        );

        for tx_id in active_native_to_native_out {
            let conv = self
                .ctx
                .c_op
                .get_conversion_with_phase(tx_id)
                .call()
                .await?
                .0;

            // Same readiness rules
            if !conv.approved || conv.completed || conv.refunded {
                continue;
            }

            if !conv.deposited {
                continue;
            }

            ready.push((tx_id, conv));
        }

        if !ready.is_empty() {
            info!(
                to_tx_id = %to_tx_id,
                count = ready.len(),
                tx_ids = ?ready.iter().map(|r| r.0).collect::<Vec<_>>(),
                "Found ready Native→BTC and Native→NativeOut conversions awaiting payout"
            );
        }

        Ok(ready)
    }

    async fn find_btc_to_native_completed(
        &self,
        to_tx_id: U256,
        dest_network: Option<SupportedNetwork>,
    ) -> Result<Vec<(U256, Self::Conversion)>> {
        let mut completed = Vec::new();

        let (dest_network_u256, use_network_filter): (U256, bool) = match dest_network {
            Some(net) => (U256::from(net as u8), true),
            None => (U256::zero(), false), // ignored when use_network_filter == false
        };

        for tx_type in [
            TransactionType::BITCOIN_TO_NATIVE,
            TransactionType::NATIVE_TO_NATIVE_OUT,
        ] {
            let mut ids: Vec<U256> = self
                .ctx
                .contract
                .get_tx_ids_by_filter(
                    tx_type,
                    TransactionPhase::COMPLETED,
                    Address::zero(),
                    Bytes::new(),
                    dest_network_u256,
                    use_network_filter,
                    U256::one(),
                    to_tx_id,
                    U256::from(500),
                )
                .call()
                .await
                .map_err(anyhow::Error::from)?;

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

            let conv = self
                .ctx
                .c_op
                .get_conversion_with_phase(*tx_id)
                .call()
                .await?
                .0;

            if conv.is_native_to_bitcoin {
                continue;
            }

            if !conv.completed || conv.refunded {
                continue;
            }

            ready.push((*tx_id, conv));
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
        conv: Self::Conversion,
    ) -> Result<()> {
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

    async fn handle_btc_to_native_conversion(
        &self,
        tx_id: U256,
        conv: Self::Conversion,
    ) -> Result<()> {
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
            "✅ Merkle proof submitted txId={} | contract_tx_hash={:?}",
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
