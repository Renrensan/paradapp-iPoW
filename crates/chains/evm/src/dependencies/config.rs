use std::env;

use crate::network::EvmNetwork;

#[derive(Clone)]
pub struct EvmConfig {
    pub network: EvmNetwork,
    pub rpc_url: String,
    pub operator_private_key: String,
    pub contract_address: String,
    pub enable_onchain_lp_topup: String,
    pub btc_root_xpub: String,
}

impl EvmConfig {
    pub fn load(network: EvmNetwork) -> Self {
        dotenvy::dotenv().ok();

        let rpc_url =
            env::var(network.rpc_env()).expect("Missing RPC env var for selected network");

        let contract_address = env::var(network.contract_env())
            .expect("Missing CONTRACT env var for selected network");

        let operator_private_key = env::var(network.operator_private_key())
            .expect("Missing operator private key for selected network");

        let enable_onchain_lp_topup: String =
            env::var("ENABLE_ONCHAIN_LP_TOPUP").unwrap_or_else(|_| "false".to_string());

        let btc_root_xpub = env::var(network.btc_root_xpub_env())
            .or_else(|_| env::var("BTC_ROOT_XPUB"))
            .expect("Missing BTC root xpub env var");

        Self {
            network,
            rpc_url,
            operator_private_key,
            contract_address,
            enable_onchain_lp_topup,
            btc_root_xpub,
        }
    }
}
