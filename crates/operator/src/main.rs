mod chain_operator;
mod registry;

use clap::{Parser, Subcommand, ValueEnum};
use paradapp_chain_evm::dependencies::config::EvmConfig;
use paradapp_chain_evm::network::EvmNetwork;
use paradapp_core::dependencies::config::CoreConfig;
use paradapp_core::dependencies::context::CoreContext;
use std::sync::{Arc, RwLock};
use tracing::{error, info};

use crate::chain_operator::ChainOperator;
use crate::registry::Registry;
use paradapp_core::consts::supported_network_enum::SupportedNetwork;
use paradapp_operator_api::create_router;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq)]
pub enum Engine {
    Approver,
    Streamer,
    Converter,
    Rebalance,
    All,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run as a specific network operator (No API)
    Operator {
        #[arg(long, value_enum, default_value = "all")]
        engine: Engine,

        #[arg(long, default_value = "hedera")]
        src: String,

        #[arg(long)]
        watch_sources: Vec<String>,
    },

    /// Run only the API service
    Api {
        #[arg(long, default_value = "8888")]
        port: u16,

        /// Which network config to load for the API
        #[arg(long, default_value = "hedera")]
        network: String,
    },

    /// Run everything: All networks AND API service
    Monolith {
        #[arg(long, value_enum, default_value = "all")]
        engine: Engine,

        #[arg(long, default_value = "8888")]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = paradapp_core::tracing::init("paradapp_operator");
    let cli = Cli::parse();
    let core_ctx = Arc::new(CoreContext::init(CoreConfig::load()).await?);

    match cli.command {
        Commands::Operator { engine, src, watch_sources } => {
            let src_key = src.to_lowercase();
            let stack = Registry::get_stack(&src_key, core_ctx.clone()).await?;

            info!(network = %src_key, engine = ?engine, "Launching Operator (Standalone)");
            ChainOperator::run(stack, watch_sources, engine).await?;
        },

        Commands::Api { port, .. } => {
            info!("Launching API Service on port {}", port);

            // 1. Create a map to hold all your network configurations
            let mut all_configs = std::collections::HashMap::new();

            // 2. Define the networks you want the API to track
            let networks =
                vec![EvmNetwork::Hedera, EvmNetwork::EthereumSepolia];

            // 3. Load each one into the map
            for net in networks {
                let identifier = net.string_identifier().to_string();
                let config = EvmConfig::load(net);
                all_configs.insert(identifier, config);
            }

            // 4. Wrap the whole map in the Arc/RwLock
            let shared_state = Arc::new(RwLock::new(all_configs));

            // 5. Start the API with the multi-network state
            start_api(shared_state, port).await;
        },

        Commands::Monolith { engine, port } => {
            info!(
                "Launching Monolith: All Operators + Network-Agnostic API on port {}",
                port
            );

            // 1. Setup API Shared State with multiple networks
            let mut all_configs = std::collections::HashMap::new();

            // List all networks the API should display
            let networks =
                vec![EvmNetwork::Hedera, EvmNetwork::EthereumSepolia];

            for net in networks {
                let identifier = net.string_identifier().to_string();
                let config = EvmConfig::load(net);
                all_configs.insert(identifier, config);
            }

            let api_config = Arc::new(RwLock::new(all_configs));

            // 2. Setup Stacks for the Operators
            let hedera_key = SupportedNetwork::HEDERA.as_str();
            let eth_key = SupportedNetwork::ETH.as_str();

            let hedera_stack =
                Registry::get_stack(hedera_key, core_ctx.clone()).await?;
            let eth_stack =
                Registry::get_stack(eth_key, core_ctx.clone()).await?;

            // 3. Run all tasks concurrently
            // tokio::select! ensures if any critical service (API or Bot) fails, we know immediately.
            tokio::select! {
                _ = start_api(api_config, port) => {
                    error!("API Server task exited unexpectedly");
                },
                res = ChainOperator::run(hedera_stack, vec![eth_key.to_string()], engine) => {
                    error!(result = ?res, "Hedera operator task exited");
                },
                res = ChainOperator::run(eth_stack, vec![hedera_key.to_string()], engine) => {
                    error!(result = ?res, "Ethereum operator task exited");
                },
                _ = tokio::signal::ctrl_c() => {
                    info!("Monolith shutdown signal received (Ctrl+C)");
                }
            }
        },
    }

    Ok(())
}

async fn start_api(
    config: Arc<RwLock<std::collections::HashMap<String, EvmConfig>>>,
    port: u16,
) {
    let app = create_router(config); // This will call the router with the map
    let addr = format!("0.0.0.0:{}", port);

    match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!("API Server running on http://{}", addr);
            axum::serve(listener, app).await.unwrap();
        },
        Err(e) => error!("Could not bind API: {}", e),
    }
}
