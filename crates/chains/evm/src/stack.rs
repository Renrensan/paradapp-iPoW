use sqlx::SqlitePool;
use std::sync::Arc;

use crate::approving_adapter::EvmApprovingAdapter;
use crate::converting_adapter::EvmConvertingAdapter;
use crate::dependencies::config::EvmConfig;
use crate::dependencies::context::EvmContext;
use crate::dependencies::db::sqlite::SqliteStorage;
use crate::helper::EvmChainHelper;
use crate::network::EvmNetwork;
use crate::streaming_adapter::EvmStreamingAdapter;

// Core Trait Imports
use paradapp_core::context::CoreContext;
use paradapp_core::traits::chain_helper_adapter::ChainHelperAdapter;

pub struct EvmStack {
    pub network_id: String,
    pub helper: Arc<EvmChainHelper>,
    pub streaming: Arc<EvmStreamingAdapter>,
    pub approving: Arc<EvmApprovingAdapter>,
    pub converting: Arc<EvmConvertingAdapter>,
    pub sqlite_pool: SqlitePool,
}

impl EvmStack {
    pub async fn init(network: EvmNetwork, core_ctx: Arc<CoreContext>) -> anyhow::Result<Self> {
        let network_name = network.string_identifier().to_string();

        // 1. Initialize Context
        let cfg = EvmConfig::load(network);
        let ctx = Arc::new(EvmContext::init(cfg).await?);

        // 2. Initialize SQLITE Pool
        let sqlite = SqliteStorage::init(network.string_identifier()).await?;
        let sqlite_pool = sqlite.pool();

        // 3. Initialize Helper
        let helper = Arc::new(EvmChainHelper::new(ctx.clone(), core_ctx.clone()));

        // 4. Cast helper to the Trait Object required by your adapters
        let helper_trait: Arc<dyn ChainHelperAdapter> = helper.clone();

        // 5. Initialize Adapters matching your exact struct definition
        let approving = Arc::new(EvmApprovingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            sqlite_storage: sqlite_pool.clone(),
            helper: helper_trait.clone(),
        });

        let converting = Arc::new(EvmConvertingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            sqlite_storage: sqlite_pool.clone(),
            helper: helper_trait.clone(),
        });

        let streaming = Arc::new(EvmStreamingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            helper: helper_trait,
        });

        Ok(Self {
            network_id: network_name,
            helper,
            streaming,
            approving,
            converting,
            sqlite_pool: sqlite_pool.clone(),
        })
    }
}
