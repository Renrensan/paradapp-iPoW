use crate::{
    bindings::paradapp_convert::ParadappConvert,
    dependencies::config::EvmConfig,
};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, HttpRateLimitRetryPolicy, Provider, RetryClient},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::sync::Arc;

type ResilientProvider = Provider<RetryClient<Http>>;
type ResilientSigner = SignerMiddleware<Arc<ResilientProvider>, LocalWallet>;
#[derive(Clone)]
pub struct EvmContext {
    pub provider: Arc<ResilientProvider>,
    pub contract: Arc<ParadappConvert<ResilientProvider>>,
    pub c_op: Arc<ParadappConvert<ResilientSigner>>,
    pub cfg: Arc<EvmConfig>,
}

impl EvmContext {
    pub async fn init(cfg: EvmConfig) -> anyhow::Result<Self> {
        let cfg = Arc::new(cfg);

        // Provider
        let client = cfg.rpc_url.parse::<Http>()?;
        let retry_client = RetryClient::new(
            client,
            Box::new(HttpRateLimitRetryPolicy),
            5,
            2000,
        );
        let provider = Arc::new(Provider::new(retry_client));

        // Wallet
        let wallet: LocalWallet = cfg.operator_private_key.parse()?;
        let wallet = wallet.with_chain_id(cfg.network.chain_id());
        let contract_address: Address = cfg.contract_address.parse()?;

        // Read-only contract
        let contract =
            Arc::new(ParadappConvert::new(contract_address, provider.clone()));
        // Signer middleware
        let signer_inner = SignerMiddleware::new(provider.clone(), wallet);
        let signer = Arc::new(signer_inner);

        // Operator-enabled contract
        let c_op =
            Arc::new(ParadappConvert::new(contract_address, signer.clone()));

        Ok(Self { provider, contract, c_op, cfg })
    }
}
