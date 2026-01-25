use crate::{bindings::paradapp_convert::ParadappConvert, dependencies::config::EvmConfig};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::Address,
};
use std::sync::Arc;

/// Unified runtime context for EVM
#[derive(Clone)]
pub struct EvmContext {
    pub provider: Arc<Provider<Http>>,
    pub contract: Arc<ParadappConvert<Provider<Http>>>,
    pub c_op: Arc<ParadappConvert<SignerMiddleware<Provider<Http>, LocalWallet>>>,
    pub cfg: Arc<EvmConfig>,
}

impl EvmContext {
    pub async fn init(cfg: EvmConfig) -> anyhow::Result<Self> {
        let cfg = Arc::new(cfg);

        // Provider
        let provider = Arc::new(Provider::<Http>::try_from(cfg.rpc_url.as_str())?);

        // Wallet
        let wallet: LocalWallet = cfg.operator_private_key.parse()?;
        let wallet = wallet.with_chain_id(cfg.network.chain_id());
        let wallet = Arc::new(wallet);

        // Contract
        let contract_address: Address = cfg.contract_address.parse::<Address>()?;
        // ---------
        // Read-only contract
        // ---------
        let contract = Arc::new(ParadappConvert::new(contract_address, provider.clone()));
        // ---------
        // Signer middleware
        // ---------
        let signer = Arc::new(SignerMiddleware::new(
            (*provider).clone(),
            (*wallet).clone(),
        ));
        // ---------
        // Operator-enabled contract
        // ---------
        let c_op = Arc::new(ParadappConvert::new(contract_address, signer.clone()));

        Ok(Self {
            provider,
            contract,
            c_op,
            cfg,
        })
    }
}
