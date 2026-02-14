use std::sync::Arc;

use crate::{
    common::{
        consts::liquidity::Liquidity,
        helpers::{
            parse_native_token::parse_human_native_token,
            preflight::preflight_commit_global,
        },
    },
    dependencies::context::EvmContext,
};
use anyhow::anyhow;
use anyhow::{Context, Result};
use async_trait::async_trait;
use ethers::types::U256;
use paradapp_core::{
    btc::btc_service::{
        btc_tip_height, decode_header80, epoch_start, header80_by_height,
        sweep_btc_to_main,
    },
    consts::supported_network_enum::SupportedNetwork,
    dependencies::context::CoreContext,
    models::conversion::Conversion,
    traits::chain_provider_adapter::{
        AnchorInfo, BitcoinProgramType, BitcoinToNativeCommitArgs,
        ChainProviderAdapter, GlobalChainState, TxIdFilter,
    },
};
use tracing::{error, info, warn};

/// EVM-specific chain provider that wraps contract binding calls.
pub struct EvmChainProvider {
    pub ctx: Arc<EvmContext>,
    pub core_ctx: Arc<CoreContext>,
}

impl EvmChainProvider {
    pub fn new(ctx: Arc<EvmContext>, core_ctx: Arc<CoreContext>) -> Self {
        Self { ctx, core_ctx }
    }
}

#[async_trait]
impl ChainProviderAdapter for EvmChainProvider {
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
        // 1. Acquire a permit from the global limiter.
        // Since rpc_limiter is an Arc<Semaphore>, this is thread-safe across all stacks.
        let _permit =
            self.core_ctx.rpc_limiter.acquire().await.map_err(|e| {
                anyhow::anyhow!("Failed to acquire RPC permit: {}", e)
            })?;

        // --- EVM RPC Check ---
        let evm_ok = match self
            .ctx
            .provider
            .request::<(), ethers::types::U64>("eth_blockNumber", ())
            .await
        {
            Ok(bn) => {
                info!(block_number = %bn, "EVM RPC alive");
                true
            },
            Err(e) => {
                error!(error = %e, "EVM RPC health check failed");
                false
            },
        };

        // --- Bitcoin Esplora Check ---
        let url =
            format!("{}/blocks/tip/height", self.core_ctx.cfg.esplora_base);
        let btc_ok = match self.core_ctx.http.get(url).send().await {
            Ok(resp) => {
                let status = resp.status();
                if status.is_success() {
                    match resp.text().await {
                        Ok(text) => text.trim().parse::<u32>().is_ok(),
                        Err(e) => {
                            error!(error = %e, "Failed to read Esplora response text");
                            false
                        },
                    }
                } else {
                    error!(status = %status.as_u16(), "Bitcoin Esplora returned error status");
                    false
                }
            },
            Err(e) => {
                error!(error = %e, "Bitcoin Esplora connection failed");
                false
            },
        };

        // 2. Mandatory gap enforcement.
        // We sleep while still holding the _permit.
        // No other task can acquire the permit until this sleep finishes.
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

        // 3. Evaluation
        if !evm_ok || !btc_ok {
            anyhow::bail!(
                "One or more upstream RPCs are down (EVM: {}, BTC: {})",
                evm_ok,
                btc_ok
            );
        }

        Ok(())
    }

    async fn trigger_btc_sweep(&self) -> Result<()> {
        let network = self.ctx.cfg.network.string_identifier();

        // 1. Get the starting point from Redis
        let start =
            self.core_ctx.redis_storage.get_last_swept_index(network).await?;

        // 2. Get the current bridge index (how many addresses have we derived?)
        let end = self
            .core_ctx
            .redis_storage
            .get_next_derivation_index(network)
            .await?;

        if end <= start {
            info!(start, end, "No new indexes to sweep.");
            return Ok(());
        }

        info!(start, end, "Initiating scheduled rebalance sweep");

        // Perform the actual BTC sweep logic
        match sweep_btc_to_main(
            &self.core_ctx,
            &self.ctx.cfg.btc_mnemonic,
            start,
            end,
        )
        .await
        {
            Ok(btc_txid) => {
                if !btc_txid.is_empty() {
                    info!(%btc_txid, start, end, "Sweep successful");
                } else {
                    info!(start, end, "Sweep completed (no funds found)");
                }

                // 3. Update the last swept index in Redis so we don't repeat work
                self.core_ctx
                    .redis_storage
                    .set_last_swept_index(network, end)
                    .await?;
            },
            Err(e) => anyhow::bail!("Provider sweep failed: {}", e),
        }

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
                },
                Err(e) => {
                    info!(
                        error = %e,
                        "nativeLiquidity() view not found or failed; treating as 0."
                    );
                },
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

    async fn maybe_rebalance_contract_liquidity(
        &self,
        native_liq: U256,
    ) -> Result<()> {
        let c_op = self.ctx.c_op.clone();
        let low_native = parse_human_native_token(Liquidity::HBAR_LIQ_LOW)?;
        let high_native = parse_human_native_token(Liquidity::HBAR_LIQ_HIGH)?;
        let enable_topup: bool =
            self.ctx.cfg.enable_onchain_lp_topup.to_lowercase() == "true";

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
                    },
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
            info!(
                "   TODO: call withdrawNativeLiquidity() → deposit to exchange."
            );
        } else {
            info!("Native liquidity within range – no rebalance needed.");
        }

        Ok(())
    }

    async fn jump_to_anchor_from_zero_active(
        &self,
        global_tip: u64,
        anchor_h: u64,
    ) -> Result<u64> {
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

        let first_h = epoch_start(anchor_h);
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
                    format!(
                        "Failed to fetch epoch-first header at height {first_h}"
                    )
                })?;

            let header80_bytes = decode_header80(&header80).map_err(|e| {
                anyhow!("decode failed for first_h={first_h}: {e}")
            })?;

            let preflight = preflight_commit_global(
                &self.ctx,
                header80_bytes.clone(),
                first_h,
            )
            .await;

            if !preflight.static_ok {
                let err = preflight.static_err.unwrap_or_default();
                if err.to_string().to_lowercase().contains("height-rewrite") {
                    info!(height = %first_h, "Epoch-first already committed (height-rewrite)");
                    committed_up_to = committed_up_to.max(first_h);
                } else {
                    return Err(anyhow!(
                        "Preflight failed for epoch-first {first_h}: {err}"
                    ));
                }
            } else {
                match self
                    .ctx
                    .c_op
                    .commit_global_bitcoin_header_80(
                        header80_bytes,
                        U256::from(first_h),
                    )
                    .send()
                    .await
                {
                    Ok(pending) => match pending.await {
                        Ok(Some(receipt)) => {
                            info!(
                                tx_hash = ?receipt.transaction_hash,
                                height = %first_h,
                                "Epoch-first header"
                            );
                            committed_up_to = committed_up_to.max(first_h);
                        },
                        Ok(None) => {
                            warn!(
                                height = %first_h,
                                "Epoch-first header TX dropped from mempool — will retry"
                            );
                        },
                        Err(e) => {
                            warn!(
                                height = %first_h,
                                error = %e,
                                "Epoch-first header TX failed while awaiting receipt — will retry"
                            );
                        },
                    },
                    Err(e) => {
                        warn!(
                            height = %first_h,
                            error = %e,
                            "Failed to broadcast epoch-first — will retry"
                        );
                        // Do NOT advance tip
                    },
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

        let (_, anchor80) =
            header80_by_height(&self.core_ctx, anchor_h).await.with_context(
                || format!("Failed to fetch anchor header {anchor_h}"),
            )?;

        let anchor80_bytes = decode_header80(&anchor80).map_err(|e| {
            anyhow!("decode failed for anchor_h={anchor_h}: {e}")
        })?;

        let preflight = preflight_commit_global(
            &self.ctx,
            anchor80_bytes.clone(),
            anchor_h,
        )
        .await;

        if !preflight.static_ok {
            let err = preflight.static_err.unwrap_or_default();
            let err_msg = err.to_string().to_lowercase();
            if err_msg.contains("height-rewrite") {
                info!(height = %anchor_h, "Anchor already stored (height-rewrite)");
            } else if err_msg.contains("no-jump-while-active") {
                warn!(height = %anchor_h, "no-jump-while-active triggered — possible race!");
                return Ok(committed_up_to);
            } else {
                return Err(anyhow!(
                    "Preflight failed for anchor {anchor_h}: {err}"
                ));
            }
        } else {
            match self
                .ctx
                .c_op
                .commit_global_bitcoin_header_80(
                    anchor80_bytes,
                    U256::from(anchor_h),
                )
                .send()
                .await
            {
                Ok(pending) => {
                    match pending.await {
                        Ok(Some(receipt)) => {
                            info!(
                                tx_hash = ?receipt.transaction_hash,
                                height = %anchor_h,
                                "ANCHOR HEADER TX MINED — jump successful"
                            );
                            committed_up_to = anchor_h; // Critical: update to anchor_h, not first_h!
                        },
                        Ok(None) => {
                            warn!(
                                height = %anchor_h,
                                "ANCHOR HEADER TX dropped from mempool — will retry next cycle"
                            );
                            // Do not advance
                        },
                        Err(e) => {
                            warn!(
                                height = %anchor_h,
                                error = %e,
                                "ANCHOR HEADER TX failed while awaiting receipt — will retry next cycle"
                            );
                            // Do not advance
                        },
                    }
                },
                Err(e) => {
                    warn!(
                        height = %anchor_h,
                        error = %e,
                        "Failed to broadcast anchor header — will retry next cycle"
                    );
                    // Do not advance
                },
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

    async fn next_tx_id(&self) -> Result<U256> {
        let next_tx_id: U256 = self.ctx.contract.next_tx_id().call().await?;

        Ok(next_tx_id)
    }

    async fn global_tip_height(&self) -> Result<U256> {
        let c_op = self.ctx.c_op.clone();

        match c_op.global_tip_height().call().await {
            Ok(height) => {
                info!(%height, "Fetched global tip height from EVM contract");
                Ok(height)
            },
            Err(e) => {
                error!(error = %e, "Failed to fetch global tip height");
                Err(anyhow::anyhow!(e))
            },
        }
    }

    async fn min_anchor_height(&self) -> Result<U256> {
        let c_op = self.ctx.c_op.clone();

        match c_op.min_anchor_height().call().await {
            Ok(height) => {
                info!(%height, "Fetched min anchor height from EVM contract");
                Ok(height)
            },
            Err(e) => {
                error!(error = %e, "Failed to fetch min anchor height");
                Err(anyhow::anyhow!(e))
            },
        }
    }

    async fn commit_bitcoin_to_native(
        &self,
        args: BitcoinToNativeCommitArgs,
    ) -> Result<()> {
        let c_op = self.ctx.c_op.clone();

        match c_op
            .commit_bitcoin_to_native(
                args.bitcoin_amount,
                args.network_id,
                args.user_program,
                args.dest_address,
                args.network_address,
                args.duty_window_seconds,
                args.paradapp_receive_program,
                args.locked_anchor_height,
                args.slippage,
            )
            .send()
            .await
        {
            Ok(pending_tx) => {
                info!(
                    tx_hash = ?pending_tx.tx_hash(),
                    "Sent CommitBitcoinToNative transaction"
                );
                Ok(())
            },
            Err(e) => {
                error!(error = %e, "Failed to send CommitBitcoinToNative transaction");
                Err(anyhow::anyhow!(e))
            },
        }
    }

    async fn anchor_info(&self, tx_id: U256) -> Result<AnchorInfo> {
        let c_op = self.ctx.c_op.clone();

        match c_op.anchor_info(tx_id).call().await {
            Ok((anchor_height, epoch_first_height)) => {
                Ok(AnchorInfo { anchor_height, epoch_first_height })
            },
            Err(e) => {
                error!(
                    %tx_id,
                    error = %e,
                    "Failed to fetch anchor info from EVM contract"
                );
                Err(anyhow::anyhow!(e))
            },
        }
    }

    async fn get_conversion_info(&self, tx_id: U256) -> Result<Conversion> {
        let c = &self.ctx.contract;

        let conv = c.conversions(tx_id).call().await?;

        let (
            user,
            is_native_to_bitcoin,
            slippage,
            user_program,
            paradapp_receive_program,
            network_address,
            network_id,
            native_amount,
            bitcoin_amount,
            created_at,
            approved_at,
            deposited_at,
            commit_fee,
            approved,
            deposited,
            completed,
            refunded,
            reserved_native,
            operator_duty_expires_at,
        ) = conv;

        Ok(Conversion {
            user,
            is_native_to_bitcoin,
            slippage,
            user_program,
            paradapp_receive_program,
            network_address,
            network_id,
            native_amount,
            bitcoin_amount,
            created_at,
            approved_at,
            deposited_at,
            commit_fee,
            approved,
            deposited,
            completed,
            refunded,
            reserved_native,
            operator_duty_expires_at,
        })
    }

    async fn get_global_chain_state(&self) -> Result<GlobalChainState> {
        let contract = &self.ctx.contract;

        let next_tx_id = contract.next_tx_id().call().await?;
        let conf_req = contract.confirmations_required().call().await?.as_u64();
        let global_tip = contract.global_tip_height().call().await?.as_u64();
        let active_open =
            contract.active_open_conversions().call().await?.as_u64();
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

    async fn get_tx_ids_by_filter(
        &self,
        filter: TxIdFilter,
    ) -> Result<Vec<U256>> {
        let contract = self.ctx.contract.clone();

        // Filter params
        let user_filter = filter.user_filter.unwrap_or_default();
        let bitcoin_program_filter =
            filter.bitcoin_program_filter.clone().unwrap_or_default();
        let search_user_program = !matches!(
            filter.bitcoin_program_type,
            Some(BitcoinProgramType::Paradapp)
        );
        let (dest_network_u256, use_network_filter): (U256, bool) =
            match filter.dest_network {
                Some(net) => (U256::from(net as u8), true),
                None => (U256::zero(), false),
            };

        // Contract call
        match contract
            .get_tx_ids_by_filter(
                filter.type_filter,
                filter.phase_filter,
                user_filter,
                bitcoin_program_filter,
                search_user_program,
                dest_network_u256,
                use_network_filter,
                filter.from_tx_id,
                filter.to_tx_id,
                filter.max_results,
            )
            .call()
            .await
        {
            Ok(tx_ids) => {
                info!(count = tx_ids.len(), "Fetched tx_ids");
                Ok(tx_ids)
            },
            Err(e) => {
                error!(error = %e, "Failed to fetch tx_ids by filter");
                Err(anyhow::anyhow!(e))
            },
        }
    }

    async fn estimate_bitcoin_from_native(
        &self,
        native_amount: U256,
    ) -> Result<U256> {
        let c_op = self.ctx.c_op.clone();

        match c_op.estimate_bitcoin_from_native(native_amount).call().await {
            Ok(estimated_btc) => {
                info!(
                    native_amount = %native_amount,
                    estimated_btc = %estimated_btc,
                    "Estimated Bitcoin from native"
                );
                Ok(estimated_btc)
            },
            Err(e) => {
                error!(
                    error = %e,
                    native_amount = %native_amount,
                    "Failed to estimate Bitcoin from native"
                );
                Err(anyhow::anyhow!(e))
            },
        }
    }

    async fn estimate_native_from_bitcoin(
        &self,
        bitcoin_amount: U256,
    ) -> anyhow::Result<U256> {
        let c_op = self.ctx.c_op.clone();

        match c_op.estimate_native_from_bitcoin(bitcoin_amount).call().await {
            Ok(native_amount) => {
                info!(%bitcoin_amount, %native_amount, "Estimated native from Bitcoin");
                Ok(native_amount)
            },
            Err(e) => {
                error!(error = %e, %bitcoin_amount, "Failed to estimate native from Bitcoin");
                Err(anyhow::anyhow!(e))
            },
        }
    }
}
