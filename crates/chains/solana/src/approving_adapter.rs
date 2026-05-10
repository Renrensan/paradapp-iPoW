use std::sync::Arc;

use anchor_lang::AccountDeserialize;
use anchor_lang::solana_program::system_program;
use async_trait::async_trait;
use ethers::{types::U256, utils::hex};
use paradapp_core::{
    btc::btc_service::derive_p2wpkh_address,
    consts::{
        transaction_phase::TransactionPhase, transaction_type::TransactionType,
    },
    dependencies::context::CoreContext,
    supra::supra_service::{self, SupraNetwork},
    traits::{
        approving_adapter::ApprovingAdapter,
        chain_provider_adapter::{ChainProviderAdapter, TxIdFilter},
    },
};
use solana_sdk::{
    hash::hash,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    transaction::Transaction,
};
use tracing::{error, info, warn};

use crate::{
    bindings::ipow_types::ipow_paradapp_solana::{
        ID,
        accounts::{Conversion, GlobalState},
        types::ConversionStatus,
    },
    dependencies::context::SolanaContext,
};
use anyhow::Result;

pub struct SolanaApprovingAdapter {
    pub ctx: Arc<SolanaContext>,
    pub core_ctx: Arc<CoreContext>,
    pub chain_provider: Arc<dyn ChainProviderAdapter>,
}

#[async_trait]
impl ApprovingAdapter for SolanaApprovingAdapter {
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
        let index = self.get_or_create_index_for_tx(tx_id).await?;
        let (idx, address, script) =
            derive_p2wpkh_address(xpub, index, self.core_ctx.btc_network)?;

        Ok((idx, address, script))
    }

    async fn handle_operator_closes_for_active(
        &self,
        tx_id: U256,
        conf_req: u64,
    ) -> Result<()> {
        let tx_id_u64 = tx_id.as_u64();
        let (conversion_pda, _) = Pubkey::find_program_address(
            &[b"conversion", &tx_id_u64.to_le_bytes()],
            &ID,
        );

        let account_data =
            match self.ctx.rpc_client.get_account_data(&conversion_pda).await {
                Ok(data) => data,
                Err(_) => return Ok(()),
            };

        let conv = Conversion::try_deserialize(&mut &account_data[..])?;

        if !conv.window_started {
            return Ok(());
        }

        let global_data = self
            .ctx
            .rpc_client
            .get_account_data(&self.ctx.global_state_pda)
            .await?;
        let global_state = GlobalState::try_deserialize(&mut &global_data[..])?;
        let last_height = global_state.global_tip_height;

        let now_sec = chrono::Utc::now().timestamp();

        let deposit_end = conv.window_start_height + 9; // DEPOSIT_BLOCKS_WINDOW (10)
        let proof_end = conv.window_start_height + 39; // PROOF_BLOCKS_WINDOW (40)

        if conv.is_native_to_bitcoin {
            // Native → BTC
            if !matches!(conv.status, ConversionStatus::Approved) {
                return Ok(());
            }

            let deposited = conv.deposited_at > 0;
            if !deposited {
                let deposit_over = last_height > deposit_end;
                let duty_active = conv.operator_duty_expires_at > 0
                    && now_sec <= conv.operator_duty_expires_at;

                if deposit_over && duty_active {
                    info!(
                        tx_id = %tx_id,
                        "[op] Native→BTC txId={} deposit timeout, timeoutNoDeposit_NativeTokentoBTC",
                        tx_id
                    );
                    warn!(tx_id = %tx_id, "timeout_no_deposit is missing from Solana program; skipping.");
                    // NOTE: If you add `timeout_no_deposit` back to your lib.rs, execute it here.
                }
            }
        } else {
            // BTC → Native (Operator handles CloseNoBitcoin)
            if !matches!(conv.status, ConversionStatus::Approved) {
                return Ok(());
            }

            let end_height = proof_end + conf_req.saturating_sub(1);
            let window_over = last_height > end_height;
            let duty_active = conv.operator_duty_expires_at > 0
                && now_sec <= conv.operator_duty_expires_at;

            if window_over && duty_active {
                info!(
                    tx_id = %tx_id,
                    "[op] BTC→Native txId={} window over, closeNoBTC_BTCtoNative",
                    tx_id
                );

                let discriminator =
                    hash(b"global:close_no_bitcoin_bitcoin_to_native")
                        .to_bytes()[..8]
                        .to_vec();

                let ix = Instruction {
                    program_id: ID,
                    accounts: vec![
                        AccountMeta::new(self.ctx.global_state_pda, false),
                        AccountMeta::new(conversion_pda, false),
                        AccountMeta::new(self.ctx.operator.pubkey(), true),
                    ],
                    data: discriminator,
                };

                let bh = self.ctx.rpc_client.get_latest_blockhash().await?;
                let tx = Transaction::new_signed_with_payer(
                    &[ix],
                    Some(&self.ctx.operator.pubkey()),
                    &[&*self.ctx.operator],
                    bh,
                );

                match self
                    .ctx
                    .rpc_client
                    .send_and_confirm_transaction(&tx)
                    .await
                {
                    Ok(sig) => {
                        info!(tx_hash = ?sig, tx_id = %tx_id, "close_no_bitcoin_bitcoin_to_native tx mined")
                    },
                    Err(e) => {
                        warn!(tx_id = %tx_id, error = %e, "Failed to send close_no_bitcoin_bitcoin_to_native")
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

        let mut seen = std::collections::HashSet::<U256>::new();
        let mut candidates: Vec<(U256, String)> = Vec::new();

        // 1. User Expired Scenarios
        for tx_id in user_expired_ids {
            if !seen.insert(tx_id) {
                continue;
            }

            let tx_id_u64 = tx_id.as_u64();
            let (conversion_pda, _) = Pubkey::find_program_address(
                &[b"conversion", &tx_id_u64.to_le_bytes()],
                &ID,
            );

            if let Ok(account_data) =
                self.ctx.rpc_client.get_account_data(&conversion_pda).await
            {
                if let Ok(conv) =
                    Conversion::try_deserialize(&mut &account_data[..])
                {
                    if conv.is_native_to_bitcoin
                        && matches!(conv.status, ConversionStatus::Approved)
                    {
                        candidates.push((
                            tx_id,
                            "timeoutNoDeposit_NativeToBitcoin".to_string(),
                        ));
                    } else if !conv.is_native_to_bitcoin
                        && matches!(conv.status, ConversionStatus::Approved)
                    {
                        candidates.push((
                            tx_id,
                            "closeNoBitcoin_BitcoinToNative".to_string(),
                        ));
                    }
                }
            }
        }

        // 2. Operator Expired Scenarios
        for tx_id in op_expired_ids {
            if !seen.insert(tx_id) {
                continue;
            }

            let tx_id_u64 = tx_id.as_u64();
            let (conversion_pda, _) = Pubkey::find_program_address(
                &[b"conversion", &tx_id_u64.to_le_bytes()],
                &ID,
            );

            if let Ok(account_data) =
                self.ctx.rpc_client.get_account_data(&conversion_pda).await
            {
                if let Ok(conv) =
                    Conversion::try_deserialize(&mut &account_data[..])
                {
                    if matches!(
                        conv.status,
                        ConversionStatus::Completed
                            | ConversionStatus::Refunded
                    ) {
                        continue;
                    }

                    if conv.is_native_to_bitcoin {
                        if matches!(conv.status, ConversionStatus::Deposited) {
                            candidates.push((
                                tx_id,
                                "refundAfterNoProof_NativeTokentoBTC"
                                    .to_string(),
                            ));
                        }
                    } else if matches!(
                        conv.status,
                        ConversionStatus::Approved
                            | ConversionStatus::Deposited
                    ) {
                        candidates.push((
                            tx_id,
                            "claimNative_AfterOperatorExpired".to_string(),
                        ));
                    }
                }
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
        for (tx_id, kind) in candidates {
            let tx_id_u64 = tx_id.as_u64();
            let (conversion_pda, _) = Pubkey::find_program_address(
                &[b"conversion", &tx_id_u64.to_le_bytes()],
                &ID,
            );
            let (escrow_pda, _) =
                Pubkey::find_program_address(&[b"escrow"], &ID);

            // Get user pubkey for the instructions that require it
            let account_data = match self
                .ctx
                .rpc_client
                .get_account_data(&conversion_pda)
                .await
            {
                Ok(data) => data,
                Err(_) => continue,
            };
            let conv = match Conversion::try_deserialize(&mut &account_data[..])
            {
                Ok(c) => c,
                Err(_) => continue,
            };

            match kind {
                "timeoutNoDeposit_NativeToBitcoin" => {
                    info!(tx_id = %tx_id, "[jump] User-close timeoutNoDeposit_NativeToBitcoin");
                    let _ =
                        self.handle_operator_closes_for_active(tx_id, 1).await;
                },
                "closeNoBitcoin_BitcoinToNative" => {
                    info!(tx_id = %tx_id, "[jump] User-close closeNoBitcoin_BitcoinToNative");
                    let _ =
                        self.handle_operator_closes_for_active(tx_id, 1).await;
                },
                "refundAfterNoProof_NativeTokentoBTC" => {
                    info!(tx_id = %tx_id, "[jump] User-close refundAfterNoProof_NativeTokentoBTC");

                    let discriminator =
                        hash(b"global:refund_after_no_proof_native_to_bitcoin")
                            .to_bytes()[..8]
                            .to_vec();

                    let ix = Instruction {
                        program_id: ID,
                        accounts: vec![
                            AccountMeta::new(self.ctx.global_state_pda, false),
                            AccountMeta::new(escrow_pda, false),
                            AccountMeta::new(conversion_pda, false),
                            AccountMeta::new(conv.user, false), // Unchecked User
                            AccountMeta::new(self.ctx.operator.pubkey(), true), // Payer (Operator Bot)
                            AccountMeta::new_readonly(
                                system_program::id(),
                                false,
                            ),
                        ],
                        data: discriminator,
                    };

                    let bh = self.ctx.rpc_client.get_latest_blockhash().await?;
                    let tx = Transaction::new_signed_with_payer(
                        &[ix],
                        Some(&self.ctx.operator.pubkey()),
                        &[&*self.ctx.operator],
                        bh,
                    );

                    match self
                        .ctx
                        .rpc_client
                        .send_and_confirm_transaction(&tx)
                        .await
                    {
                        Ok(sig) => {
                            info!(tx_hash = ?sig, tx_id = %tx_id, "refundAfterNoProof_NativeTokentoBTC tx mined")
                        },
                        Err(e) => {
                            warn!(tx_id = %tx_id, error = %e, "Failed to send refundAfterNoProof_NativeTokentoBTC")
                        },
                    }
                },
                "claimNative_AfterOperatorExpired" => {
                    info!(tx_id = %tx_id, "[jump] User-close claimNative_AfterOperatorExpired");

                    let discriminator =
                        hash(b"global:claim_native_after_operator_expired")
                            .to_bytes()[..8]
                            .to_vec();

                    let ix = Instruction {
                        program_id: ID,
                        accounts: vec![
                            AccountMeta::new(self.ctx.global_state_pda, false),
                            AccountMeta::new(escrow_pda, false),
                            AccountMeta::new(conversion_pda, false),
                            AccountMeta::new(conv.user, false), // Unchecked User
                            AccountMeta::new(self.ctx.operator.pubkey(), true), // Payer (Operator Bot)
                            AccountMeta::new_readonly(
                                system_program::id(),
                                false,
                            ),
                        ],
                        data: discriminator,
                    };

                    let bh = self.ctx.rpc_client.get_latest_blockhash().await?;
                    let tx = Transaction::new_signed_with_payer(
                        &[ix],
                        Some(&self.ctx.operator.pubkey()),
                        &[&*self.ctx.operator],
                        bh,
                    );

                    match self
                        .ctx
                        .rpc_client
                        .send_and_confirm_transaction(&tx)
                        .await
                    {
                        Ok(sig) => {
                            info!(tx_hash = ?sig, tx_id = %tx_id, "claimNative_AfterOperatorExpired tx mined")
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
        let tx_id_u64 = tx_id.as_u64();
        let (conversion_pda, _) = Pubkey::find_program_address(
            &[b"conversion", &tx_id_u64.to_le_bytes()],
            &ID,
        );

        let account_data =
            self.ctx.rpc_client.get_account_data(&conversion_pda).await?;

        // Make sure you have imported Conversion as SolConversion at the top
        let conv = Conversion::try_deserialize(&mut &account_data[..])?;

        let tolerance_multiplier = 0.005;

        let source_network = self.chain_provider.network();
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
            conv.native_amount.to_string().parse::<f64>().unwrap_or(0.0)
                / 10f64.powi(source_decimals as i32);
        let btc_float = conv.bitcoin_amount as f64 / 100_000_000.0;

        if conv.is_native_to_bitcoin {
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
            conv.is_native_to_bitcoin,
            conv.network_id == 0,
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
                    return Err(anyhow::anyhow!(
                        "missing receive program for Native→BTC"
                    ));
                }
            },
        };

        // Construct `approve_and_start_with_anchor` instruction dynamically
        let mut data =
            solana_sdk::hash::hash(b"global:approve_and_start_with_anchor")
                .to_bytes()[..8]
                .to_vec();
        data.extend_from_slice(&(duty_seconds as i64).to_le_bytes());
        data.extend_from_slice(&(script_arg.len() as u32).to_le_bytes()); // Borsh Vector prefix
        data.extend_from_slice(&script_arg);

        let ix = Instruction {
            program_id: ID,
            accounts: vec![
                AccountMeta::new(self.ctx.global_state_pda, false),
                AccountMeta::new(conversion_pda, false),
                AccountMeta::new(self.ctx.operator.pubkey(), true),
            ],
            data,
        };

        let bh = self.ctx.rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.ctx.operator.pubkey()),
            &[&*self.ctx.operator],
            bh,
        );

        match self.ctx.rpc_client.send_and_confirm_transaction(&tx).await {
            Ok(sig) => {
                info!(tx_hash = ?sig, tx_id = %tx_id, "Sent approve tx");
                Ok(())
            },
            Err(e) => {
                error!(tx_id = %tx_id, error = %e, "Failed to send approve tx");
                Err(anyhow::anyhow!("Failed to approve transaction: {}", e))
            },
        }
    }
}
