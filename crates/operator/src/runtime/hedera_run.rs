use std::sync::Arc;

use paradapp_chain_evm::{
    approving_adapter::EvmApprovingAdapter,
    converting_adapter::EvmConvertingAdapter,
    dependencies::{config::EvmConfig, context::EvmContext, db::sqlite::SqliteStorage},
    network::EvmNetwork,
    streaming_adapter::EvmStreamingAdapter,
};
use paradapp_core::context::CoreContext;
use tokio::task::JoinHandle;

use crate::common::{
    approving::{approving_bot::ApprovingBot, approving_orchestrator::ApprovingOrchestrator},
    converting::{converting_bot::ConvertingBot, converting_orchestrator::ConvertingOrchestrator},
    streaming::{streaming_bot::StreamingBot, streaming_orchestrator::StreamingOrchestrator},
};

pub async fn start(core_ctx: Arc<CoreContext>) -> anyhow::Result<JoinHandle<()>> {
    let network = EvmNetwork::Hedera.string_identifier();

    // network context
    let evm_cfg = EvmConfig::load(EvmNetwork::Hedera);
    let evm_ctx = Arc::new(EvmContext::init(evm_cfg).await?);

    // SQLite storage
    let sqlite_storage = SqliteStorage::init(network).await?;
    let sqlite_storage_pool = sqlite_storage.pool();

    // adapters
    let streaming = Arc::new(EvmStreamingAdapter {
        ctx: evm_ctx.clone(),
        core_ctx: core_ctx.clone(),
    });
    let approving = Arc::new(EvmApprovingAdapter {
        ctx: evm_ctx.clone(),
        core_ctx: core_ctx.clone(),
        sqlite_storage: sqlite_storage_pool.clone(),
    });
    let converting = Arc::new(EvmConvertingAdapter {
        ctx: evm_ctx.clone(),
        core_ctx: core_ctx.clone(),
        sqlite_storage: sqlite_storage_pool.clone(),
    });

    // orchestrators
    let streaming_orch = StreamingOrchestrator::new(streaming, network);
    let approving_orch = ApprovingOrchestrator::new(approving, network);
    let converting_orch = ConvertingOrchestrator::new(converting, core_ctx, network);

    // bots
    let streaming_bot = StreamingBot::new(streaming_orch, 30);
    let approving_bot = ApprovingBot::new(approving_orch, 10);
    let converting_bot = ConvertingBot::new(converting_orch, 15);

    Ok(tokio::spawn(async move {
        tokio::join!(
            streaming_bot.run(),
            approving_bot.run(),
            converting_bot.run()
        );
    }))
}
