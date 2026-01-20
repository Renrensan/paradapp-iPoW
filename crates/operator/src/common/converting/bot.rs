use crate::common::converting::converting::ConvertingOrchestrator;
use std::time::Duration;
use tokio::signal;
use tracing::{error, info};

pub struct ConvertingBot<A: paradapp_core::traits::converting::ConvertingAdapter> {
    orchestrator: ConvertingOrchestrator<A>,
    interval: Duration,
}

impl<A: paradapp_core::traits::converting::ConvertingAdapter> ConvertingBot<A> {
    pub fn new(orchestrator: ConvertingOrchestrator<A>, interval_secs: u64) -> Self {
        Self {
            orchestrator,
            interval: Duration::from_secs(interval_secs),
        }
    }

    pub async fn run(self) {
        // run immediately
        if let Err(e) = self.orchestrator.run_once().await {
            error!(error = %e, "initial run failed");
        }

        loop {
            tokio::select! {
                _ = signal::ctrl_c() => {
                    info!("shutdown signal received");
                    break;
                }

                _ = tokio::time::sleep(self.interval) => {
                    if let Err(e) = self.orchestrator.run_once().await {
                        error!(error = %e, "run_once failed");
                    }
                }
            }
        }
    }
}
