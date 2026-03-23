use anyhow::Result;
use async_trait::async_trait;
use ethers::types::{Address, Bytes, H160, U256};

use crate::{
    consts::supported_network_enum::SupportedNetwork,
    models::conversion::Conversion,
};

/// Parameters for filtering transaction IDs.
#[derive(Default, Clone)]
pub enum BitcoinProgramType {
    #[default]
    User,
    Paradapp,
}
#[derive(Clone)]
pub struct TxIdFilter {
    pub type_filter: u8,
    pub phase_filter: u8,
    pub user_filter: Option<H160>,
    pub bitcoin_program_filter: Option<Bytes>,
    pub bitcoin_program_type: Option<BitcoinProgramType>,
    pub dest_network: Option<SupportedNetwork>,
    pub from_tx_id: U256,
    pub to_tx_id: U256,
    pub max_results: U256,
}
impl Default for TxIdFilter {
    fn default() -> Self {
        Self {
            type_filter: 0,
            phase_filter: 0,
            user_filter: None,
            bitcoin_program_filter: None,
            bitcoin_program_type: None,
            dest_network: None,
            from_tx_id: U256::one(),
            to_tx_id: U256::max_value(),
            max_results: U256::from(500),
        }
    }
}

/// Parameters for committing a Bitcoin -> Native conversion.
pub struct BitcoinToNativeCommitArgs {
    pub bitcoin_amount: U256,
    pub network_id: U256,
    pub user_program: Bytes,
    pub dest_address: Address,
    pub network_address: Bytes,
    pub duty_window_seconds: U256,
    pub paradapp_receive_program: Bytes,
    pub locked_anchor_height: U256,
    pub slippage: u16,
}

#[derive(Debug, Clone)]
pub struct AnchorInfo {
    pub anchor_height: U256,
    pub epoch_first_height: U256,
}

pub struct GlobalChainState {
    pub next_tx_id: U256,
    pub confirmations_required: u64,
    pub global_tip: u64,
    pub safe_anchor: u64,
    pub active_open: u64,
    pub btc_tip: u64,
}

pub struct SubmittedProofInfo {
    pub set: bool,
    pub verified: bool,
    pub invalid: bool,
    pub attempts: u8,
    pub tx_id_le: [u8; 32],
    pub block_hash_le: [u8; 32],
    pub block_height: U256,
    pub out_value_sats: u64,
    pub out_program: ethers::core::types::Bytes,
}
#[async_trait]
pub trait ChainProviderAdapter: Send + Sync {
    async fn check_rpc_health(&self) -> Result<()>;

    fn network(&self) -> SupportedNetwork;

    fn min_transaction_limit(&self) -> u64;

    fn max_transaction_limit(&self) -> u64;

    async fn trigger_btc_sweep(&self) -> Result<()>;

    async fn read_liquidity(&self) -> Result<U256>;

    async fn maybe_rebalance_contract_liquidity(
        &self,
        native_liq: U256,
    ) -> Result<()>;

    async fn jump_to_anchor_from_zero_active(
        &self,
        global_tip: u64,
        anchor_h: u64,
    ) -> Result<u64>;

    async fn global_tip_height(&self) -> Result<U256>;

    async fn proof_info(&self, tx_id: U256) -> Result<SubmittedProofInfo>;

    async fn min_anchor_height(&self) -> Result<U256>;

    async fn get_tx_ids_by_filter(
        &self,
        filter: TxIdFilter,
    ) -> Result<Vec<U256>>;

    async fn next_tx_id(&self) -> Result<U256>;

    async fn commit_bitcoin_to_native(
        &self,
        args: BitcoinToNativeCommitArgs,
    ) -> Result<()>;

    async fn anchor_info(&self, tx_id: U256) -> Result<AnchorInfo>;

    async fn get_conversion_info(&self, tx_id: U256) -> Result<Conversion>;

    async fn get_global_chain_state(&self) -> Result<GlobalChainState>;

    async fn estimate_bitcoin_from_native(
        &self,
        native_amount: U256,
    ) -> Result<U256>;

    async fn estimate_native_from_bitcoin(
        &self,
        bitcoin_amount: U256,
    ) -> anyhow::Result<U256>;
}
