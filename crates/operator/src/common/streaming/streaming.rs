use anyhow::Result;
use paradapp_core::traits::streaming::{StreamTarget, StreamingAdapter};
use std::sync::Arc;
use tracing::info;

pub struct StreamingOrchestrator {
    adapter: Arc<dyn StreamingAdapter>,
    network: &'static str,
}

impl StreamingOrchestrator {
    pub fn new(adapter: Arc<dyn StreamingAdapter>, network: &'static str) -> Self {
        Self { adapter, network }
    }

    #[tracing::instrument(
        name = "operator_streaming",
        skip(self),
        fields(network = %self.network)
    )]
    pub async fn run_once(&self) -> Result<()> {
        let active_ids = self.adapter.get_active_tx_ids(1000).await?;
        if active_ids.is_empty() {
            info!("No active conversions found – nothing to stream this pass.");
            return Ok(());
        }

        info!("Found active conversions: {:?}", active_ids);

        let mut needed_tx_ids = Vec::new();
        let mut max_target: u64 = 0;

        for tx_id in active_ids {
            let StreamTarget {
                needed,
                target_height,
                reason,
            } = self.adapter.compute_stream_target(tx_id).await?;

            if !needed {
                info!(tx_id = %tx_id, reason = %reason, "txId does not need streaming.");
                continue;
            }

            needed_tx_ids.push(tx_id);

            if target_height > max_target {
                max_target = target_height;
            }
        }

        if needed_tx_ids.is_empty() {
            info!("No conversions require additional headers this pass.");
            return Ok(());
        }

        let ids_str = needed_tx_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        info!(
            needed_ids = %ids_str,
            max_target,
            "Need to stream headers up to targetHeight"
        );

        self.adapter
            .push_headers_global(max_target, needed_tx_ids)
            .await?;

        info!("Done streaming headers for this pass.");
        Ok(())
    }
}
