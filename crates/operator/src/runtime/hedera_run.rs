use std::sync::Arc;

use paradapp_chain_evm::{
    approving_adapter::EvmApprovingAdapter,
    converting_adapter::EvmConvertingAdapter,
    dependencies::{config::EvmConfig, context::EvmContext, db::sqlite::SqliteStorage},
    helper::EvmChainHelper,
    network::EvmNetwork,
    streaming_adapter::EvmStreamingAdapter,
};
use paradapp_core::{
    consts::supported_network_enum::SupportedNetwork, context::CoreContext,
    traits::interop_resolver::InteropResolver as InteropResolverTrait,
};
use paradapp_interop::resolver::InteropResolver;
use tokio::{signal, task::JoinHandle};

use crate::common::{
    approving::{approving_bot::ApprovingBot, approving_orchestrator::ApprovingOrchestrator},
    converting::{converting_bot::ConvertingBot, converting_orchestrator::ConvertingOrchestrator},
    streaming::{streaming_bot::StreamingBot, streaming_orchestrator::StreamingOrchestrator},
};

pub async fn start(core_ctx: Arc<CoreContext>) -> anyhow::Result<JoinHandle<()>> {
    /*
     * ============================================================
     * SOURCE CHAIN: HEDERA
     * ============================================================
     */
    let hedera_network = EvmNetwork::Hedera.string_identifier();

    let hedera_cfg = EvmConfig::load(EvmNetwork::Hedera);
    let hedera_ctx = Arc::new(EvmContext::init(hedera_cfg).await?);

    let hedera_sqlite = SqliteStorage::init(hedera_network).await?;
    let hedera_pool = hedera_sqlite.pool();

    let hedera_helper = Arc::new(EvmChainHelper::new(hedera_ctx.clone(), core_ctx.clone()));

    let hedera_streaming = Arc::new(EvmStreamingAdapter {
        ctx: hedera_ctx.clone(),
        core_ctx: core_ctx.clone(),
        helper: hedera_helper.clone(),
    });
    let hedera_approving = Arc::new(EvmApprovingAdapter {
        ctx: hedera_ctx.clone(),
        core_ctx: core_ctx.clone(),
        sqlite_storage: hedera_pool.clone(),
        helper: hedera_helper.clone(),
    });
    let hedera_converting = Arc::new(EvmConvertingAdapter {
        ctx: hedera_ctx.clone(),
        core_ctx: core_ctx.clone(),
        sqlite_storage: hedera_pool.clone(),
        helper: hedera_helper.clone(),
    });

    /*
     * ============================================================
     * DESTINATION CHAIN: ETHEREUM SEPOLIA
     * ============================================================
     */
    let eth_network = EvmNetwork::EthereumSepolia.string_identifier();

    let eth_cfg = EvmConfig::load(EvmNetwork::EthereumSepolia);
    let eth_ctx = Arc::new(EvmContext::init(eth_cfg).await?);

    let eth_sqlite = SqliteStorage::init(eth_network).await?;
    let eth_pool = eth_sqlite.pool();

    let eth_helper = Arc::new(EvmChainHelper::new(eth_ctx.clone(), core_ctx.clone()));

    let eth_streaming = Arc::new(EvmStreamingAdapter {
        ctx: eth_ctx.clone(),
        core_ctx: core_ctx.clone(),
        helper: eth_helper.clone(),
    });
    let eth_approving = Arc::new(EvmApprovingAdapter {
        ctx: eth_ctx.clone(),
        core_ctx: core_ctx.clone(),
        sqlite_storage: eth_pool.clone(),
        helper: eth_helper.clone(),
    });

    /*
     * ============================================================
     * EXISTING BOTS (HEDERA ONLY – unchanged)
     * ============================================================
     */
    let streaming_orch = StreamingOrchestrator::new(hedera_streaming.clone(), hedera_network);
    let approving_orch = ApprovingOrchestrator::new(
        hedera_approving.clone(),
        hedera_network,
        hedera_helper.clone(),
    );
    let converting_orch =
        ConvertingOrchestrator::new(hedera_converting, core_ctx.clone(), hedera_network);

    let streaming_bot = StreamingBot::new(streaming_orch, 30);
    let approving_bot = ApprovingBot::new(approving_orch, 10);
    let converting_bot = ConvertingBot::new(converting_orch, 15);

    /*
     * ============================================================
     * INTEROP RESOLVER (HEDERA → ETH)
     * ============================================================
     */
    let interop_resolver = Arc::new(InteropResolver {
        // SOURCE = HEDERA
        source_helper: hedera_helper,
        source_approver: hedera_approving,
        source_streaming: hedera_streaming,

        // DEST = ETH
        dest_helper: eth_helper,
        dest_approver: eth_approving,
        dest_streaming: eth_streaming,

        dest_network: SupportedNetwork::ETH,
    });

    /*
     * ============================================================
     * SPAWN BOTS
     * ============================================================
     */
    let interop = interop_resolver.clone();

    Ok(tokio::spawn(async move {
        tokio::select! {
            _ = streaming_bot.run() => {},
            _ = approving_bot.run() => {},
            _ = converting_bot.run() => {},
            _ = async {
                loop {
                    if let Err(e) = interop.run_once(24 * 60 * 60).await {
                        tracing::warn!(error = %e, "Interop run_once failed");
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                }
            } => {},
            _ = signal::ctrl_c() => {
                tracing::info!("Shutdown signal received, breaking loop");
            }
        }
    }))
}
