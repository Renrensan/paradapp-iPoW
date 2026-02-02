use crate::approving_adapter::EvmApprovingAdapter;
use crate::converting_adapter::EvmConvertingAdapter;
use crate::dependencies::config::EvmConfig;
use crate::dependencies::context::EvmContext;
use crate::dependencies::db::sqlite::SqliteStorage;
use crate::evm_provider::EvmChainProvider;
use crate::network::EvmNetwork;
use crate::streaming_adapter::EvmStreamingAdapter;
use async_trait::async_trait;
use paradapp_core::consts::supported_network_enum::SupportedNetwork;
use paradapp_core::context::CoreContext;
use paradapp_core::traits::approving_adapter::ApprovingAdapter;
use paradapp_core::traits::chain_provider_adapter::ChainProviderAdapter;
use paradapp_core::traits::chain_stack::ChainStack;
use paradapp_core::traits::converting_adapter::ConvertingAdapter;
use paradapp_core::traits::streaming_adapter::StreamingAdapter;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct EvmStack {
    pub network_id: String,
    pub network_enum: SupportedNetwork,
    pub chain_provider: Arc<EvmChainProvider>,
    pub streaming: Arc<EvmStreamingAdapter>,
    pub approving: Arc<EvmApprovingAdapter>,
    pub converting: Arc<EvmConvertingAdapter>,
    pub sqlite_pool: SqlitePool,
}

impl EvmStack {
    pub async fn init(network: EvmNetwork, core_ctx: Arc<CoreContext>) -> anyhow::Result<Self> {
        let network_name = network.string_identifier().to_string();
        let network_enum: SupportedNetwork = network.into();

        // 1. Initialize Context
        let cfg = EvmConfig::load(network);
        let ctx = Arc::new(EvmContext::init(cfg).await?);

        // 2. Initialize SQLITE Pool
        let sqlite = SqliteStorage::init(network.string_identifier()).await?;
        let sqlite_pool = sqlite.pool();

        // 3. Initialize Provider
        let provider = Arc::new(EvmChainProvider::new(ctx.clone(), core_ctx.clone()));

        // 4. Cast provider to the Trait Object
        let provider_trait: Arc<dyn ChainProviderAdapter> = provider.clone();

        // 5. Initialize Adapters matching your exact struct definition
        let approving = Arc::new(EvmApprovingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            sqlite_storage: sqlite_pool.clone(),
            chain_provider: provider_trait.clone(),
        });

        let converting = Arc::new(EvmConvertingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            sqlite_storage: sqlite_pool.clone(),
            chain_provider: provider_trait.clone(),
        });

        let streaming = Arc::new(EvmStreamingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            chain_provider: provider_trait,
        });

        Ok(Self {
            network_id: network_name,
            network_enum,
            chain_provider: provider,
            streaming,
            approving,
            converting,
            sqlite_pool: sqlite_pool.clone(),
        })
    }
}

#[async_trait]
impl ChainStack for EvmStack {
    fn converting(&self) -> Arc<dyn ConvertingAdapter> {
        self.converting.clone()
    }

    fn approving(&self) -> Arc<dyn ApprovingAdapter> {
        self.approving.clone()
    }

    fn streaming(&self) -> Arc<dyn StreamingAdapter> {
        self.streaming.clone()
    }

    fn chain_provider(&self) -> Arc<dyn ChainProviderAdapter> {
        self.chain_provider.clone()
    }

    fn network_id(&self) -> &str {
        &self.network_id
    }

    fn network_enum(&self) -> SupportedNetwork {
        self.network_enum
    }

    fn core_context(&self) -> Arc<CoreContext> {
        self.converting.core_ctx.clone()
    }
}
