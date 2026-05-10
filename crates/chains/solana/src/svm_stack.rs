use crate::approving_adapter::SolanaApprovingAdapter;
use crate::converting_adapter::SolanaConvertingAdapter;
use crate::dependencies::config::SolanaConfig;
use crate::dependencies::context::SolanaContext;
use crate::network::SolanaNetwork;
use crate::streaming_adapter::SolanaStreamingAdapter;
use crate::svm_provider::SvmChainProvider;
use async_trait::async_trait;
use paradapp_core::consts::supported_network_enum::SupportedNetwork;
use paradapp_core::dependencies::context::CoreContext;
use paradapp_core::traits::{
    approving_adapter::ApprovingAdapter,
    chain_provider_adapter::ChainProviderAdapter, chain_stack::ChainStack,
    converting_adapter::ConvertingAdapter, streaming_adapter::StreamingAdapter,
};
use std::sync::Arc;

pub struct SvmStack {
    pub network_id: String,
    pub network_enum: SupportedNetwork,
    pub chain_provider: Arc<SvmChainProvider>,
    pub streaming: Arc<SolanaStreamingAdapter>,
    pub approving: Arc<SolanaApprovingAdapter>,
    pub converting: Arc<SolanaConvertingAdapter>,
}

impl SvmStack {
    pub async fn init(
        network: SolanaNetwork,
        core_ctx: Arc<CoreContext>,
    ) -> anyhow::Result<Self> {
        let network_name = network.string_identifier().to_string();
        let network_enum: SupportedNetwork = network.into();

        let cfg = SolanaConfig::load(network);
        let ctx = Arc::new(SolanaContext::init(cfg).await?);

        let provider =
            Arc::new(SvmChainProvider::new(ctx.clone(), core_ctx.clone()));
        let provider_trait: Arc<dyn ChainProviderAdapter> = provider.clone();

        let streaming = Arc::new(SolanaStreamingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            chain_provider: provider_trait.clone(),
        });

        let approving = Arc::new(SolanaApprovingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            chain_provider: provider_trait.clone(),
        });

        let converting = Arc::new(SolanaConvertingAdapter {
            ctx: ctx.clone(),
            core_ctx: core_ctx.clone(),
            chain_provider: provider_trait.clone(),
        });

        Ok(Self {
            network_id: network_name,
            network_enum,
            chain_provider: provider,
            streaming,
            approving,
            converting,
        })
    }
}

#[async_trait]
impl ChainStack for SvmStack {
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
        self.chain_provider.core_ctx.clone()
    }
}
