pub mod common;
pub mod runtime;

use std::sync::Arc;

use clap::{Parser, ValueEnum};
use paradapp_core::context::{CoreConfig, CoreContext};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Network {
    Hedera,
    Ethereum,
    All,
}

/// Paradapp Operator
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Network to run the operator on
    #[arg(long, value_enum)]
    network: Network,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = paradapp_core::tracing::init("paradapp_operator");

    let cli = Cli::parse();
    tracing::info!(network = ?cli.network, "starting operator");

    let core_ctx = Arc::new(CoreContext::init(CoreConfig::load()).await?);

    match cli.network {
        Network::Hedera => {
            let hedera = runtime::hedera::start(core_ctx.clone()).await?;
            tokio::select! {
                _ = hedera => {}
            }
        }

        Network::Ethereum => {
            let ethereum = runtime::ethereum::start(core_ctx.clone()).await?;
            tokio::select! {
                _ = ethereum => {}
            }
        }

        Network::All => {
            let hedera = runtime::hedera::start(core_ctx.clone()).await?;
            let ethereum = runtime::ethereum::start(core_ctx.clone()).await?;
            tokio::select! {
                _ = hedera => {},
                _ = ethereum => {},
            }
        }
    }

    Ok(())
}
