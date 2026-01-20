use anyhow::Result;
use async_trait::async_trait;
use ethers::types::U256;

pub struct GlobalChainState {
    pub next_tx_id: U256,
    pub confirmations_required: u64,
    pub global_tip: u64,
    pub safe_anchor: u64,
    pub active_open: u64,
    pub btc_tip: u64,
}

#[async_trait]
pub trait ApprovingAdapter: Send + Sync {
    async fn check_rpc_health(&self) -> Result<()>;

    fn epoch_start(&self, height: u64) -> u64;

    async fn get_global_chain_state(&self) -> Result<GlobalChainState>;

    async fn get_or_create_index_for_tx(&self, tx_id: U256) -> Result<u32>;

    async fn get_or_create_receive_program_for_tx(
        &self,
        tx_id: U256,
    ) -> Result<(u32, String, Vec<u8>)>;

    async fn jump_to_anchor_from_zero_active(&self, global_tip: u64, anchor_h: u64) -> Result<u64>;

    async fn get_tx_ids_by_phase(
        &self,
        phase: u8,
        type_filter: u8,
        from_tx_id: U256,
        to_tx_id: U256,
        max_results: U256,
    ) -> Result<Vec<U256>>;

    async fn handle_operator_closes_for_active(&self, tx_id: U256, conf_req: u64) -> Result<()>;

    async fn discover_user_close_candidates(
        &self,
        to_tx_id: U256,
        conf_req: u64,
    ) -> Result<Vec<(U256, String)>>;

    async fn execute_user_closes(
        &self,
        candidates: Vec<(U256, &'static str)>,
    ) -> anyhow::Result<()>;

    async fn get_pending_txids(&self, max_results: u32) -> Result<Vec<U256>>;

    async fn approve_one_tx(&self, tx_id: U256, duty_seconds: u64) -> Result<()>;

    async fn stream_headers_to_height(
        &self,
        current_tip: u64,
        up_to_height: u64,
        max_count: u64,
    ) -> Result<u64>;
}
