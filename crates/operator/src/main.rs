mod chain_operator;
mod registry;

use clap::{Parser, ValueEnum};
use std::sync::Arc;
use tracing::{error, info};

use crate::chain_operator::ChainOperator;
use crate::registry::Registry;
use paradapp_core::consts::supported_network_enum::SupportedNetwork;
use paradapp_core::context::{CoreConfig, CoreContext};

/// Defines which specific logic engine this instance should run.
#[derive(Debug, Clone, Copy, ValueEnum, PartialEq)]
pub enum Engine {
    Approver,
    Streamer,
    Converter,
    All,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Operator,
    Monolith, // Renamed from "All" to avoid confusion with Engine::All
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, value_enum, default_value = "operator")]
    mode: Mode,

    /// The specific engine to run (approver, streamer, converter, or all)
    /// This allows deploying specific logic pieces to different microservices.
    #[arg(long, value_enum, default_value = "all")]
    engine: Engine,

    /// Source network (e.g., hedera, eth, solana)
    #[arg(long, default_value = "hedera")]
    src: String,

    /// List of networks to watch for bridge intents (connected stacks)
    #[arg(long)]
    watch_sources: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = paradapp_core::tracing::init("paradapp_operator");

    let cli = Cli::parse();
    let core_ctx = Arc::new(CoreContext::init(CoreConfig::load()).await?);

    match cli.mode {
        Mode::Operator => {
            let src_key = cli.src.to_lowercase();
            let stack = Registry::get_stack(&src_key, core_ctx.clone()).await?;

            info!(
                network = %src_key,
                engine = ?cli.engine,
                "Launching Operator Instance"
            );

            // Pass the selected engine to the run function
            ChainOperator::run(stack, cli.watch_sources, cli.engine).await?;
        }

        Mode::Monolith => {
            let hedera_key = SupportedNetwork::HEDERA.as_str();
            let eth_key = SupportedNetwork::ETH.as_str();

            let hedera_stack = Registry::get_stack(hedera_key, core_ctx.clone()).await?;
            let eth_stack = Registry::get_stack(eth_key, core_ctx.clone()).await?;

            info!("Starting Full Monolith (All Networks, All Engines)");

            tokio::select! {
                res = ChainOperator::run(hedera_stack, vec![eth_key.to_string()], Engine::All) => {
                    error!(result = ?res, "Hedera operator task exited");
                },
                res = ChainOperator::run(eth_stack, vec![hedera_key.to_string()], Engine::All) => {
                    error!(result = ?res, "Ethereum operator task exited");
                },
                _ = tokio::signal::ctrl_c() => {
                    info!("Monolith shutdown signal received");
                }
            }
        }
    }

    Ok(())
}
