use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    consts::supported_network_enum::SupportedNetwork,
    context::CoreContext,
    traits::{
        approving_adapter::ApprovingAdapter, chain_provider_adapter::ChainProviderAdapter,
        converting_adapter::ConvertingAdapter, streaming_adapter::StreamingAdapter,
    },
};

#[async_trait]
pub trait ChainStack: Send + Sync {
    fn converting(&self) -> Arc<dyn ConvertingAdapter>;
    fn approving(&self) -> Arc<dyn ApprovingAdapter>;
    fn streaming(&self) -> Arc<dyn StreamingAdapter>;
    fn chain_provider(&self) -> Arc<dyn ChainProviderAdapter>;
    fn network_id(&self) -> &str;
    fn network_enum(&self) -> SupportedNetwork;
    fn core_context(&self) -> Arc<CoreContext>;
}
