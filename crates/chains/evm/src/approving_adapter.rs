use std::sync::Arc;

use async_trait::async_trait;
use ethers::{
    types::{Bytes, U256},
    utils::hex,
};
use paradapp_core::{
    btc::btc_service::derive_p2wpkh_address,
    consts::{
        supported_network_enum::SupportedNetwork,
        transaction_phase::TransactionPhase, transaction_type::TransactionType,
    },
    dependencies::context::CoreContext,
    supra::supra_service::{self, SupraNetwork},
    traits::{
        approving_adapter::ApprovingAdapter,
        chain_provider_adapter::{ChainProviderAdapter, TxIdFilter},
    },
};
use tracing::{error, info, warn};

use crate::{
    bindings::paradapp_convert::Conversion, dependencies::context::EvmContext,
};
use anyhow::{Result, anyhow};

pub struct EvmApprovingAdapter {
    pub ctx: Arc<EvmContext>,
    pub core_ctx: Arc<CoreContext>,
    pub chain_provider: Arc<dyn ChainProviderAdapter>,
}

#[async_trait]
impl ApprovingAdapter for EvmApprovingAdapter {
    async fn get_or_create_index_for_tx(&self, tx_id: U256) -> Result<u32> {
        let network: &str = self.ctx.cfg.network.string_identifier();
        let tx_id_str = tx_id.to_string();

        self.core_ctx
            .redis_storage
            .get_or_create_index_for_tx(network, &tx_id_str)
            .await
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
        let (idx, address, script) =
            derive_p2wpkh_address(xpub, index, self.core_ctx.btc_network)?;

        Ok((idx, address, script))
    }

    async fn handle_operator_closes_for_active(
        &self,
        tx_id: U256,
        conf_req: u64,
    ) -> Result<()> {
        // 1. Fetch conversion info
        let (conv, _phase): (Conversion, u8) =
            self.ctx.contract.get_conversion_with_phase(tx_id).call().await?;

        let now_sec = chrono::Utc::now().timestamp() as u64;

        // 2. Fetch window info
        let (
            headers_started,
            _start_height,
            last_height,
            deposit_end,
            proof_end,
            duty_expires_at,
        ) = self.ctx.contract.windows_for(tx_id).call().await?;

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
                let duty_active = duty_expires_at != U256::zero()
                    && now_sec <= duty_expires_at.as_u64();

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
                    match c_op
                        .timeout_no_deposit_nativeto_bitcoin(tx_id)
                        .send()
                        .await
                    {
                        Ok(pending) => {
                            info!(
                                tx_hash = ?pending.tx_hash(),
                                tx_id = %tx_id,
                                "timeout_no_deposit_nativeto_bitcoin tx sent"
                            );
                        },
                        Err(e) => {
                            warn!(
                                tx_id = %tx_id,
                                error = %e,
                                "Failed to send timeout_no_deposit_hba_rto_btc — retrying next cycle"
                            );
                            return Ok(());
                        },
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
            let duty_active = duty_expires_at != U256::zero()
                && now_sec <= duty_expires_at.as_u64();

            if window_over && duty_active {
                info!(
                    tx_id = %tx_id,
                    "[op] BTC→Native txId={} window over, closeNoBTC_BTCtoNative",
                    tx_id
                );

                let c_op = self.ctx.c_op.clone();

                // 1. Static call
                let call_static =
                    c_op.close_no_bitcoin_bitcoin_to_native(tx_id);
                call_static.call().await?;

                // 2. Send transaction non blocking
                match c_op
                    .close_no_bitcoin_bitcoin_to_native(tx_id)
                    .send()
                    .await
                {
                    Ok(pending) => {
                        info!(
                            tx_hash = ?pending.tx_hash(),
                            tx_id = %tx_id,
                            "close_no_bitcoin_bitcoin_to_native tx sent)"
                        );
                    },
                    Err(e) => {
                        warn!(
                            tx_id = %tx_id,
                            error = %e,
                            "Failed to send close_no_bitcoin_bitcoin_to_native — retrying next cycle"
                        );
                        return Ok(());
                    },
                }
            }
        }

        Ok(())
    }

    async fn discover_user_close_candidates(
        &self,
        to_tx_id: U256,
    ) -> Result<Vec<(U256, String)>> {
        use futures::try_join;

        let (op_expired_ids, user_expired_ids) = try_join!(
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::ANY,
                phase_filter: TransactionPhase::OPERATOR_DUTY_EXPIRED,
                to_tx_id,
                ..Default::default()
            }),
            self.chain_provider.get_tx_ids_by_filter(TxIdFilter {
                type_filter: TransactionType::ANY,
                phase_filter: TransactionPhase::USER_ACTION_EXPIRED,
                to_tx_id,
                ..Default::default()
            }),
        )?;

        info!(
            op_expired_count = op_expired_ids.len(),
            user_expired_count = user_expired_ids.len(),
            "Fetched expired transaction IDs from filters"
        );

        let mut seen = std::collections::HashSet::<U256>::new();
        let mut candidates: Vec<(U256, String)> = Vec::new();

        let contract = self.ctx.contract.clone();
        let c_op = self.ctx.c_op.clone();

        for tx_id in user_expired_ids {
            if !seen.insert(tx_id) {
                continue;
            }

            let (conv, _phase): (Conversion, u8) =
                contract.get_conversion_with_phase(tx_id).call().await?;

            if conv.is_native_to_bitcoin {
                if c_op
                    .timeout_no_deposit_nativeto_bitcoin(tx_id)
                    .call()
                    .await
                    .is_ok()
                {
                    info!(tx_id = %tx_id, method = "timeoutNoDeposit_NativeToBitcoin", "Candidate found");
                    candidates.push((
                        tx_id,
                        "timeoutNoDeposit_NativeToBitcoin".to_string(),
                    ));
                }
            } else if c_op
                .close_no_bitcoin_bitcoin_to_native(tx_id)
                .call()
                .await
                .is_ok()
            {
                info!(tx_id = %tx_id, method = "closeNoBitcoin_BitcoinToNative", "Candidate found");
                candidates.push((
                    tx_id,
                    "closeNoBitcoin_BitcoinToNative".to_string(),
                ));
            }
        }

        for tx_id in op_expired_ids {
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
                    info!(tx_id = %tx_id, method = "refundAfterNoProof_NativeTokentoBTC", "Candidate found");
                    candidates.push((
                        tx_id,
                        "refundAfterNoProof_NativeTokentoBTC".to_string(),
                    ));
                }
            } else if c_op
                .claim_native_after_operator_expired(tx_id)
                .call()
                .await
                .is_ok()
            {
                info!(tx_id = %tx_id, method = "claimNative_AfterOperatorExpired", "Candidate found");
                candidates.push((
                    tx_id,
                    "claimNative_AfterOperatorExpired".to_string(),
                ));
            }
        }

        info!(
            count = candidates.len(),
            "Discovered candidates (Direct User Expiry + Validated Operator Expiry)"
        );

        Ok(candidates)
    }

    async fn execute_user_closes(
        &self,
        candidates: Vec<(U256, &'static str)>,
    ) -> Result<()> {
        let c_op = self.ctx.c_op.clone();

        for (tx_id, kind) in candidates {
            match kind {
                "timeoutNoDeposit_NativeToBitcoin" => {
                    info!(tx_id = %tx_id, "[jump] User-close timeoutNoDeposit_NativeToBitcoin");

                    let can_execute = c_op
                        .timeout_no_deposit_nativeto_bitcoin(tx_id)
                        .call()
                        .await;
                    if let Err(e) = can_execute {
                        error!(tx_id = %tx_id, error = ?e, "Static call failed: timeoutNoDeposit_NativeToBitcoin");
                        continue;
                    }

                    match c_op
                        .timeout_no_deposit_nativeto_bitcoin(tx_id)
                        .send()
                        .await
                    {
                        Ok(pending) => {
                            let tx_hash = pending.tx_hash();
                            let _ = pending.await;
                            info!(tx_hash = ?tx_hash, tx_id = %tx_id, "timeoutNoDeposit_NativeToBitcoin tx mined");
                        },
                        Err(e) => {
                            warn!(tx_id = %tx_id, error = %e, "Failed to send timeoutNoDeposit_NativeToBitcoin")
                        },
                    }
                },

                "closeNoBitcoin_BitcoinToNative" => {
                    info!(tx_id = %tx_id, "[jump] User-close closeNoBitcoin_BitcoinToNative");

                    let can_execute = c_op
                        .close_no_bitcoin_bitcoin_to_native(tx_id)
                        .call()
                        .await;
                    if let Err(e) = can_execute {
                        error!(tx_id = %tx_id, error = ?e, "Static call failed: closeNoBitcoin_BitcoinToNative");
                        continue;
                    }

                    match c_op
                        .close_no_bitcoin_bitcoin_to_native(tx_id)
                        .send()
                        .await
                    {
                        Ok(pending) => {
                            let tx_hash = pending.tx_hash();
                            let _ = pending.await;
                            info!(tx_hash = ?tx_hash, tx_id = %tx_id, "closeNoBitcoin_BitcoinToNative tx mined");
                        },
                        Err(e) => {
                            warn!(tx_id = %tx_id, error = %e, "Failed to send closeNoBitcoin_BitcoinToNative")
                        },
                    }
                },

                "refundAfterNoProof_NativeTokentoBTC" => {
                    info!(tx_id = %tx_id, "[jump] User-close refundAfterNoProof_NativeTokentoBTC");

                    let can_execute = c_op
                        .refund_after_no_proof_native_to_bitcoin(tx_id)
                        .call()
                        .await;
                    if let Err(e) = can_execute {
                        error!(tx_id = %tx_id, error = ?e, "Static call failed: refundAfterNoProof_NativeTokentoBTC");
                        continue;
                    }

                    match c_op
                        .refund_after_no_proof_native_to_bitcoin(tx_id)
                        .send()
                        .await
                    {
                        Ok(pending) => {
                            let tx_hash = pending.tx_hash();
                            let _ = pending.await;
                            info!(tx_hash = ?tx_hash, tx_id = %tx_id, "refundAfterNoProof_NativeTokentoBTC tx mined");
                        },
                        Err(e) => {
                            warn!(tx_id = %tx_id, error = %e, "Failed to send refundAfterNoProof_NativeTokentoBTC")
                        },
                    }
                },

                "claimNative_AfterOperatorExpired" => {
                    info!(tx_id = %tx_id, "[jump] User-close claimNative_AfterOperatorExpired");

                    let can_execute = c_op
                        .claim_native_after_operator_expired(tx_id)
                        .call()
                        .await;
                    if let Err(e) = can_execute {
                        error!(tx_id = %tx_id, error = ?e, "Static call failed: claimNative_AfterOperatorExpired");
                        continue;
                    }

                    match c_op
                        .claim_native_after_operator_expired(tx_id)
                        .send()
                        .await
                    {
                        Ok(pending) => {
                            let tx_hash = pending.tx_hash();
                            let _ = pending.await;
                            info!(tx_hash = ?tx_hash, tx_id = %tx_id, "claimNative_AfterOperatorExpired tx mined");
                        },
                        Err(e) => {
                            warn!(tx_id = %tx_id, error = %e, "Failed to send claimNative_AfterOperatorExpired")
                        },
                    }
                },

                _ => continue,
            }
        }

        Ok(())
    }

    async fn approve_one_tx(
        &self,
        tx_id: U256,
        duty_seconds: u64,
    ) -> Result<()> {
        let contract = &self.ctx.contract.clone();
        let c_op = &self.ctx.c_op.clone();

        let conv = contract.conversions(tx_id).call().await?;
        let (
            _user,
            is_native_to_bitcoin,
            _user_program,
            _paradapp_receive_program,
            _network_address,
            network_id,
            native_amount,
            bitcoin_amount,
            _commit_fee,
            _reserved_native,
            _created_at,
            _approved_at,
            _deposited_at,
            _operator_duty_expires_at,
            _approved,
            _deposited,
            _completed,
            _refunded,
        ) = conv;

        let tolerance_multiplier = 0.005;

        // Determine Source Network from Config
        let source_network_name = self.ctx.cfg.network.string_identifier();
        let source_network = SupportedNetwork::from_str(source_network_name)
            .ok_or_else(|| {
                anyhow!(
                    "Unsupported operator source network: {}",
                    source_network_name
                )
            })?;

        let source_decimals = source_network.decimals();

        // Use source_network for price fetching to ensure we price what was deposited
        let market_ratio = match supra_service::get_token_price_vs_btc(
            &self.core_ctx,
            source_network,
            Some(SupraNetwork::Testnet),
        )
        .await
        {
            Ok(ratio) => ratio,
            Err(e) => {
                tracing::warn!(tx_id = %tx_id, error = %e, "Failed to fetch market ratio from Supra");
                return Ok(());
            },
        };

        let native_float =
            native_amount.to_string().parse::<f64>().unwrap_or(0.0)
                / 10f64.powi(source_decimals as i32);
        let btc_float = bitcoin_amount.as_u64() as f64 / 100_000_000.0;

        if is_native_to_bitcoin {
            let tx_ratio =
                if native_float > 0.0 { btc_float / native_float } else { 0.0 };
            let max_allowed_ratio = market_ratio * (1.0 + tolerance_multiplier);

            info!(
                tx_id = %tx_id,
                tx_ratio = tx_ratio,
                market_ratio = market_ratio,
                max_allowed_ratio = max_allowed_ratio,
                asset = %source_network,
                "Checking price for Native->BTC approval"
            );

            if tx_ratio > max_allowed_ratio {
                warn!(tx_id = %tx_id, tx_ratio = tx_ratio, "Price check failed: N2B ratio exceeds max payout");
                return Ok(());
            }
        } else {
            let tx_ratio =
                if btc_float > 0.0 { native_float / btc_float } else { 0.0 };
            let market_native_per_btc =
                if market_ratio > 0.0 { 1.0 / market_ratio } else { 0.0 };
            let max_allowed_ratio =
                market_native_per_btc * (1.0 + tolerance_multiplier);

            info!(
                tx_id = %tx_id,
                tx_ratio = tx_ratio,
                market_ratio = market_native_per_btc,
                max_allowed_ratio = max_allowed_ratio,
                asset = %source_network,
                "Checking price for BTC->Native approval"
            );

            if tx_ratio > max_allowed_ratio {
                warn!(tx_id = %tx_id, tx_ratio = tx_ratio, "Price check failed: B2N ratio exceeds max payout");
                return Ok(());
            }
        }

        let xpub_str: &str = self.ctx.cfg.btc_root_xpub.as_ref();

        let script_arg: Vec<u8> = match (
            is_native_to_bitcoin,
            network_id.is_zero(),
        ) {
            (false, _) | (true, false) => {
                match self
                    .get_or_create_receive_program_for_tx(tx_id, xpub_str)
                    .await
                {
                    Ok((index, address, script_buf)) => {
                        info!(tx_id = %tx_id, address = %address, index = index, "Assigned BTC addr");
                        script_buf
                    },
                    Err(err) => {
                        warn!(tx_id = %tx_id, error = %err, "Failed deriving BTC address");
                        return Ok(());
                    },
                }
            },
            (true, true) => {
                if let Some(static_program) =
                    &self.core_ctx.cfg.paradapp_receive_program
                {
                    hex::decode(static_program.trim_start_matches("0x"))
                        .unwrap_or_default()
                } else {
                    return Err(anyhow!(
                        "missing receive program for Native→BTC"
                    ));
                }
            },
        };

        let call = c_op.approve_and_start_with_anchor_and_first(
            tx_id,
            U256::from(duty_seconds),
            Bytes::from(script_arg),
        );

        if let Err(err) = call.clone().call().await {
            error!(tx_id = %tx_id, err = %err, "callStatic approve failed");
            return Ok(());
        }

        match call.send().await {
            Ok(pending) => {
                info!(tx_hash = ?pending.tx_hash(), tx_id = %tx_id, "Sent approve tx")
            },
            Err(e) => {
                warn!(tx_id = %tx_id, error = %e, "Failed to send approve tx")
            },
        }

        Ok(())
    }
}
