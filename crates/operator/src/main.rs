mod local_operator;
mod registry;

use clap::{Parser, ValueEnum};
use std::sync::Arc;
use tracing::{error, info};

use crate::local_operator::LocalOperator;
use crate::registry::Registry;
use paradapp_core::consts::supported_network_enum::SupportedNetwork;
use paradapp_core::context::{CoreConfig, CoreContext};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Operator,
    All,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, value_enum)]
    mode: Mode,

    /// Source network (e.g., hedera, eth, btc)
    #[arg(long, default_value = "hedera")]
    src: String,

    /// List of networks to watch for bridge intents
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
            // Normalizing input to lowercase ensures registry match
            let src_key = cli.src.to_lowercase();
            let stack = Registry::get_stack(&src_key, core_ctx.clone()).await?;

            LocalOperator::run(stack, cli.watch_sources).await?;
        }

        Mode::All => {
            // Using the enum methods we created to get clean keys
            let hedera_key = SupportedNetwork::HEDERA.as_str();
            let eth_key = SupportedNetwork::ETH.as_str();

            let hedera_stack = Registry::get_stack(hedera_key, core_ctx.clone()).await?;
            let eth_stack = Registry::get_stack(eth_key, core_ctx.clone()).await?;

            info!("Starting monolith");

            tokio::select! {
                res = LocalOperator::run(hedera_stack.clone(), vec![eth_key.to_string()]) => {
                    error!(result = ?res, "Hedera operator task exited");
                },
                res = LocalOperator::run(eth_stack.clone(), vec![hedera_key.to_string()]) => {
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
