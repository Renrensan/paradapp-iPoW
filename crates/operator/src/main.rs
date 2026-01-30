mod bridge_operator;
mod local_operator;
mod registry;

use clap::{Parser, ValueEnum};
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::{error, info};

use crate::bridge_operator::BridgeOperator;
use crate::local_operator::LocalOperator;
use crate::registry::Registry;
use paradapp_core::context::{CoreConfig, CoreContext};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Mode {
    Operator,
    Bridge,
    All,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, value_enum)]
    mode: Mode,

    #[arg(long, default_value = "Hedera")]
    src: String,

    #[arg(long)]
    dst: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = paradapp_core::tracing::init("paradapp_operator");

    let cli = Cli::parse();
    info!(mode = ?cli.mode, src = %cli.src, dst = ?cli.dst, "starting operator");

    let core_ctx = Arc::new(CoreContext::init(CoreConfig::load()).await?);

    match cli.mode {
        Mode::Operator => {
            let stack = Registry::get_stack(&cli.src, core_ctx).await?;
            LocalOperator::run(stack).await?;
        }

        Mode::Bridge => {
            if cli.dst.is_empty() {
                return Err(anyhow::anyhow!("--dst is required for bridge mode"));
            }

            let src_stack = Registry::get_stack(&cli.src, core_ctx.clone()).await?;
            let mut set = JoinSet::new();

            for dst_name in cli.dst {
                let dst_stack = Registry::get_stack(&dst_name, core_ctx.clone()).await?;

                let s_fwd = src_stack.clone();
                let d_fwd = dst_stack.clone();
                set.spawn(async move {
                    if let Err(e) = BridgeOperator::run(s_fwd, d_fwd).await {
                        error!(error = %e, "forward bridge task failed");
                    }
                });

                let s_rev = src_stack.clone();
                let d_rev = dst_stack.clone();
                set.spawn(async move {
                    if let Err(e) = BridgeOperator::run(d_rev, s_rev).await {
                        error!(error = %e, "reverse bridge task failed");
                    }
                });
            }

            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("shutdown signal received");
                    set.abort_all();
                }
                _ = async {
                    while let Some(res) = set.join_next().await {
                        if let Err(e) = res {
                            error!(error = %e, "bridge task panicked");
                        }
                    }
                } => {}
            }
        }

        Mode::All => {
            let hedera_stack = Registry::get_stack("Hedera", core_ctx.clone()).await?;
            let eth_stack = Registry::get_stack("Ethereum", core_ctx.clone()).await?;

            info!("starting monolith (all local + bridge)");

            tokio::select! {
                res = LocalOperator::run(hedera_stack.clone()) => {
                    error!(result = ?res, "Hedera local exited");
                },
                res = LocalOperator::run(eth_stack.clone()) => {
                    error!(result = ?res, "Ethereum local exited");
                },
                res = BridgeOperator::run(hedera_stack.clone(), eth_stack.clone()) => {
                    error!(result = ?res, "Hedera -> Eth bridge exited");
                },
                res = BridgeOperator::run(eth_stack, hedera_stack) => {
                    error!(result = ?res, "Eth -> Hedera bridge exited");
                },
                _ = tokio::signal::ctrl_c() => {
                    info!("monolith shutdown signal received");
                }
            }
        }
    }

    Ok(())
}
