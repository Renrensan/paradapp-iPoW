use crate::btc::btc::parse_btc_network;
use bitcoin::Network;
use reqwest::Client;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct CoreConfig {
    pub esplora_base: String,
    pub mempool_api: String,
    pub btc_network: String,
    pub operator_btc_wallet_address: String,
    pub operator_btc_wallet_private_key: String,
    pub btc_root_xpub: Option<String>,
    pub paradapp_receive_program: Option<String>,
    pub btc_hot_address: Option<String>,
    pub btc_main_address: Option<String>,
    pub btc_mnemonic: Option<String>,
}

impl CoreConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        // Bitcoin network
        let btc_network = env::var("BTC_NETWORK").unwrap_or_else(|_| "testnet".to_string());

        // Explorers / APIs
        let esplora_base =
            env::var("ESPLORA_BASE").unwrap_or_else(|_| "https://blockstream.info/api".to_string());

        let mempool_api =
            env::var("MEMPOOL_API").unwrap_or_else(|_| "https://mempool.space/api".to_string());

        // Wallets
        let operator_btc_wallet_address = env::var("OPERATOR_BTC_WALLET_ADDRESS")
            .expect("Set OPERATOR_BTC_WALLET_ADDRESS in .env");

        let operator_btc_wallet_private_key = env::var("OPERATOR_BTC_WALLET_PRIVATE_KEY")
            .expect("Set OPERATOR_BTC_WALLET_PRIVATE_KEY in .env");

        // Optional BTC wallet extras
        let btc_hot = env::var("BTC_HOT_ADDRESS").unwrap_or_default();
        let btc_main = env::var("BTC_MAIN_ADDRESS").unwrap_or_default();

        // BTC derivation sources
        let btc_root_xpub = env::var("BTC_ROOT_XPUB").ok();
        let paradapp_receive_program = env::var("PARADAPP_RECEIVE_PROGRAM").ok();
        if btc_root_xpub.is_none() && paradapp_receive_program.is_none() {
            panic!("Set BTC_ROOT_XPUB or PARADAPP_RECEIVE_PROGRAM in .env");
        }

        // Optional BTC mnemonic
        let btc_mnemonic = env::var("BTC_MNEMONIC").ok();

        Self {
            esplora_base,
            mempool_api,
            btc_network,
            operator_btc_wallet_address,
            operator_btc_wallet_private_key,
            btc_root_xpub,
            paradapp_receive_program,
            btc_hot_address: if btc_hot.is_empty() {
                None
            } else {
                Some(btc_hot)
            },
            btc_main_address: if btc_main.is_empty() {
                None
            } else {
                Some(btc_main)
            },
            btc_mnemonic,
        }
    }
}

#[derive(Clone)]
pub struct CoreContext {
    pub http: Arc<Client>,
    pub cfg: Arc<CoreConfig>,
    pub btc_network: Network,
}

impl CoreContext {
    pub async fn init(cfg: CoreConfig) -> anyhow::Result<Self> {
        let cfg = Arc::new(cfg);

        // Parse Bitcoin network
        let btc_network = parse_btc_network(&cfg.btc_network)?;

        // HTTP client
        let http = Arc::new(
            Client::builder()
                .timeout(std::time::Duration::from_secs(25))
                .user_agent("paradapp-core-bot")
                .build()?,
        );

        Ok(Self {
            http,
            cfg,
            btc_network,
        })
    }
}
