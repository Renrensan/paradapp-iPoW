use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    types::{Bytes, U256},
    utils::hex,
};
use paradapp_core::{
    btc::btc_service::derive_p2wpkh_address,
    consts::{transaction_phase::TransactionPhase, transaction_type::TransactionType},
    context::CoreContext,
    traits::{
        approving_adapter::ApprovingAdapter,
        chain_provider_adapter::{ChainProviderAdapter, TxIdFilter},
    },
};
use sqlx::SqlitePool;
use tracing::{error, info, warn};

use crate::{bindings::paradapp_convert::Conversion, dependencies::context::EvmContext};
use anyhow::{Result, anyhow};

pub struct EvmApprovingAdapter {
    pub ctx: Arc<EvmContext>,
    pub core_ctx: Arc<CoreContext>,
    pub sqlite_storage: SqlitePool,
    pub chain_provider: Arc<dyn ChainProviderAdapter>,
}

#[async_trait]
impl ApprovingAdapter for EvmApprovingAdapter {
    async fn get_or_create_index_for_tx(&self, tx_id: U256) -> Result<u32> {
        let pool: &SqlitePool = &self.sqlite_storage;
        let tx_id_str = tx_id.to_string();

        // Try to fetch existing index
        if let Some(idx) =
            sqlx::query_scalar::<_, i64>("SELECT idx FROM receive_state WHERE tx_id = ?1")
                .bind(&tx_id_str)
                .fetch_optional(pool)
                .await?
        {
            return Ok(idx as u32);
        }

        // Fetch next_index
        let next_index: i64 =
            sqlx::query_scalar("SELECT idx FROM receive_state WHERE tx_id = '__next_index__'")
                .fetch_optional(pool)
                .await?
                .unwrap_or(0);

        // Insert tx_id -> index mapping
        sqlx::query("INSERT INTO receive_state (tx_id, idx) VALUES (?1, ?2)")
            .bind(&tx_id_str)
            .bind(next_index)
            .execute(pool)
            .await?;

        // Increment next_index
        sqlx::query(
            "INSERT INTO receive_state (tx_id, idx) VALUES ('__next_index__', ?1)
             ON CONFLICT(tx_id) DO UPDATE SET idx=excluded.idx",
        )
        .bind(next_index + 1)
        .execute(pool)
        .await?;

        Ok(next_index as u32)
    }

    async fn get_or_create_receive_program_for_tx(
        &self,
        tx_id: U256,
        xpub: &str,
    ) -> Result<(u32, String, Vec<u8>)> {
        // --------------------------------------------------
        // Get or create deterministic receive index
        // --------------------------------------------------
        let index = self.get_or_create_index_for_tx(tx_id).await?;

        // --------------------------------------------------
        //  BTC derivation
        // --------------------------------------------------
        let (idx, address, script) = derive_p2wpkh_address(xpub, index, self.core_ctx.btc_network)?;

        Ok((idx, address, script))
    }

    async fn handle_operator_closes_for_active(&self, tx_id: U256, conf_req: u64) -> Result<()> {
        // 1. Fetch conversion info
        let (conv, _phase): (Conversion, u8) = self
            .ctx
            .contract
            .get_conversion_with_phase(tx_id)
            .call()
            .await?;

        let now_sec = chrono::Utc::now().timestamp() as u64;

        // 2. Fetch window info
        let (headers_started, _start_height, last_height, deposit_end, proof_end, duty_expires_at) =
            self.ctx.contract.windows_for(tx_id).call().await?;

        if !headers_started {
            return Ok(());
        }

        let c_op = self.ctx.c_op.clone();

        // 3. Native → BTC
        if conv.is_native_to_bitcoin {
            if !conv.approved || conv.completed || conv.refunded {
                return Ok(());
            }

            if !conv.deposited {
                let deposit_over = last_height > deposit_end;
                let duty_active =
                    duty_expires_at != U256::zero() && now_sec <= duty_expires_at.as_u64();

                if deposit_over && duty_active {
                    info!(
                        tx_id = %tx_id,
                        "[op] Native→BTC txId={} padeposit, timeoutNoDeposit_NativeTokentoBTC",
                        tx_id
                    );

                    // STATIC CALL
                    let _ = c_op
                        .timeout_no_deposit_nativeto_bitcoin(tx_id)
                        .call()
                        .await?;

                    // SEND TX
                    match c_op.timeout_no_deposit_nativeto_bitcoin(tx_id).send().await {
                        Ok(pending) => {
                            info!(
                                tx_hash = ?pending.tx_hash(),
                                tx_id = %tx_id,
                                "timeout_no_deposit_nativeto_bitcoin tx sent"
                            );
                        }
                        Err(e) => {
                            warn!(
                                tx_id = %tx_id,
                                error = %e,
                                "Failed to send timeout_no_deposit_hba_rto_btc — retrying next cycle"
                            );
                            return Ok(());
                        }
                    }
                }
            }
        } else {
            // 4. BTC → Native
            if !conv.approved || conv.completed || conv.refunded {
                return Ok(());
            }

            let end_height = proof_end + (conf_req - 1);
            let window_over = last_height > end_height;
            let duty_active =
                duty_expires_at != U256::zero() && now_sec <= duty_expires_at.as_u64();

            if window_over && duty_active {
                info!(
                    tx_id = %tx_id,
                    "[op] BTC→Native txId={} window over, closeNoBTC_BTCtoNative",
                    tx_id
                );

                let c_op = self.ctx.c_op.clone();

                // 1. Static call
                let call_static = c_op.close_no_bitcoin_bitcoin_to_native(tx_id);
                call_static.call().await?;

                // 2. Send transaction non blocking
                match c_op.close_no_bitcoin_bitcoin_to_native(tx_id).send().await {
                    Ok(pending) => {
                        info!(
                            tx_hash = ?pending.tx_hash(),
                            tx_id = %tx_id,
                            "close_no_bitcoin_bitcoin_to_native tx sent)"
                        );
                    }
                    Err(e) => {
                        warn!(
                            tx_id = %tx_id,
                            error = %e,
                            "Failed to send close_no_bitcoin_bitcoin_to_native — retrying next cycle"
                        );
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    }

    async fn discover_user_close_candidates(
        &self,
        to_tx_id: U256,
        conf_req: u64,
    ) -> Result<Vec<(U256, String)>> {
        let conf_req_bn = U256::from(conf_req);

        use futures::try_join;

        // --- ACTIVE_WAITING_PROOF ---
        let (native_to_btc, btc_to_native, native_to_native_in, native_to_native_out) = try_join!(
            // 1. Native -> BTC
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_BITCOIN,
                phase_filter: TransactionPhase::ACTIVE_WAITING_PROOF,
                to_tx_id,
                ..Default::default()
            }),
            // 2. BTC -> Native
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::BITCOIN_TO_NATIVE,
                phase_filter: TransactionPhase::ACTIVE_WAITING_PROOF,
                to_tx_id,
                ..Default::default()
            }),
            // 3. Native -> Native IN
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_IN,
                phase_filter: TransactionPhase::ACTIVE_WAITING_PROOF,
                to_tx_id,
                ..Default::default()
            }),
            // 4. Native -> Native OUT
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                phase_filter: TransactionPhase::ACTIVE_WAITING_PROOF,
                to_tx_id,
                ..Default::default()
            }),
        )?;

        let active_txs: Vec<U256> = native_to_btc
            .into_iter()
            .chain(btc_to_native)
            .chain(native_to_native_in)
            .chain(native_to_native_out)
            .collect();

        let (native_to_btc, btc_to_native, native_to_native_in, native_to_native_out) = try_join!(
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_BITCOIN,
                phase_filter: TransactionPhase::OPERATOR_DUTY_EXPIRED,
                to_tx_id,
                ..Default::default()
            }),
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::BITCOIN_TO_NATIVE,
                phase_filter: TransactionPhase::OPERATOR_DUTY_EXPIRED,
                to_tx_id,
                ..Default::default()
            }),
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_IN,
                phase_filter: TransactionPhase::OPERATOR_DUTY_EXPIRED,
                to_tx_id,
                ..Default::default()
            }),
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::NATIVE_TO_NATIVE_OUT,
                phase_filter: TransactionPhase::OPERATOR_DUTY_EXPIRED,
                to_tx_id,
                ..Default::default()
            }),
        )?;

        let duty_expired_txs: Vec<U256> = native_to_btc
            .into_iter()
            .chain(btc_to_native)
            .chain(native_to_native_in)
            .chain(native_to_native_out)
            .collect();

        let mut seen = std::collections::HashSet::<U256>::new();
        let mut candidates: Vec<(U256, String)> = Vec::new();

        let contract = self.ctx.contract.clone();
        let c_op = self.ctx.c_op.clone();

        // --- ACTIVE logic ---
        for tx_id in active_txs {
            if !seen.insert(tx_id) {
                continue;
            }

            let (conv, _phase): (Conversion, u8) =
                contract.get_conversion_with_phase(tx_id).call().await?;

            let (
                headers_started,
                _start_height,
                last_height,
                _deposit_end,
                proof_end,
                _duty_expires_at,
            ) = contract.windows_for(tx_id).call().await?;

            if !headers_started
                || !conv.is_native_to_bitcoin
                || !conv.approved
                || conv.completed
                || conv.refunded
                || !conv.deposited
            {
                continue;
            }

            let end_height = proof_end + (conf_req_bn - U256::from(1));
            if last_height > end_height
                && c_op
                    .refund_after_no_proof_native_to_bitcoin(tx_id)
                    .call()
                    .await
                    .is_ok()
            {
                candidates.push((tx_id, "refundAfterNoProof_NativeTokentoBTC".to_string()));
            }
        }

        // --- DUTY_EXPIRED logic ---
        for tx_id in duty_expired_txs {
            if !seen.insert(tx_id) {
                continue;
            }

            let (conv, _phase): (Conversion, u8) =
                contract.get_conversion_with_phase(tx_id).call().await?;

            if !conv.approved || conv.completed || conv.refunded {
                continue;
            }

            if conv.is_native_to_bitcoin {
                if c_op
                    .refund_after_no_proof_native_to_bitcoin(tx_id)
                    .call()
                    .await
                    .is_ok()
                {
                    candidates.push((tx_id, "refundAfterNoProof_NativeTokentoBTC".to_string()));
                }
            } else if c_op
                .claim_native_after_operator_expired(tx_id)
                .call()
                .await
                .is_ok()
            {
                candidates.push((tx_id, "claimNative_AfterOperatorExpired".to_string()));
            }
        }

        info!(
            count = candidates.len(),
            "Discovered user-close candidates (for jump cost comparison)"
        );

        Ok(candidates)
    }

    async fn execute_user_closes(&self, candidates: Vec<(U256, &'static str)>) -> Result<()> {
        let c_op = self.ctx.c_op.clone();

        for (tx_id, kind) in candidates {
            match kind {
                "refundAfterNoProof_NativeTokentoBTC" => {
                    info!(
                        tx_id = %tx_id,
                        "[jump] User-close refundAfterNoProof_NativeTokentoBTC"
                    );

                    // 1. static call
                    let can_execute = c_op
                        .refund_after_no_proof_native_to_bitcoin(tx_id)
                        .call()
                        .await;

                    if can_execute.is_err() {
                        continue;
                    }

                    // 2. send real transaction
                    match c_op
                        .refund_after_no_proof_native_to_bitcoin(tx_id)
                        .send()
                        .await
                    {
                        Ok(pending) => {
                            let tx_hash = pending.tx_hash();
                            let _ = pending.await;

                            info!(
                                tx_hash = ?tx_hash,
                                tx_id = %tx_id,
                                "refundAfterNoProof_NativeTokentoBTC tx mined"
                            );
                        }
                        Err(e) => {
                            warn!(
                                tx_id = %tx_id,
                                error = %e,
                                "Failed to send refundAfterNoProof_NativeTokentoBTC — retrying next cycle"
                            );
                        }
                    }
                }

                "claimNative_AfterOperatorExpired" => {
                    info!(
                        tx_id = %tx_id,
                        "[jump] User-close claimNative_AfterOperatorExpired"
                    );

                    // 1. static call
                    let can_execute = c_op.claim_native_after_operator_expired(tx_id).call().await;

                    if can_execute.is_err() {
                        continue;
                    }

                    // 2. send real transaction
                    match c_op.claim_native_after_operator_expired(tx_id).send().await {
                        Ok(pending) => {
                            let tx_hash = pending.tx_hash();
                            let _ = pending.await;

                            info!(
                                tx_hash = ?tx_hash,
                                tx_id = %tx_id,
                                "claimNative_AfterOperatorExpired tx mined"
                            );
                        }
                        Err(e) => {
                            warn!(
                                tx_id = %tx_id,
                                error = %e,
                                "Failed to send claimNative_AfterOperatorExpired — retrying next cycle"
                            );
                        }
                    }
                }

                _ => continue,
            }
        }

        Ok(())
    }

    async fn approve_one_tx(&self, tx_id: U256, duty_seconds: u64) -> Result<()> {
        let contract = &self.ctx.contract.clone();
        let c_op = &self.ctx.c_op.clone();

        // 1. Load the conversion data
        let conv = contract.conversions(tx_id).call().await?;
        let (
            _user,
            is_native_to_bitcoin,
            _slippage,
            _user_program,
            _paradapp_receive_program,
            _network_address,
            network_id,
            _native_amount,
            _bitcoin_amount,
            _created_at,
            _approved_at,
            _deposited_at,
            _commit_fee,
            _approved,
            _deposited,
            _completed,
            _refunded,
            _reserved_native,
            _operator_duty_expires_at,
        ) = conv;

        // 2. Decide scriptArg
        let xpub_str: &str = &self.ctx.cfg.btc_root_xpub;

        let script_arg: Vec<u8> = match (is_native_to_bitcoin, network_id.is_zero()) {
            // Path A: Not Native-to-Bitcoin (B2N) OR it is N2N (Cross-network)
            (false, _) | (true, false) => {
                match self
                    .get_or_create_receive_program_for_tx(tx_id, xpub_str)
                    .await
                {
                    Ok((index, address, script_buf)) => {
                        info!(
                            tx_id = %tx_id,
                            address = %address,
                            index = index,
                            is_n2n = (network_id != U256::zero()),
                            "Assigned BTC addr via XPUB for conversion"
                        );
                        script_buf
                    }
                    Err(err) => {
                        warn!(
                            tx_id = %tx_id,
                            error = %err,
                            "Failed deriving address from XPUB"
                        );
                        return Ok(());
                    }
                }
            }

            // Path B: Pure Native-to-Bitcoin (Network ID 0)
            (true, true) => {
                if let Some(static_program) = &self.core_ctx.cfg.paradapp_receive_program {
                    let decoded =
                        hex::decode(static_program.trim_start_matches("0x")).unwrap_or_default();

                    info!(
                        tx_id = %tx_id,
                        "Native→BTC tx using static receive program from PARADAPP_RECEIVE_PROGRAM"
                    );
                    decoded
                } else {
                    info!(
                        tx_id = %tx_id,
                        "Cannot approve Native→BTC tx – missing PARADAPP_RECEIVE_PROGRAM"
                    );
                    return Err(anyhow!("missing receive program for Native→BTC"));
                }
            }
        };

        info!(
            tx_id = %tx_id,
            is_native_to_bitcoin = %is_native_to_bitcoin,
            "Trying to approve transaction"
        );

        // Convert hex strings (anchor80 / first80) to Bytes
        let script_arg_bytes = Bytes::from(script_arg);

        // Build the call ONCE
        let duty_seconds_bn = U256::from(duty_seconds);
        let call = c_op.approve_and_start_with_anchor_and_first(
            tx_id,
            duty_seconds_bn,
            script_arg_bytes.clone(),
            1000,
        );

        // 3. callStatic once
        if let Err(err) = call.clone().call().await {
            error!(
                tx_id = %tx_id,
                err = %err,
                "callStatic approve failed"
            );
            return Ok(());
        }

        // 4. Send real tx — fire-and-forget
        match call.send().await {
            Ok(pending) => {
                info!(
                    tx_hash = ?pending.tx_hash(),
                    tx_id = %tx_id,
                    "Sent approve tx"
                );
            }
            Err(e) => {
                warn!(
                    tx_id = %tx_id,
                    error = %e,
                    "Failed to send approve tx — retrying next cycle"
                );
            }
        }

        Ok(())
    }
}
