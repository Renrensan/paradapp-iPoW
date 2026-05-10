use anchor_lang::AccountDeserialize;
use anchor_lang::solana_program::system_program;
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use ethers::types::U256;
use paradapp_core::{
    btc::btc_service::{
        btc_tip_height, check_work_le, decode_header80, epoch_start,
        header80_by_height,
    },
    dependencies::context::CoreContext,
    traits::{
        chain_provider_adapter::ChainProviderAdapter,
        streaming_adapter::{StreamTarget, StreamingAdapter},
    },
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    transaction::Transaction,
};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

use crate::bindings::ipow_types::ipow_paradapp_solana::{
    ID,
    accounts::{Conversion, GlobalState},
    types::ConversionStatus,
};
use crate::dependencies::context::SolanaContext;

pub struct SolanaStreamingAdapter {
    pub ctx: Arc<SolanaContext>,
    pub core_ctx: Arc<CoreContext>,
    pub chain_provider: Arc<dyn ChainProviderAdapter>,
}

impl SolanaStreamingAdapter {
    async fn attempt_finalize_cached_proof(&self, tx_id: U256) -> Result<()> {
        let tx_id_u64 = tx_id.as_u64();
        let (conversion_pda, _) = Pubkey::find_program_address(
            &[b"conversion", &tx_id_u64.to_le_bytes()],
            &ID,
        );
        let (proof_cache_pda, _) = Pubkey::find_program_address(
            &[b"proof", &tx_id_u64.to_le_bytes()],
            &ID,
        );

        let pc_data = match self
            .ctx
            .rpc_client
            .get_account_data(&proof_cache_pda)
            .await
        {
            Ok(d) => d,
            Err(_) => return Ok(()),
        };
        if pc_data.len() < 60 {
            return Ok(());
        }

        let is_set = pc_data[16] != 0;
        let is_verified = pc_data[17] != 0;
        let is_invalid = pc_data[18] != 0;
        if !is_set || is_verified || is_invalid {
            return Ok(());
        }

        let mut txid_le = [0u8; 32];
        txid_le.copy_from_slice(&pc_data[20..52]);

        let mut pb_height_bytes = [0u8; 8];
        pb_height_bytes.copy_from_slice(&pc_data[52..60]);
        let pb_height = u64::from_le_bytes(pb_height_bytes);

        let (header_pda, _) = Pubkey::find_program_address(
            &[b"header", &pb_height.to_le_bytes()],
            &ID,
        );
        if self.ctx.rpc_client.get_account_data(&header_pda).await.is_err() {
            return Ok(());
        }

        let conv_data =
            match self.ctx.rpc_client.get_account_data(&conversion_pda).await {
                Ok(d) => d,
                Err(_) => return Ok(()),
            };
        let conv = match Conversion::try_deserialize(&mut &conv_data[..]) {
            Ok(c) => c,
            Err(_) => return Ok(()),
        };

        let (global_state_pda, _) =
            Pubkey::find_program_address(&[b"global_state"], &ID);
        let (escrow_vault_pda, _) =
            Pubkey::find_program_address(&[b"escrow"], &ID);
        let (height_tracker_pda, _) = Pubkey::find_program_address(
            &[b"tracker", &pb_height.to_le_bytes()],
            &ID,
        );
        let (used_proof_pda, _) =
            Pubkey::find_program_address(&[b"used_proof", &txid_le], &ID);

        let data = solana_sdk::hash::hash(b"global:try_finalize_cached_proof")
            .to_bytes()[..8]
            .to_vec();

        let ix = Instruction {
            program_id: ID,
            accounts: vec![
                AccountMeta::new(global_state_pda, false),
                AccountMeta::new(escrow_vault_pda, false),
                AccountMeta::new(conversion_pda, false),
                AccountMeta::new(proof_cache_pda, false),
                AccountMeta::new(height_tracker_pda, false),
                AccountMeta::new_readonly(header_pda, false),
                AccountMeta::new(used_proof_pda, false),
                AccountMeta::new(self.ctx.operator.pubkey(), false),
                AccountMeta::new(conv.user, false),
                AccountMeta::new(self.ctx.operator.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data,
        };

        let compute_budget_program_id = std::str::FromStr::from_str(
            "ComputeBudget111111111111111111111111111111",
        )
        .unwrap();
        let mut compute_data = vec![2u8];
        compute_data.extend_from_slice(&1_000_000u32.to_le_bytes());
        let compute_budget_ix = Instruction {
            program_id: compute_budget_program_id,
            accounts: vec![],
            data: compute_data,
        };

        let bh = self.ctx.rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[compute_budget_ix, ix],
            Some(&self.ctx.operator.pubkey()),
            &[&*self.ctx.operator],
            bh,
        );

        let _ = self.ctx.rpc_client.send_and_confirm_transaction(&tx).await;
        Ok(())
    }
}

#[async_trait]
impl StreamingAdapter for SolanaStreamingAdapter {
    async fn push_headers_global(
        &self,
        target_height_plus: u64,
        tx_ids_to_check: Vec<U256>,
    ) -> Result<()> {
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

        loop {
            for tx_id in &tx_ids_to_check {
                let _ = self.attempt_finalize_cached_proof(*tx_id).await;
            }

            let account_data = self
                .ctx
                .rpc_client
                .get_account_data(&self.ctx.global_state_pda)
                .await
                .map_err(|e| anyhow!("{e}"))?;

            let global_state =
                GlobalState::try_deserialize(&mut &account_data[..])?;
            let tip = global_state.global_tip_height;

            let next_height = if tip == 0 { 1 } else { tip + 1 };

            if next_height > effective_target {
                info!(
                    %next_height,
                    %effective_target,
                    "Stopping Streaming: next_height passed effective_target"
                );
                break;
            }

            let (_hash, header80) =
                header80_by_height(&self.core_ctx, next_height).await?;

            let (ok, bits, _, _) =
                check_work_le(&header80).map_err(|e| anyhow!("{e}"))?;
            if !ok {
                return Err(anyhow!(
                    "Header at height {} low-work (bits=0x{:x})",
                    next_height,
                    bits
                ));
            }

            let header80_bytes =
                decode_header80(&header80).map_err(|e| anyhow!("{e}"))?;

            let (header_pda, _) = Pubkey::find_program_address(
                &[b"header", &next_height.to_le_bytes()],
                &ID,
            );

            if self.ctx.rpc_client.get_account(&header_pda).await.is_ok() {
                info!(height = next_height, "height already stored, skipping.");
                // Note: We break instead of continue to prevent an infinite loop
                // in case the global tip hasn't been updated yet.
                break;
            }

            let prev_height = if next_height > 0 { next_height - 1 } else { 0 };
            let (prev_height_tracker_pda, _) = Pubkey::find_program_address(
                &[b"tracker", &prev_height.to_le_bytes()],
                &ID,
            );

            let is_retarget =
                next_height % 2016 == 0 && tip != 0 && next_height == tip + 1;

            let prev_start_pda = if is_retarget {
                Pubkey::find_program_address(
                    &[b"header", &epoch_start(next_height).to_le_bytes()],
                    &ID,
                )
                .0
            } else {
                ID
            };

            let prev_end_pda = if is_retarget {
                Pubkey::find_program_address(
                    &[b"header", &(next_height - 1).to_le_bytes()],
                    &ID,
                )
                .0
            } else {
                ID
            };

            let mut data = vec![7, 35, 38, 214, 91, 62, 171, 36];
            let mut header_array = [0u8; 80];
            header_array.copy_from_slice(&header80_bytes[..80]);
            data.extend_from_slice(&header_array);
            data.extend_from_slice(&next_height.to_le_bytes());

            let ix = Instruction {
                program_id: ID,
                accounts: vec![
                    AccountMeta::new(self.ctx.global_state_pda, false),
                    AccountMeta::new(header_pda, false),
                    AccountMeta::new_readonly(prev_height_tracker_pda, false),
                    AccountMeta::new_readonly(prev_start_pda, false),
                    AccountMeta::new_readonly(prev_end_pda, false),
                    AccountMeta::new(self.ctx.operator.pubkey(), true),
                    AccountMeta::new_readonly(system_program::id(), false),
                ],
                data,
            };

            let bh = self
                .ctx
                .rpc_client
                .get_latest_blockhash()
                .await
                .map_err(|e| anyhow!("{e}"))?;

            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&self.ctx.operator.pubkey()),
                &[&*self.ctx.operator],
                bh,
            );

            match self.ctx.rpc_client.send_and_confirm_transaction(&tx).await {
                Ok(_) => {
                    pushed += 1;
                    info!(height = next_height, " ↳ stored header height");
                },
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("6011") {
                        info!(
                            height = next_height,
                            "height already stored, skipping."
                        );
                        continue;
                    }
                    error!(height = next_height, error = %err_str, "commitGlobalBTCHeader80 failed");
                    return Err(anyhow!(err_str));
                },
            }

            sleep(Duration::from_millis(150)).await;
        }

        info!(
            pushed = pushed,
            effective_target = effective_target,
            "[GLOBAL] streamed headers"
        );

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

        for h in start..=end {
            let (_hash, header80) =
                header80_by_height(&self.core_ctx, h).await.with_context(
                    || format!("Failed to fetch header80 for height {h}"),
                )?;

            let header80_bytes = decode_header80(&header80).map_err(|e| {
                anyhow!("failed to decode header80 at height {h}: {e}")
            })?;

            let (header_pda, _) = Pubkey::find_program_address(
                &[b"header", &h.to_le_bytes()],
                &ID,
            );

            if self.ctx.rpc_client.get_account(&header_pda).await.is_ok() {
                info!(height = %h, "height already stored, skipping");
                new_tip = h;
                continue;
            }

            let prev_height = if h > 0 { h - 1 } else { 0 };
            let (prev_height_tracker_pda, _) = Pubkey::find_program_address(
                &[b"tracker", &prev_height.to_le_bytes()],
                &ID,
            );

            let is_retarget =
                h % 2016 == 0 && current_tip != 0 && h == current_tip + 1;

            let prev_start_pda = if is_retarget {
                Pubkey::find_program_address(
                    &[b"header", &epoch_start(h).to_le_bytes()],
                    &ID,
                )
                .0
            } else {
                ID
            };

            let prev_end_pda = if is_retarget {
                Pubkey::find_program_address(
                    &[b"header", &(h - 1).to_le_bytes()],
                    &ID,
                )
                .0
            } else {
                ID
            };

            let mut data = vec![7, 35, 38, 214, 91, 62, 171, 36];
            let mut header_array = [0u8; 80];
            header_array.copy_from_slice(&header80_bytes[..80]);
            data.extend_from_slice(&header_array);
            data.extend_from_slice(&h.to_le_bytes());

            let ix = Instruction {
                program_id: ID,
                accounts: vec![
                    AccountMeta::new(self.ctx.global_state_pda, false),
                    AccountMeta::new(header_pda, false),
                    AccountMeta::new_readonly(prev_height_tracker_pda, false),
                    AccountMeta::new_readonly(prev_start_pda, false),
                    AccountMeta::new_readonly(prev_end_pda, false),
                    AccountMeta::new(self.ctx.operator.pubkey(), true),
                    AccountMeta::new_readonly(system_program::id(), false),
                ],
                data,
            };

            let bh = self
                .ctx
                .rpc_client
                .get_latest_blockhash()
                .await
                .map_err(|e| anyhow!("{e}"))?;

            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&self.ctx.operator.pubkey()),
                &[&*self.ctx.operator],
                bh,
            );

            match self.ctx.rpc_client.send_and_confirm_transaction(&tx).await {
                Ok(_) => {
                    info!(height = %h, "Global header stored");
                    new_tip = h;
                },
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("6011") {
                        info!(height = %h, "height already stored, skipping");
                        new_tip = h;
                        continue;
                    } else if err_str.contains("no-jump-while-active") {
                        warn!(height = %h, "no-jump-while-active, stopping stream");
                        return Ok(new_tip);
                    } else {
                        error!(height = %h, error = %err_str, "commitGlobalBTCHeader80 failed");
                        return Ok(new_tip);
                    }
                },
            }
        }

        Ok(new_tip)
    }

    async fn compute_stream_target(&self, tx_id: U256) -> Result<StreamTarget> {
        let tx_id_u64 = tx_id.as_u64();
        let (conversion_pda, _) = Pubkey::find_program_address(
            &[b"conversion", &tx_id_u64.to_le_bytes()],
            &ID,
        );

        let account_data =
            match self.ctx.rpc_client.get_account_data(&conversion_pda).await {
                Ok(data) => data,
                Err(_) => {
                    return Ok(StreamTarget {
                        needed: false,
                        target_height: 0,
                        reason: "headers-not-started".into(),
                    });
                },
            };

        let conv = Conversion::try_deserialize(&mut &account_data[..])?;

        if !conv.window_started {
            return Ok(StreamTarget {
                needed: false,
                target_height: 0,
                reason: "headers-not-started".into(),
            });
        }

        if matches!(
            conv.status,
            ConversionStatus::Completed | ConversionStatus::Refunded
        ) {
            return Ok(StreamTarget {
                needed: false,
                target_height: 0,
                reason: "closed".into(),
            });
        }

        let deposited = conv.deposited_at > 0;
        let deposit_end = conv.window_start_height + 10;
        let proof_end = conv.window_start_height + 40;

        let target = if conv.is_native_to_bitcoin && !deposited {
            deposit_end + 1
        } else {
            proof_end + 1
        };

        if target == 0 {
            return Ok(StreamTarget {
                needed: false,
                target_height: 0,
                reason: "no-window".into(),
            });
        }

        let account_data_global = self
            .ctx
            .rpc_client
            .get_account_data(&self.ctx.global_state_pda)
            .await?;
        let global_state =
            GlobalState::try_deserialize(&mut &account_data_global[..])?;
        let last_height = global_state.global_tip_height;

        if last_height >= target {
            return Ok(StreamTarget {
                needed: false,
                target_height: 0,
                reason: "already-past-target".into(),
            });
        }

        info!(
            tx_id = %tx_id,
            last_height = %last_height,
            target = %target,
            tx_type = %if conv.is_native_to_bitcoin { "NATIVE→BTC" } else { "BTC→NATIVE" },
            "tx needs headers"
        );

        Ok(StreamTarget {
            needed: true,
            target_height: target,
            reason: "ok".into(),
        })
    }
}
