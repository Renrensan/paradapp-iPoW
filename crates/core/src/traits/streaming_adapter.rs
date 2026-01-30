use anyhow::Result;
use async_trait::async_trait;
use ethers::types::U256;

use crate::consts::supported_network_enum::SupportedNetwork;

pub struct StreamTarget {
    pub needed: bool,
    pub target_height: u64,
    pub reason: String,
}
#[async_trait]
pub trait StreamingAdapter: Send + Sync {
    async fn push_headers_global(
        &self,
        target_height_plus: u64,
        tx_ids_to_check: Vec<U256>,
    ) -> Result<()>;
    async fn get_active_tx_ids(
        &self,
        max_results: u64,
        dest_network: Option<SupportedNetwork>,
    ) -> Result<Vec<U256>>;
    async fn compute_stream_target(&self, tx_id: U256) -> Result<StreamTarget>;
}
