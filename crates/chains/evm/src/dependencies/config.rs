use crate::network::EvmNetwork;
use config::{Config, File};
use std::env;

#[derive(Clone)]
pub struct EvmConfig {
    pub network: EvmNetwork,
    pub rpc_url: String,
    pub operator_private_key: String,
    pub contract_address: String,
    pub enable_onchain_lp_topup: String,
    pub btc_root_xpub: String,
    pub btc_mnemonic: String,
    pub min_transaction_limit: u64,
    pub max_transaction_limit: u64,
}

impl EvmConfig {
    pub fn load(network: EvmNetwork) -> Self {
        dotenvy::dotenv().ok();

        // Initialize the config builder for YAML only
        let settings = Config::builder()
            .add_source(File::with_name("config").required(true))
            .build()
            .expect("Failed to load config.yml");

        // Map the identifier to the YAML keys
        let net_key = match network.string_identifier() {
            "ethereum" => "eth_sepolia",
            _ => "hedera",
        };

        // --- FROM config.yml ---
        let rpc_url = settings
            .get_string(&format!("networks.{}.rpc_url", net_key))
            .expect("Missing rpc_url in config.yml for selected network");

        let min_transaction_limit: u64 = settings
            .get_string(&format!("networks.{}.min_transaction_limit", net_key))
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .expect("Invalid min transaction limit in config.yml");

        let max_transaction_limit: u64 = settings
            .get_string(&format!("networks.{}.max_transaction_limit", net_key))
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .expect("Invalid max transaction limit in config.yml");

        let contract_address = settings
            .get_string(&format!("networks.{}.contract_address", net_key))
            .expect("Missing contract_address in config.yml");

        // --- FROM .env ---
        let operator_private_key = env::var(network.operator_private_key())
            .expect("Missing operator private key for selected network");

        let enable_onchain_lp_topup: String =
            env::var("ENABLE_ONCHAIN_LP_TOPUP")
                .unwrap_or_else(|_| "false".to_string());

        let btc_root_xpub = env::var(network.btc_root_xpub_env())
            .or_else(|_| env::var("BTC_ROOT_XPUB"))
            .expect("Missing BTC root xpub env var");

        let btc_mnemonic = env::var(network.btc_mnemonic_env())
            .or_else(|_| env::var("BTC_MNEMONIC"))
            .expect("Missing BTC mnemonic env var");

        Self {
            network,
            rpc_url,
            operator_private_key,
            contract_address,
            enable_onchain_lp_topup,
            btc_root_xpub,
            btc_mnemonic,
            min_transaction_limit,
            max_transaction_limit,
        }
    }
}
