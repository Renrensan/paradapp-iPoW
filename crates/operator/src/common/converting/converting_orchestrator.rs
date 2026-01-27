use anyhow::Result;
use ethers_core::types::U256;
use paradapp_core::{
    btc::btc_service::maybe_rebalance_btc_wallets, context::CoreContext,
    traits::converting_adapter::ConvertingAdapter,
};
use std::sync::Arc;
use tracing::{info, warn};

pub struct ConvertingOrchestrator<A: ConvertingAdapter> {
    adapter: Arc<A>,
    core_ctx: Arc<CoreContext>,
    network: &'static str,
}

impl<A: ConvertingAdapter> ConvertingOrchestrator<A> {
    pub fn new(adapter: Arc<A>, core_ctx: Arc<CoreContext>, network: &'static str) -> Self {
        Self {
            adapter,
            core_ctx,
            network,
        }
    }

    #[tracing::instrument(
        name = "operator_converting",
        skip(self),
        fields(network = %self.network)
    )]
    pub async fn run_once(&self) -> Result<()> {
        // Determine latest tx id
        let next_tx_id = self.adapter.next_tx_id().await?;
        let to_tx_id = next_tx_id.saturating_sub(U256::one());

        if to_tx_id == U256::zero() {
            info!("No conversions exist yet.");
            return Ok(());
        }

        // Get conversions needing processing
        let ready_h2b = self.adapter.find_native_to_btc_ready(to_tx_id).await?;
        let ready_b2h = self.adapter.find_btc_to_native_completed(to_tx_id).await?;

        if ready_h2b.is_empty() && ready_b2h.is_empty() {
            info!("No conversions requiring off-chain work this pass.");
        }

        // Handle NATIVE→BTC
        // Collect tx_ids
        let tx_ids: Vec<U256> = ready_h2b.iter().map(|(tx_id, _)| *tx_id).collect();

        // Fetch processed tx_id -> btc_txid
        let processed_map = self.adapter.get_processed_native_to_btc(&tx_ids).await?;

        for (tx_id, conv) in ready_h2b {
            // Case A: BTC already sent → check confirmation & submit proof
            if let Some(btc_txid) = processed_map.get(&tx_id) {
                match self
                    .adapter
                    .check_confirmation_and_build_proof(tx_id, btc_txid)
                    .await?
                {
                    Some(proof) => {
                        if let Err(err) = self.adapter.submit_merkle_proof(tx_id, proof).await {
                            warn!("Error submitting merkle proof txId={}: {:?}", tx_id, err);
                        }
                    }
                    None => {
                        // Not confirmed yet, retry later
                    }
                }

                continue;
            }

            // Case B: Not processed yet → send BTC
            if let Err(err) = self
                .adapter
                .handle_native_to_btc_conversion(tx_id, conv)
                .await
            {
                warn!(
                    "Error handling NATIVE→BTC conversion txId={}: {:?}",
                    tx_id, err
                );
            }
        }

        // Handle BTC→NATIVE
        for (tx_id, conv) in ready_b2h {
            if let Err(err) = self
                .adapter
                .handle_btc_to_native_conversion(tx_id, conv)
                .await
            {
                warn!(
                    "Error handling BTC→NATIVE conversion txId={}: {:?}",
                    tx_id, err
                );
            }
        }

        // Liquidity Check
        let native_liq = self.adapter.read_liquidity().await?;
        self.adapter
            .maybe_rebalance_contract_liquidity(native_liq)
            .await?;

        // BTC hot wallet rebalance
        maybe_rebalance_btc_wallets(&self.core_ctx).await?;

        info!("Done conversion pass.");

        Ok(())
    }
}
