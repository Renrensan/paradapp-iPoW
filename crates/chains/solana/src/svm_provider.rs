use std::sync::Arc;

use crate::{
    bindings::ipow_types::ipow_paradapp_solana::{
        ID,
        accounts::{Conversion as SolanaConversion, GlobalState},
        types::ConversionStatus,
    },
    dependencies::context::SolanaContext,
};
use anchor_lang::AccountDeserialize;
use anchor_lang::solana_program::{system_instruction, system_program};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use ethers::types::{Bytes, H160, U256};
use paradapp_core::{
    btc::btc_service::{
        btc_tip_height, decode_header80, epoch_start, header80_by_height,
        sweep_btc_to_main,
    },
    consts::{
        supported_network_enum::SupportedNetwork,
        transaction_phase::TransactionPhase, transaction_type::TransactionType,
    },
    dependencies::context::CoreContext,
    models::conversion::Conversion,
    traits::chain_provider_adapter::{
        AnchorInfo, BitcoinProgramType, ChainProviderAdapter, GlobalChainState,
        SubmittedProofInfo, TxIdFilter,
    },
};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    transaction::Transaction,
};

pub struct SvmChainProvider {
    pub ctx: Arc<SolanaContext>,
    pub core_ctx: Arc<CoreContext>,
}

impl SvmChainProvider {
    pub fn new(ctx: Arc<SolanaContext>, core_ctx: Arc<CoreContext>) -> Self {
        Self { ctx, core_ctx }
    }

    async fn get_global_state(&self) -> Result<GlobalState> {
        let account_data = self
            .ctx
            .rpc_client
            .get_account_data(&self.ctx.global_state_pda)
            .await
            .map_err(|e| anyhow!("{e}"))?;

        GlobalState::try_deserialize(&mut &account_data[..])
            .map_err(|e| anyhow!("Failed to deserialize GlobalState: {e}"))
    }
}

#[async_trait]
impl ChainProviderAdapter for SvmChainProvider {
    fn network(&self) -> SupportedNetwork {
        self.ctx.cfg.network.into()
    }

    fn min_transaction_limit(&self) -> u64 {
        self.ctx.cfg.min_transaction_limit
    }

    fn max_transaction_limit(&self) -> u64 {
        self.ctx.cfg.max_transaction_limit
    }

    async fn check_rpc_health(&self) -> Result<()> {
        let _permit = self
            .core_ctx
            .rpc_limiter
            .acquire()
            .await
            .map_err(|e| anyhow!("Failed to acquire RPC permit: {}", e))?;

        let svm_ok = self.ctx.rpc_client.get_version().await.is_ok();
        let btc_ok = true;

        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

        if !svm_ok || !btc_ok {
            anyhow::bail!("Upstream RPCs are down");
        }
        Ok(())
    }

    async fn trigger_btc_sweep(&self) -> Result<()> {
        let network = self.ctx.cfg.network.string_identifier();
        let start =
            self.core_ctx.redis_storage.get_last_swept_index(network).await?;
        let end = self
            .core_ctx
            .redis_storage
            .get_next_derivation_index(network)
            .await?;

        if end <= start {
            return Ok(());
        }

        let (_, actual_end_idx) = sweep_btc_to_main(
            &self.core_ctx,
            &self.ctx.cfg.btc_mnemonic,
            start,
            end,
        )
        .await?;

        self.core_ctx
            .redis_storage
            .set_last_swept_index(network, actual_end_idx)
            .await?;
        Ok(())
    }

    async fn read_liquidity(&self) -> Result<U256> {
        let lamports =
            self.ctx.rpc_client.get_balance(&self.ctx.global_state_pda).await?;
        Ok(U256::from(lamports))
    }

    async fn maybe_rebalance_contract_liquidity(
        &self,
        native_liq: U256,
    ) -> Result<()> {
        let low_threshold = U256::from(10_000_000_000u64);
        let enable_topup =
            self.ctx.cfg.enable_onchain_lp_topup.to_lowercase() == "true";

        if native_liq < low_threshold && enable_topup {
            let ix = system_instruction::transfer(
                &self.ctx.operator.pubkey(),
                &self.ctx.global_state_pda,
                (low_threshold - native_liq).as_u64(),
            );
            let bh = self.ctx.rpc_client.get_latest_blockhash().await?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&self.ctx.operator.pubkey()),
                &[&*self.ctx.operator],
                bh,
            );
            self.ctx.rpc_client.send_and_confirm_transaction(&tx).await?;
        }
        Ok(())
    }

    async fn jump_to_anchor_from_zero_active(
        &self,
        global_tip: u64,
        anchor_h: u64,
    ) -> Result<u64> {
        if anchor_h <= global_tip {
            return Ok(global_tip);
        }
        let first_h = epoch_start(anchor_h);
        let mut last_success = global_tip;

        for h in [first_h, anchor_h] {
            if h <= global_tip {
                continue;
            }

            let (_, header80) = header80_by_height(&self.core_ctx, h).await?;
            let header80_bytes =
                decode_header80(&header80).map_err(|e| anyhow!("{e}"))?;

            let (header_pda, _) = Pubkey::find_program_address(
                &[b"header", &h.to_le_bytes()],
                &ID,
            );

            if self.ctx.rpc_client.get_account(&header_pda).await.is_ok() {
                tracing::info!(
                    height = h,
                    "Jump target height already stored, skipping transaction."
                );
                last_success = h;
                continue;
            }

            let prev_height = if h > 0 { h - 1 } else { 0 };
            let (prev_height_tracker_pda, _) = Pubkey::find_program_address(
                &[b"tracker", &prev_height.to_le_bytes()],
                &ID,
            );

            let is_retarget =
                h % 2016 == 0 && global_tip != 0 && h == global_tip + 1;

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
            data.extend_from_slice(&header80_bytes[..80]);
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

            let bh = self.ctx.rpc_client.get_latest_blockhash().await?;
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&self.ctx.operator.pubkey()),
                &[&*self.ctx.operator],
                bh,
            );

            match self.ctx.rpc_client.send_and_confirm_transaction(&tx).await {
                Ok(_) => {
                    tracing::info!(
                        height = h,
                        "⛓️ [JUMP] Successfully stored jump header"
                    );
                    last_success = h;
                },
                Err(e) => {
                    let err_str = e.to_string();
                    if err_str.contains("6011") {
                        tracing::info!(
                            height = h,
                            "Jump header already exists, continuing"
                        );
                        last_success = h;
                    } else {
                        tracing::error!(height = h, error = %err_str, "Failed to send jump transaction");
                        return Err(anyhow::anyhow!(
                            "Jump failed: {}",
                            err_str
                        ));
                    }
                },
            }
        }
        Ok(last_success)
    }

    async fn next_tx_id(&self) -> Result<U256> {
        let state = self.get_global_state().await?;
        Ok(U256::from(state.next_tx_id))
    }

    async fn global_tip_height(&self) -> Result<U256> {
        let state = self.get_global_state().await?;
        Ok(U256::from(state.global_tip_height))
    }

    async fn proof_info(&self, tx_id: U256) -> Result<SubmittedProofInfo> {
        let tx_id_u64 = tx_id.as_u64();
        let (proof_cache_pda, _) = Pubkey::find_program_address(
            &[b"proof", &tx_id_u64.to_le_bytes()],
            &ID,
        );

        let (conversion_pda, _) = Pubkey::find_program_address(
            &[b"conversion", &tx_id_u64.to_le_bytes()],
            &ID,
        );

        // 1. Try to fetch the NEW 2-Phase ProofCache PDA
        match self.ctx.rpc_client.get_account_data(&proof_cache_pda).await {
            Ok(cache_data) => {
                if cache_data.len() >= 60 {
                    let mut txid_le = [0u8; 32];
                    txid_le.copy_from_slice(&cache_data[20..52]);

                    let mut pb_height_bytes = [0u8; 8];
                    pb_height_bytes.copy_from_slice(&cache_data[52..60]);

                    // We skip parsing branch_le and merkle_index (offset 60+)
                    // Need to find out_value_sats and out_program at the end of the struct
                    // Let's use Anchor deserialization to be safe instead of manual offsets
                    // since branch_le is a Vec (dynamic size)
                    if let Ok(pc) = crate::bindings::ipow_types::ipow_paradapp_solana::accounts::ProofCache::try_deserialize(&mut &cache_data[..]) {
                        return Ok(SubmittedProofInfo {
                            set: pc.is_set,
                            verified: pc.is_verified,
                            invalid: pc.is_invalid,
                            attempts: pc.attempts,
                            tx_id_le: pc.txid_le,
                            block_hash_le: [0u8; 32],
                            block_height: U256::from(pc.proof_block_height),
                            out_value_sats: pc.out_value_sats,
                            out_program: Bytes::from(pc.out_program),
                        });
                    }
                }
            },
            Err(_) => {
                // 2. FALLBACK FOR OLD TRANSACTIONS
                if let Ok(conv_data) =
                    self.ctx.rpc_client.get_account_data(&conversion_pda).await
                {
                    if let Ok(conv) =
                        SolanaConversion::try_deserialize(&mut &conv_data[..])
                    {
                        if conv.proof_verified {
                            return Ok(SubmittedProofInfo {
                                set: true,
                                verified: true,
                                invalid: false,
                                attempts: 1,
                                tx_id_le: conv.proof_txid_le,
                                block_hash_le: [0u8; 32],
                                block_height: U256::from(
                                    conv.window_start_height + 1,
                                ),
                                out_value_sats: conv.bitcoin_amount,
                                out_program: Bytes::from(conv.user_program),
                            });
                        }
                    }
                }
            },
        }

        // 3. If no cache and not verified, return clean EVM empty state
        Ok(SubmittedProofInfo {
            set: false,
            verified: false,
            invalid: false,
            attempts: 0,
            tx_id_le: [0u8; 32],
            block_hash_le: [0u8; 32],
            block_height: U256::zero(),
            out_value_sats: 0,
            out_program: Bytes::new(),
        })
    }

    async fn min_anchor_height(&self) -> Result<U256> {
        let state = self.get_global_state().await?;
        Ok(U256::from(state.min_anchor_height))
    }

    async fn commit_bitcoin_to_native(
        &self,
        args: paradapp_core::traits::chain_provider_adapter::BitcoinToNativeCommitArgs,
    ) -> anyhow::Result<()> {
        let (global_state_pda, _) =
            Pubkey::find_program_address(&[b"global_state"], &ID);

        // 1. Fetch Global State to get the contract's `next_tx_id`
        let global_data =
            self.ctx.rpc_client.get_account_data(&global_state_pda).await?;
        let mut id_bytes = [0u8; 8];
        id_bytes.copy_from_slice(&global_data[42..50]);
        let next_tx_id = u64::from_le_bytes(id_bytes);

        // 2. Derive PDAs required for OperatorOpenTunnel
        let (conversion_pda, _) = Pubkey::find_program_address(
            &[b"conversion", &next_tx_id.to_le_bytes()],
            &ID,
        );

        let (network_config_pda, _) = Pubkey::find_program_address(
            &[b"network", &args.network_id.as_u64().to_le_bytes()],
            &ID,
        );

        // 3. Build Instruction Data payload for `operator_open_tunnel`
        let mut data = solana_sdk::hash::hash(b"global:operator_open_tunnel")
            .to_bytes()[..8]
            .to_vec();

        // bitcoin_amount (u64)
        data.extend_from_slice(&args.bitcoin_amount.as_u64().to_le_bytes());
        // native_amount (u64)
        data.extend_from_slice(&args.native_amount.as_u64().to_le_bytes());
        // network_id (u64)
        data.extend_from_slice(&args.network_id.as_u64().to_le_bytes());

        // dest_address (Pubkey - 32 bytes)
        // FIX: Use .as_ref() and copy the full 32 bytes directly!
        let mut pubkey_bytes = [0u8; 32];
        let dest_slice = args.dest_address.as_ref();
        let copy_len = std::cmp::min(dest_slice.len(), 32);
        pubkey_bytes[..copy_len].copy_from_slice(&dest_slice[..copy_len]);
        data.extend_from_slice(&pubkey_bytes);

        // network_address (Vec<u8> -> length u32 + bytes)
        let net_addr = args.network_address.to_vec();
        data.extend_from_slice(&(net_addr.len() as u32).to_le_bytes());
        data.extend_from_slice(&net_addr);

        // duty_window_seconds (i64)
        data.extend_from_slice(
            &(args.duty_window_seconds.as_u64() as i64).to_le_bytes(),
        );

        // paradapp_receive_program (Vec<u8> -> length u32 + bytes)
        let recv_prog = args.paradapp_receive_program.to_vec();
        data.extend_from_slice(&(recv_prog.len() as u32).to_le_bytes());
        data.extend_from_slice(&recv_prog);

        // locked_anchor_height (u64)
        data.extend_from_slice(
            &args.locked_anchor_height.as_u64().to_le_bytes(),
        );

        // 4. Build Instruction
        let ix = Instruction {
            program_id: ID,
            accounts: vec![
                AccountMeta::new(global_state_pda, false), // global_state
                AccountMeta::new(conversion_pda, false),   // conversion
                AccountMeta::new_readonly(network_config_pda, false), // network_config
                AccountMeta::new(self.ctx.operator.pubkey(), true), // operator (signer)
                AccountMeta::new_readonly(system_program::id(), false), // system_program
            ],
            data,
        };

        // Increase compute budget
        let compute_budget_program_id = std::str::FromStr::from_str(
            "ComputeBudget111111111111111111111111111111",
        )
        .unwrap();
        let compute_budget_ix = Instruction {
            program_id: compute_budget_program_id,
            accounts: vec![],
            data: vec![2, 64, 66, 15, 0], // set compute unit limit to 1_000_000
        };

        let bh = self.ctx.rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(
            &[compute_budget_ix, ix],
            Some(&self.ctx.operator.pubkey()),
            &[&*self.ctx.operator],
            bh,
        );

        match self.ctx.rpc_client.send_and_confirm_transaction(&tx).await {
            Ok(sig) => {
                tracing::info!(
                    tx_hash = ?sig,
                    "✅ [TUNNEL SUCCESS] Hedera correctly tunneled the intent into Solana via operator_open_tunnel!"
                );
                Ok(())
            },
            Err(e) => Err(anyhow::anyhow!("RPC Error: {}", e)),
        }
    }
    async fn anchor_info(&self, tx_id: U256) -> Result<AnchorInfo> {
        let (pda, _) = Pubkey::find_program_address(
            &[b"conversion", &tx_id.as_u64().to_le_bytes()],
            &ID,
        );
        let account_data = self.ctx.rpc_client.get_account_data(&pda).await?;
        let conv = SolanaConversion::try_deserialize(&mut &account_data[..])?;
        Ok(AnchorInfo {
            anchor_height: U256::from(conv.window_start_height),
            epoch_first_height: U256::from(epoch_start(
                conv.window_start_height,
            )),
        })
    }

    async fn get_conversion_info(&self, tx_id: U256) -> Result<Conversion> {
        let (pda, _) = Pubkey::find_program_address(
            &[b"conversion", &tx_id.as_u64().to_le_bytes()],
            &ID,
        );
        let account_data = self.ctx.rpc_client.get_account_data(&pda).await?;
        let conv = SolanaConversion::try_deserialize(&mut &account_data[..])?;

        Ok(Conversion {
            user: H160::from_slice(&conv.user.to_bytes()[..20]),
            is_native_to_bitcoin: false,
            user_program: Bytes::from(conv.user_program.to_vec()),
            paradapp_receive_program: Bytes::from(
                conv.paradapp_receive_program.to_vec(),
            ),
            network_address: Bytes::from(conv.network_address.to_vec()),
            network_id: U256::from(conv.network_id),
            native_amount: U256::from(conv.native_amount),
            bitcoin_amount: U256::from(conv.bitcoin_amount),
            commit_fee: U256::from(conv.commit_fee),
            reserved_native: U256::zero(),
            created_at: U256::from(conv.created_at),
            approved_at: U256::from(conv.approved_at),
            deposited_at: U256::from(conv.deposited_at),
            operator_duty_expires_at: U256::from(conv.operator_duty_expires_at),
            approved: conv.approved_at > 0,
            deposited: conv.deposited_at > 0,
            completed: matches!(conv.status, ConversionStatus::Completed),
            refunded: matches!(conv.status, ConversionStatus::Refunded),
        })
    }

    async fn get_global_chain_state(&self) -> Result<GlobalChainState> {
        let state = self.get_global_state().await?;
        let btc_tip = btc_tip_height(&self.core_ctx).await?;

        Ok(GlobalChainState {
            next_tx_id: U256::from(state.next_tx_id),
            confirmations_required: 1,
            btc_tip,
            safe_anchor: btc_tip,
            global_tip: state.global_tip_height,
            active_open: state.active_open_conversions,
        })
    }

    async fn get_tx_ids_by_filter(
        &self,
        filter: TxIdFilter,
    ) -> Result<Vec<U256>> {
        let mut results = Vec::new();

        // ==========================================
        // THE FIX: Safe U256 to u64 downcasting.
        // Prevents panic if filter defaults to U256::MAX.
        // ==========================================
        let from = if filter.from_tx_id > U256::from(u64::MAX) {
            u64::MAX
        } else {
            filter.from_tx_id.as_u64()
        };

        let mut to = if filter.to_tx_id == U256::MAX
            || filter.to_tx_id > U256::from(u64::MAX)
        {
            0 // Treat MAX as 0 to trigger the "latest" fetch from global state below
        } else {
            filter.to_tx_id.as_u64()
        };

        let max_results = if filter.max_results > U256::from(usize::MAX) {
            usize::MAX
        } else {
            filter.max_results.as_usize()
        };

        if max_results == 0 {
            return Ok(results);
        }

        // If to is 0, default to the latest transaction ID from the global state
        if to == 0 {
            if let Ok(data) = self
                .ctx
                .rpc_client
                .get_account_data(&self.ctx.global_state_pda)
                .await
            {
                if let Ok(global_state) =
                    GlobalState::try_deserialize(&mut &data[..])
                {
                    to = global_state.next_tx_id.saturating_sub(1);
                }
            }
        }

        if to < from || to == 0 {
            return Ok(results);
        }

        // Fetch Global State once for tip height (needed for Phase Calculation)
        let mut global_tip_height = 0;

        // In TransactionPhase, 0 is NONE, which we use to mean "No Phase Filter"
        if filter.phase_filter != TransactionPhase::NONE {
            if let Ok(data) = self
                .ctx
                .rpc_client
                .get_account_data(&self.ctx.global_state_pda)
                .await
            {
                if let Ok(global_state) =
                    GlobalState::try_deserialize(&mut &data[..])
                {
                    global_tip_height = global_state.global_tip_height;
                }
            }
        }

        let now_sec = chrono::Utc::now().timestamp();

        // Iterate Backwards (Descending Order to match EVM logic)
        let mut current_id = to;
        while current_id >= from {
            let (pda, _) = Pubkey::find_program_address(
                &[b"conversion", &current_id.to_le_bytes()],
                &ID,
            );

            if let Ok(account_data) =
                self.ctx.rpc_client.get_account_data(&pda).await
            {
                if let Ok(conv) =
                    SolanaConversion::try_deserialize(&mut &account_data[..])
                {
                    let mut is_match = true;

                    // FILTER A: Type Filter
                    if filter.type_filter != TransactionType::ANY {
                        let actual_type = match (
                            conv.is_native_to_bitcoin,
                            conv.network_id > 0,
                        ) {
                            (true, false) => TransactionType::NATIVE_TO_BITCOIN,
                            (false, false) => {
                                TransactionType::BITCOIN_TO_NATIVE
                            },
                            (true, true) => {
                                TransactionType::NATIVE_TO_NATIVE_OUT
                            },
                            (false, true) => {
                                TransactionType::NATIVE_TO_NATIVE_IN
                            },
                        };

                        if actual_type != filter.type_filter {
                            is_match = false;
                        }
                    }

                    // FILTER B: Phase Filter (INLINED LOGIC)
                    if is_match && filter.phase_filter != TransactionPhase::NONE
                    {
                        let deposit_blocks_window: u64 = 10;
                        let proof_blocks_window: u64 = 40;
                        let approval_window_sec: i64 = 15 * 60; // 900 seconds

                        let actual_phase = match conv.status {
                            ConversionStatus::None => TransactionPhase::NONE,
                            ConversionStatus::Committed => {
                                if now_sec
                                    > conv.created_at + approval_window_sec
                                {
                                    TransactionPhase::OPERATOR_APPROVAL_EXPIRED
                                } else {
                                    TransactionPhase::WAITING_OPERATOR_APPROVAL
                                }
                            },
                            ConversionStatus::Approved => {
                                if conv.is_native_to_bitcoin {
                                    let deposit_expires_at = conv
                                        .window_start_height
                                        + deposit_blocks_window
                                        - 1;
                                    if conv.window_started
                                        && global_tip_height
                                            > deposit_expires_at
                                    {
                                        TransactionPhase::USER_ACTION_EXPIRED
                                    } else {
                                        TransactionPhase::WAITING_USER_ACTION
                                    }
                                } else {
                                    if conv.operator_duty_expires_at > 0
                                        && now_sec
                                            > conv.operator_duty_expires_at
                                    {
                                        TransactionPhase::OPERATOR_DUTY_EXPIRED
                                    } else {
                                        TransactionPhase::ACTIVE_WAITING_PROOF
                                    }
                                }
                            },
                            ConversionStatus::Deposited => {
                                let proof_expires_at = conv.window_start_height
                                    + proof_blocks_window
                                    - 1;
                                if conv.window_started
                                    && global_tip_height > proof_expires_at
                                {
                                    TransactionPhase::OPERATOR_DUTY_EXPIRED
                                } else {
                                    TransactionPhase::ACTIVE_WAITING_PROOF
                                }
                            },
                            ConversionStatus::Completed => {
                                TransactionPhase::COMPLETED
                            },
                            ConversionStatus::Refunded => {
                                TransactionPhase::REFUNDED
                            },
                        };

                        if actual_phase != filter.phase_filter {
                            is_match = false;
                        }
                    }

                    // FILTER C: Destination Network
                    if is_match {
                        if let Some(dest_net) = filter.dest_network {
                            if conv.network_id != (dest_net as u64) {
                                is_match = false;
                            }
                        }
                    }

                    // FILTER D: Bitcoin Program Filter (User vs Paradapp)
                    if is_match {
                        if let Some(prog_filter) =
                            &filter.bitcoin_program_filter
                        {
                            let prog_bytes = prog_filter.to_vec();
                            let target_prog = if matches!(
                                filter.bitcoin_program_type,
                                Some(BitcoinProgramType::Paradapp)
                            ) {
                                &conv.paradapp_receive_program
                            } else {
                                &conv.user_program
                            };

                            if target_prog != &prog_bytes {
                                is_match = false;
                            }
                        }
                    }

                    // If it passed all filters, add it to the results!
                    if is_match {
                        results.push(U256::from(current_id));
                        if results.len() >= max_results {
                            break;
                        }
                    }
                }
            }

            if current_id == 0 {
                break;
            } // Prevent underflow
            current_id -= 1;
        }

        Ok(results)
    }
}
