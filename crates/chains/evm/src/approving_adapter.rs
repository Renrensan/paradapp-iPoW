use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    types::{Address, Bytes, U256},
    utils::hex,
};
use paradapp_core::{
    btc::btc_service::{
        btc_tip_height, decode_header80, derive_p2wpkh_address, header80_by_height,
    },
    consts::{transaction_phase::TransactionPhase, transaction_type::TransactionType},
    context::CoreContext,
    traits::approving_adapter::{ApprovingAdapter, GlobalChainState},
};
use sqlx::SqlitePool;
use tracing::{error, info, warn};

use crate::{
    bindings::paradapp_convert::Conversion, common::helpers::preflight::preflight_commit_global,
    dependencies::context::EvmContext,
};
use anyhow::{Context, Result, anyhow};

pub struct EvmApprovingAdapter {
    pub ctx: Arc<EvmContext>,
    pub core_ctx: Arc<CoreContext>,
    pub sqlite_storage: SqlitePool,
}

#[async_trait]
impl ApprovingAdapter for EvmApprovingAdapter {
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

            match self.core_ctx.http.get(url).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => text.parse::<u32>().is_ok(),
                    Err(_) => false,
                },
                Err(e) => {
                    error!(error = %e, "Bitcoin Esplora health failed");
                    false
                }
            }
        };

        if !evm_ok || !btc_ok {
            anyhow::bail!("one or more upstream RPCs are down");
        }

        Ok(())
    }

    fn epoch_start(&self, height: u64) -> u64 {
        height - (height % 2016)
    }

    async fn get_global_chain_state(&self) -> Result<GlobalChainState> {
        let contract = &self.ctx.contract;

        let next_tx_id = contract.next_tx_id().call().await?;
        let conf_req = contract.confirmations_required().call().await?.as_u64();
        let global_tip = contract.global_tip_height().call().await?.as_u64();
        let active_open = contract.active_open_conversions().call().await?.as_u64();
        let btc_tip = btc_tip_height(&self.core_ctx).await?;

        Ok(GlobalChainState {
            next_tx_id,
            confirmations_required: conf_req,
            btc_tip,
            safe_anchor: btc_tip,
            global_tip,
            active_open,
        })
    }

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
    ) -> Result<(u32, String, Vec<u8>)> {
        // --------------------------------------------------
        // Load XPUB from core config
        // --------------------------------------------------
        let xpub = self
            .core_ctx
            .cfg
            .btc_root_xpub
            .as_ref()
            .ok_or_else(|| anyhow!("BTC_ROOT_XPUB not set"))?;

        if xpub.is_empty() {
            return Err(anyhow!("BTC_ROOT_XPUB is empty"));
        }

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

    async fn jump_to_anchor_from_zero_active(&self, global_tip: u64, anchor_h: u64) -> Result<u64> {
        info!(
            input_global_tip = %global_tip,
            input_anchor_h = %anchor_h,
            "jump_to_anchor_from_zero_active called"
        );

        if anchor_h <= global_tip {
            info!(
                anchor_h = %anchor_h,
                global_tip = %global_tip,
                "No jump needed — anchor already <= global tip"
            );
            return Ok(global_tip);
        }

        let first_h = self.epoch_start(anchor_h);
        info!(
            calculated_first_h = %first_h,
            target_anchor_h = %anchor_h,
            current_global_tip = %global_tip,
            "Planning jump: first_h={} → anchor_h={}",
            first_h, anchor_h
        );

        let mut committed_up_to: u64 = global_tip;

        // === 1. Commit epoch-first header ===
        if first_h > global_tip && first_h > 0 {
            info!(height = %first_h, "Attempting to commit epoch-first header");

            let (_, header80) = header80_by_height(&self.core_ctx, first_h)
                .await
                .with_context(|| {
                    format!("Failed to fetch epoch-first header at height {first_h}")
                })?;

            let header80_bytes = decode_header80(&header80)
                .map_err(|e| anyhow!("decode failed for first_h={first_h}: {e}"))?;

            let preflight =
                preflight_commit_global(&self.ctx, header80_bytes.clone(), first_h).await;

            if !preflight.static_ok {
                let err = preflight.static_err.unwrap_or_default();
                if err.to_string().to_lowercase().contains("height-rewrite") {
                    info!(height = %first_h, "Epoch-first already committed (height-rewrite)");
                    committed_up_to = committed_up_to.max(first_h);
                } else {
                    return Err(anyhow!("Preflight failed for epoch-first {first_h}: {err}"));
                }
            } else {
                match self
                    .ctx
                    .c_op
                    .commit_global_bitcoin_header_80(header80_bytes, U256::from(first_h), vec![])
                    .send()
                    .await
                {
                    Ok(pending) => {
                        info!(
                            tx_hash = ?pending.tx_hash(),
                            height = %first_h,
                            "Epoch-first header TX BROADCASTED"
                        );
                        committed_up_to = committed_up_to.max(first_h);
                    }
                    Err(e) => {
                        warn!(
                            height = %first_h,
                            error = %e,
                            "Failed to broadcast epoch-first — will retry"
                        );
                        // Do NOT advance tip
                    }
                }
            }
        } else {
            info!(
                first_h = %first_h,
                global_tip = %global_tip,
                "Skipping epoch-first: already at or beyond"
            );
            committed_up_to = committed_up_to.max(first_h);
        }

        // === 2. Commit actual anchor header ===
        info!(height = %anchor_h, "Now committing anchor header");

        let (_, anchor80) = header80_by_height(&self.core_ctx, anchor_h)
            .await
            .with_context(|| format!("Failed to fetch anchor header {anchor_h}"))?;

        let anchor80_bytes = decode_header80(&anchor80)
            .map_err(|e| anyhow!("decode failed for anchor_h={anchor_h}: {e}"))?;

        let preflight = preflight_commit_global(&self.ctx, anchor80_bytes.clone(), anchor_h).await;

        if !preflight.static_ok {
            let err = preflight.static_err.unwrap_or_default();
            let err_msg = err.to_string().to_lowercase();
            if err_msg.contains("height-rewrite") {
                info!(height = %anchor_h, "Anchor already stored (height-rewrite)");
            } else if err_msg.contains("no-jump-while-active") {
                warn!(height = %anchor_h, "no-jump-while-active triggered — possible race!");
                return Ok(committed_up_to);
            } else {
                return Err(anyhow!("Preflight failed for anchor {anchor_h}: {err}"));
            }
        } else {
            match self
                .ctx
                .c_op
                .commit_global_bitcoin_header_80(anchor80_bytes, U256::from(anchor_h), vec![])
                .send()
                .await
            {
                Ok(pending) => {
                    info!(
                        tx_hash = ?pending.tx_hash(),
                        height = %anchor_h,
                        "ANCHOR HEADER TX BROADCASTED — jump successful"
                    );
                    committed_up_to = anchor_h; // Critical: update to anchor_h, not first_h!
                }
                Err(e) => {
                    warn!(
                        height = %anchor_h,
                        error = %e,
                        "Failed to broadcast anchor header — will retry next cycle"
                    );
                    // Do not advance
                }
            }
        }

        info!(
            final_committed_up_to = %committed_up_to,
            original_global_tip = %global_tip,
            target_anchor = %anchor_h,
            "jump_to_anchor_from_zero_active finished"
        );

        Ok(committed_up_to)
    }

    async fn get_tx_ids_by_phase(
        &self,
        phase: u8,
        type_filter: u8,
        from_tx_id: U256,
        to_tx_id: U256,
        max_results: U256,
    ) -> Result<Vec<U256>> {
        // Zero address = wildcard user
        let user_filter = Address::zero();

        // Call Solidity method `getTxIdsByFilter`
        let tx_ids_bn: Vec<U256> = self
            .ctx
            .contract
            .get_tx_ids_by_filter(
                type_filter,
                phase,
                user_filter,
                Bytes::new(),
                from_tx_id,
                to_tx_id,
                max_results,
            )
            .call()
            .await?;

        Ok(tx_ids_bn)
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
                        "[op] Native→BTC txId={} no deposit, timeoutNoDeposit_NativeTokentoBTC",
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

        let max_results = U256::from(500u64);
        let from_tx_id = U256::from(1u64);

        use futures::try_join;

        // --- ACTIVE_WAITING_PROOF ---
        let (native_to_btc, btc_to_native) = try_join!(
            self.get_tx_ids_by_phase(
                TransactionPhase::ACTIVE_WAITING_PROOF,
                TransactionType::NATIVE_TO_BITCOIN,
                from_tx_id,
                to_tx_id,
                max_results,
            ),
            self.get_tx_ids_by_phase(
                TransactionPhase::ACTIVE_WAITING_PROOF,
                TransactionType::BITCOIN_TO_NATIVE,
                from_tx_id,
                to_tx_id,
                max_results,
            ),
        )?;

        let active_txs: Vec<U256> = native_to_btc.into_iter().chain(btc_to_native).collect();

        // --- OPERATOR_DUTY_EXPIRED ---
        let (native_to_btc, btc_to_native) = try_join!(
            self.get_tx_ids_by_phase(
                TransactionPhase::OPERATOR_DUTY_EXPIRED,
                TransactionType::NATIVE_TO_BITCOIN,
                from_tx_id,
                to_tx_id,
                max_results,
            ),
            self.get_tx_ids_by_phase(
                TransactionPhase::OPERATOR_DUTY_EXPIRED,
                TransactionType::BITCOIN_TO_NATIVE,
                from_tx_id,
                to_tx_id,
                max_results,
            ),
        )?;

        let duty_expired_txs: Vec<U256> = native_to_btc.into_iter().chain(btc_to_native).collect();

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
                        "⚠️ [jump] User-close claimNative_AfterOperatorExpired"
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

    async fn get_pending_txids(&self, max_results: u32) -> Result<Vec<U256>> {
        let contract = self.ctx.contract.clone();

        let next_tx_id: U256 = contract.next_tx_id().call().await?;
        if next_tx_id <= U256::from(1u64) {
            return Ok(vec![]);
        }

        // toTxId = nextTxId - 1
        let to_tx_id = next_tx_id - U256::from(1u64);

        let mut tx_ids: Vec<U256> = contract
            .get_tx_ids_by_filter(
                TransactionType::NATIVE_TO_BITCOIN,
                TransactionPhase::WAITING_OPERATOR_APPROVAL,
                Address::zero(),
                Bytes::new(),
                U256::from(1u64),
                to_tx_id,
                U256::from(max_results),
            )
            .call()
            .await?;

        let mut btc_to_native_tx_ids: Vec<U256> = contract
            .get_tx_ids_by_filter(
                TransactionType::BITCOIN_TO_NATIVE,
                TransactionPhase::WAITING_OPERATOR_APPROVAL,
                Address::zero(),
                Bytes::new(),
                U256::from(1u64),
                to_tx_id,
                U256::from(max_results),
            )
            .call()
            .await?;

        tx_ids.append(&mut btc_to_native_tx_ids);

        Ok(tx_ids)
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
            _network_id,
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
        let mut script_arg: Vec<u8> = Vec::new();

        if !is_native_to_bitcoin {
            if let Some(xpub) = &self.core_ctx.cfg.btc_root_xpub {
                if !xpub.is_empty() {
                    match self.get_or_create_receive_program_for_tx(tx_id).await {
                        Ok((index, address, script_buf)) => {
                            script_arg = script_buf.clone();
                            info!(
                                tx_id = %tx_id,
                                address = address,
                                index = index,
                                "BTC→Native txId={} assigned BTC addr={} (index={})",
                                tx_id, address, index
                            );
                        }
                        Err(err) => {
                            info!(
                                tx_id = %tx_id,
                                "Cannot approve BTC→Native txId={} – failed deriving address: {err}",
                                tx_id
                            );
                            return Ok(());
                        }
                    }
                } else if let Some(static_program) = &self.core_ctx.cfg.paradapp_receive_program {
                    script_arg =
                        hex::decode(static_program.trim_start_matches("0x")).unwrap_or_default();
                    info!(
                        tx_id = %tx_id,
                        "BTC→Native txId={} using static receive program from PARADAPP_RECEIVE_PROGRAM",
                        tx_id
                    );
                } else {
                    info!(
                        tx_id = %tx_id,
                        "Cannot approve BTC→Native txId={} – no BTC_ROOT_XPUB or PARADAPP_RECEIVE_PROGRAM",
                        tx_id
                    );
                    return Err(anyhow!("missing receive program for BTC→Native"));
                }
            } else if let Some(static_program) = &self.core_ctx.cfg.paradapp_receive_program {
                script_arg = hex::decode(static_program.trim_start_matches("0x"))?;
                info!(
                    tx_id = %tx_id,
                    "BTC→Native txId={} using static receive program from PARADAPP_RECEIVE_PROGRAM",
                    tx_id
                );
            } else {
                info!(
                    tx_id = %tx_id,
                    "Cannot approve BTC→Native txId={} – no BTC_ROOT_XPUB or PARADAPP_RECEIVE_PROGRAM",
                    tx_id
                );
                return Err(anyhow!("missing receive program for BTC→Native"));
            }
        }

        info!(
            tx_id = %tx_id,
            is_native_to_bitcoin = %is_native_to_bitcoin,
            "🧷 Trying to approve transaction"
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

    async fn stream_headers_to_height(
        &self,
        current_tip: u64,
        up_to_height: u64,
        max_count: u64,
    ) -> Result<u64> {
        let start = current_tip + 1;
        let end = std::cmp::min(up_to_height, current_tip + max_count);
        if end < start {
            return Ok(current_tip);
        }

        info!(
            start = %start,
            end = %end,
            "Streaming headers from height {} to {} (contiguous, approve-bot)",
            start, end
        );

        let mut new_tip = current_tip;
        let c_op = self.ctx.c_op.clone();

        for h in start..=end {
            // 1. Fetch header80
            let (_, header80) = header80_by_height(&self.core_ctx, h)
                .await
                .with_context(|| format!("Failed to fetch header80 for height {h}"))?;

            // 2. Preflight check
            let header80_bytes = decode_header80(&header80)
                .map_err(|e| anyhow!("failed to decode header80 at height {h}: {e}"))?;
            let preflight = preflight_commit_global(&self.ctx, header80_bytes.clone(), h).await;

            if !preflight.static_ok {
                let err_msg = preflight
                    .static_err
                    .map(|e| e.to_string())
                    .unwrap_or_default()
                    .to_lowercase();

                if err_msg.contains("height-rewrite") {
                    info!(
                        height = %h,
                        "height already stored, skipping"
                    );
                    new_tip = h; // still advance tip (assume it's stored)
                    continue;
                } else if err_msg.contains("no-jump-while-active") {
                    warn!(
                        height = %h,
                        "no-jump-while-active, stopping stream"
                    );
                    return Ok(new_tip);
                } else {
                    error!(
                        height = %h,
                        error = %err_msg,
                        "commitGlobalBTCHeader80 failed"
                    );
                    return Ok(new_tip);
                }
            }

            // 3. Commit the header
            match c_op
                .commit_global_bitcoin_header_80(header80_bytes, U256::from(h), Vec::<U256>::new())
                .send()
                .await
            {
                Ok(pending_tx) => {
                    if let Some(_receipt) = pending_tx.await? {
                        info!(height = %h, "Global header stored");
                        new_tip = h;
                    } else {
                        warn!(height = %h, "Tx for height was dropped/not mined");
                    }
                }
                Err(e) => {
                    // Likely nonce/gas issue — log and stop
                    error!(
                        height = h,
                        error = %e,
                        "Failed to send commit tx for height"
                    );
                    return Ok(new_tip);
                }
            }
        }

        Ok(new_tip)
    }
}
