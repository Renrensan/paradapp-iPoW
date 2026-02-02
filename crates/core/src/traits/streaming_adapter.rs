use anyhow::Result;
use async_trait::async_trait;
use ethers::types::U256;

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

    async fn compute_stream_target(&self, tx_id: U256) -> Result<StreamTarget>;

    async fn stream_headers_to_height(
        &self,
        current_tip: u64,
        up_to_height: u64,
        max_count: u64,
    ) -> Result<u64>;
}
