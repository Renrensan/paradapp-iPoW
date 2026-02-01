use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use ethers::types::U256;

use crate::{
    btc::btc_service::BitcoinMerkleProofPayload, consts::supported_network_enum::SupportedNetwork,
    models::conversion::Conversion,
};

#[async_trait]
pub trait ConvertingAdapter: Send + Sync {
    async fn check_rpc_health(&self) -> Result<()>;

    async fn next_tx_id(&self) -> Result<U256>;

    async fn mark_processed(&self, tx_id: U256, btc_tx_id: Option<String>) -> Result<()>;

    async fn read_liquidity(&self) -> Result<U256>;

    async fn maybe_rebalance_contract_liquidity(&self, native_liq: U256) -> anyhow::Result<()>;

    async fn find_native_to_btc_ready(
        &self,
        to_tx_id: U256,
        dest_network: Option<SupportedNetwork>,
    ) -> Result<Vec<(U256, Conversion)>>;

    async fn find_btc_to_native_completed(
        &self,
        to_tx_id: U256,
        dest_network: Option<SupportedNetwork>,
    ) -> Result<Vec<(U256, Conversion)>>;

    async fn handle_native_to_btc_conversion(&self, tx_id: U256, conv: Conversion) -> Result<()>;

    async fn handle_btc_to_native_conversion(&self, tx_id: U256, conv: Conversion) -> Result<()>;

    async fn check_confirmation_and_build_proof(
        &self,
        tx_id: U256,
        btc_txid: &str,
    ) -> Result<Option<BitcoinMerkleProofPayload>>;

    async fn submit_merkle_proof(
        &self,
        tx_id: U256,
        proof: BitcoinMerkleProofPayload,
    ) -> Result<()>;

    async fn get_processed_native_to_btc(&self, tx_ids: &[U256]) -> Result<HashMap<U256, String>>;
}
