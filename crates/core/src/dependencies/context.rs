use crate::btc::btc_service::parse_btc_network;
use crate::dependencies::config::CoreConfig;
use crate::dependencies::db::redis::RedisStorage;
use bitcoin::Network;
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct CoreContext {
    pub http: Arc<Client>,
    pub cfg: Arc<CoreConfig>,
    pub btc_network: Network,
    pub rpc_limiter: Arc<Semaphore>,
    pub redis_storage: Arc<RedisStorage>,
}

impl CoreContext {
    pub async fn init(cfg: CoreConfig) -> anyhow::Result<Self> {
        let cfg = Arc::new(cfg);

        let redis_storage = Arc::new(RedisStorage::init(&cfg).await?);

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
            rpc_limiter: Arc::new(Semaphore::new(1)),
            redis_storage,
        })
    }
}
