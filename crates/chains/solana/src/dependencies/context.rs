use crate::bindings::ipow_types::ipow_paradapp_solana::ID;
use crate::dependencies::config::SolanaConfig;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct SolanaContext {
    pub rpc_client: Arc<RpcClient>,
    pub program_id: Pubkey,
    pub operator: Arc<Keypair>,
    pub cfg: Arc<SolanaConfig>,
    pub rpc_limiter: Arc<Semaphore>,
    pub global_state_pda: Pubkey,
    pub escrow_vault_pda: Pubkey,
}

impl SolanaContext {
    pub async fn init(cfg: SolanaConfig) -> anyhow::Result<Self> {
        let cfg = Arc::new(cfg);
        let rpc_client = Arc::new(RpcClient::new(cfg.rpc_url.clone()));

        let operator =
            Arc::new(Keypair::from_base58_string(&cfg.operator_private_key));

        let program_id = ID;

        let (global_state_pda, _) =
            Pubkey::find_program_address(&[b"global_state"], &program_id);

        let (escrow_vault_pda, _) =
            Pubkey::find_program_address(&[b"escrow"], &program_id);

        Ok(Self {
            rpc_client,
            program_id,
            operator,
            cfg,
            rpc_limiter: Arc::new(Semaphore::new(1)),
            global_state_pda,
            escrow_vault_pda,
        })
    }
}
