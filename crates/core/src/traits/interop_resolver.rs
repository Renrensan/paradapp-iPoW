use anyhow::Result;
use async_trait::async_trait;
use ethers::types::U256;

#[async_trait]
pub trait InteropResolver: Send + Sync {
    async fn run_once(&self, duty_seconds: u64) -> Result<()>;

    async fn attempt_approve_one_tx(&self, tx_id: U256, duty_seconds: u64) -> Result<()>;

    async fn attempt_open_tunnel(&self, tx_id: U256) -> Result<()>;
}
