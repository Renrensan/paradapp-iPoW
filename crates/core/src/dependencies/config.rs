use config::{Config, Environment, File};
use std::env;

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
    pub redis_url: String,
    pub high_finality_confirmed_block: u32,
    pub rbf_blocks_since_anchor: u64,
    pub rbf_blocks_from_tip_to_unconfirmed: u64,
}

impl CoreConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        // Initialize the config builder with YAML and Environment sources
        let settings = Config::builder()
            // Load the YAML file first
            .add_source(File::with_name("config").required(false))
            // Environment variables override YAML values
            .add_source(Environment::default().separator("__"))
            .build()
            .unwrap_or_else(|_| Config::default());

        // Bitcoin network
        let btc_network = env::var("BTC_NETWORK")
            .or_else(|_| settings.get_string("bitcoin.network"))
            .unwrap_or_else(|_| "testnet".to_string());

        let high_finality_confirmed_block: u32 =
            env::var("HIGH_FINALITY_CONFIRMED_BLOCK")
                .or_else(|_| {
                    settings.get_string("bitcoin.high_finality_confirmed_block")
                })
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .expect("HIGH_FINALITY_CONFIRMED_BLOCK must be a valid u32");

        let rbf_blocks_since_anchor: u64 = env::var("RBF_BLOCKS_SINCE_ANCHOR")
            .or_else(|_| settings.get_string("bitcoin.rbf_blocks_since_anchor"))
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .expect("RBF_BLOCKS_SINCE_ANCHOR must be a valid u64");

        let rbf_blocks_from_tip_to_unconfirmed: u64 =
            env::var("RBF_BLOCKS_FROM_TIP_TO_UNCONFIRMED")
                .or_else(|_| {
                    settings.get_string(
                        "bitcoin.rbf_blocks_from_tip_to_unconfirmed",
                    )
                })
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .expect(
                    "RBF_BLOCKS_FROM_TIP_TO_UNCONFIRMED must be a valid u64",
                );

        // Explorers / APIs
        let esplora_base = env::var("ESPLORA_BASE")
            .or_else(|_| settings.get_string("bitcoin.esplora_base"))
            .unwrap_or_else(|_| "https://blockstream.info/api".to_string());

        let mempool_api = env::var("MEMPOOL_API")
            .or_else(|_| settings.get_string("bitcoin.mempool_api"))
            .unwrap_or_else(|_| "https://mempool.space/api".to_string());

        // Wallets
        let operator_btc_wallet_address =
            env::var("OPERATOR_BTC_WALLET_ADDRESS")
                .expect("Set OPERATOR_BTC_WALLET_ADDRESS in .env");

        let operator_btc_wallet_private_key =
            env::var("OPERATOR_BTC_WALLET_PRIVATE_KEY")
                .expect("Set OPERATOR_BTC_WALLET_PRIVATE_KEY in .env");

        // Optional BTC wallet extras
        let btc_hot = env::var("BTC_HOT_ADDRESS").unwrap_or_default();
        let btc_main = env::var("BTC_MAIN_ADDRESS").unwrap_or_default();

        // BTC derivation sources
        let btc_root_xpub = env::var("BTC_ROOT_XPUB").ok();
        let paradapp_receive_program =
            env::var("PARADAPP_RECEIVE_PROGRAM").ok();
        if btc_root_xpub.is_none() && paradapp_receive_program.is_none() {
            panic!("Set BTC_ROOT_XPUB or PARADAPP_RECEIVE_PROGRAM in .env");
        }

        // Optional BTC mnemonic
        let btc_mnemonic = env::var("BTC_MNEMONIC").ok();

        let redis_url = env::var("REDIS_URL")
            .expect("REDIS_URL must be set in .env or environment");

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
            redis_url,
            rbf_blocks_since_anchor,
            rbf_blocks_from_tip_to_unconfirmed,
            high_finality_confirmed_block,
        }
    }
}
