use crate::{
    common::helpers::preflight::preflight_commit_global, dependencies::context::EvmContext,
};
use anyhow::Result;
use async_trait::async_trait;
use ethers::{abi::Address, types::U256};
use futures::try_join;
use paradapp_core::{
    btc::btc::{btc_tip_height, check_work_le, decode_header80, header80_by_height},
    consts::{transaction_phase::TransactionPhase, transaction_type::TransactionType},
    context::CoreContext,
    traits::streaming::{StreamTarget, StreamingAdapter},
};
use std::{collections::HashSet, sync::Arc, thread::sleep, time::Duration};
use tracing::info;

pub struct EvmStreamingAdapter {
    pub ctx: Arc<EvmContext>,
    pub core_ctx: Arc<CoreContext>,
}

#[async_trait]
impl StreamingAdapter for EvmStreamingAdapter {
    async fn push_headers_global(
        &self,
        target_height_plus: u64,
        tx_ids_to_check: Vec<U256>,
    ) -> Result<()> {
        // 1. Get BTC tip
        let btc_tip = btc_tip_height(&self.core_ctx).await? as u64;
        let effective_target = target_height_plus.min(btc_tip);

        if effective_target == 0 {
            info!("⚠️  [GLOBAL] effectiveTarget is 0, skipping.");
            return Ok(());
        }

        let ids_str = tx_ids_to_check
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        info!(
            effective_target = %effective_target,
            btc_tip = %btc_tip,
            tx_ids = %ids_str,
            "⛓️  [GLOBAL] streaming until height ≤ {} (BTC tip={}) for txIds=[{}]",
            effective_target,
            btc_tip,
            ids_str,
        );

        let mut pushed = 0u64;

        // 2. Loop streaming headers
        loop {
            let tip_bn: U256 = self.ctx.contract.global_tip_height().call().await?;

            let tip = tip_bn.as_u64();
            let next_height = if tip == 0 { 1 } else { tip + 1 };

            if next_height > effective_target {
                info!(
                    %next_height,
                    %effective_target,
                    "Stopping Streaming: next_height passed effective_target"
                );
                break;
            }

            // 3. Fetch header80
            let (_hash, header80) = header80_by_height(&self.core_ctx, next_height).await?;

            // 4. Proof-of-work check
            let (ok, bits, _target, _h_val) = check_work_le(&header80)?;
            if !ok {
                return Err(anyhow::anyhow!(
                    "Header at height {} low-work (bits=0x{:x})",
                    next_height,
                    bits
                ));
            }

            // 5. Preflight / callStatic check
            let header80_bytes = decode_header80(&header80).map_err(|e| {
                anyhow::anyhow!("failed to decode header80 at height {next_height}: {e}")
            })?;
            let pf = preflight_commit_global(&self.ctx, header80_bytes.clone(), next_height).await;

            if !pf.static_ok {
                if let Some(err_str) = pf.static_err {
                    let reason = err_str.to_lowercase();

                    if reason.contains("height-rewrite") {
                        info!(height = next_height, "height already stored, skipping.");
                        continue;
                    }

                    info!(reason = %reason, "commitGlobalBTCHeader80 would revert");
                    return Err(anyhow::anyhow!(reason));
                }
            }

            // 6. Actual transaction send
            let c_op = self.ctx.c_op.clone();
            let gas = U256::from(1_200_000u64);
            let call = c_op
                .commit_global_bitcoin_header_80(
                    header80_bytes,
                    U256::from(next_height),
                    tx_ids_to_check.clone(),
                )
                .gas(gas);

            let pending = call.send().await?;
            let _receipt = pending.await?;

            pushed += 1;

            info!(height = next_height, " ↳ stored header height");

            sleep(Duration::from_millis(150));
        }

        info!(
            pushed = pushed,
            effective_target = effective_target,
            "✅ [GLOBAL] streamed headers"
        );

        Ok(())
    }

    async fn get_active_tx_ids(&self, max_results: u64) -> Result<Vec<U256>> {
        let contract = &self.ctx.contract;

        // 1. nextTxId()
        let next_tx_id: U256 = contract.next_tx_id().call().await?;
        if next_tx_id <= U256::one() {
            return Ok(vec![]);
        }
        let to_tx_id = next_tx_id - U256::one();

        // 2. getTxIdsByFilter()
        let from = U256::one();
        let max_results_u256 = U256::from(max_results);

        let native_to_btc_fut = contract.get_tx_ids_by_filter(
            TransactionType::NATIVE_TO_BITCOIN,
            TransactionPhase::WAITING_USER_ACTION,
            Address::zero(),
            from,
            to_tx_id,
            max_results_u256,
        );

        let btc_to_native_fut = contract.get_tx_ids_by_filter(
            TransactionType::BITCOIN_TO_NATIVE,
            TransactionPhase::WAITING_USER_ACTION,
            Address::zero(),
            from,
            to_tx_id,
            max_results_u256,
        );

        let (native_to_btc, btc_to_native) =
            try_join!(native_to_btc_fut.call(), btc_to_native_fut.call(),)?;

        let tx_ids_waiting_user_action: Vec<U256> =
            native_to_btc.into_iter().chain(btc_to_native).collect();

        let native_to_btc_call = contract.get_tx_ids_by_filter(
            TransactionType::NATIVE_TO_BITCOIN,
            TransactionPhase::WAITING_USER_ACTION,
            Address::zero(),
            from,
            to_tx_id,
            max_results_u256,
        );

        let btc_to_native_call = contract.get_tx_ids_by_filter(
            TransactionType::BITCOIN_TO_NATIVE,
            TransactionPhase::WAITING_USER_ACTION,
            Address::zero(),
            from,
            to_tx_id,
            max_results_u256,
        );

        let (native_to_btc, btc_to_native) =
            try_join!(native_to_btc_call.call(), btc_to_native_call.call(),)?;

        let tx_ids_active_waiting_proof: Vec<U256> =
            native_to_btc.into_iter().chain(btc_to_native).collect();

        let mut set: HashSet<U256> = tx_ids_waiting_user_action.into_iter().collect();
        set.extend(tx_ids_active_waiting_proof.into_iter());

        let tx_ids: Vec<U256> = set.into_iter().collect();

        Ok(tx_ids)
    }

    async fn compute_stream_target(&self, tx_id: U256) -> Result<StreamTarget> {
        let c = &self.ctx.contract;

        // 1. Load conversion
        let conv = c.conversions(tx_id).call().await?;

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
            deposited,
            completed,
            refunded,
            _reserved_native,
            _operator_duty_expires_at,
        ) = conv;

        // 2. windowsFor(txId)
        let (headers_started, _start_height, last_height, deposit_end, proof_end, _duty_expires_at) =
            c.windows_for(tx_id).call().await?;

        if !headers_started {
            return Ok(StreamTarget {
                needed: false,
                target_height: 0,
                reason: "headers-not-started".into(),
            });
        }

        if completed || refunded {
            return Ok(StreamTarget {
                needed: false,
                target_height: 0,
                reason: "closed".into(),
            });
        }

        let target = if is_native_to_bitcoin && !deposited {
            deposit_end
        } else {
            proof_end
        };

        if target == U256::zero() {
            return Ok(StreamTarget {
                needed: false,
                target_height: 0,
                reason: "no-window".into(),
            });
        }

        if last_height >= target {
            return Ok(StreamTarget {
                needed: false,
                target_height: 0,
                reason: "already-past-target".into(),
            });
        }

        let target_plus = target + 2;

        info!(
            tx_id = %tx_id,
            last_height = %last_height,
            target = %target,
            tx_type = %if is_native_to_bitcoin { "NATIVE→BTC" } else { "BTC→NATIVE" },
            "tx needs headers"
        );

        Ok(StreamTarget {
            needed: true,
            target_height: target_plus.as_u64(),
            reason: "ok".into(),
        })
    }
}
