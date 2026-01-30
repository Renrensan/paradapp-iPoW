use anyhow::Result;
use async_trait::async_trait;
use ethers::types::U256;

#[async_trait]
pub trait InteropResolver: Send + Sync {
    async fn run_once(&self, duty_seconds: u64) -> Result<()>;

    async fn attempt_approve_one_tx_and_open_tunnel(
        &self,
        tx_id: U256,
        duty_seconds: u64,
    ) -> Result<()>;

    // New User Action Flow (Tunnel Recovery)
    async fn get_txs_by_phase(&self, to_tx_id: U256, phase: u8, max: u64) -> Result<Vec<U256>>;

    async fn attempt_open_tunnel_only(&self, tx_id: U256) -> Result<()>;
}
