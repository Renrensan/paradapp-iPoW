use paradapp_core::{traits::chain_stack::ChainStack, traits::interop_resolver::InteropResolver};
use std::sync::Arc;
use tracing::{info, warn};

pub struct BridgeOperator;

impl BridgeOperator {
    /// Spawns the bridge loop between two specific chains
    pub async fn run(
        src_stack: Arc<dyn ChainStack>,
        dst_stack: Arc<dyn ChainStack>,
    ) -> anyhow::Result<()> {
        let bridge_name = format!("{}_to_{}", src_stack.network_id(), dst_stack.network_id());
        info!(bridge = %bridge_name, "Bridge Operator loop started");

        let mut interval = tokio::time::interval(std::time::Duration::from_secs(15));

        loop {
            interval.tick().await;

            if let Err(e) = Self::tick_interop(src_stack.clone(), dst_stack.clone()).await {
                warn!(bridge = %bridge_name, error = %e, "Interop tick failed");
            }
        }
    }

    #[tracing::instrument(
        name = "operator_crosschain_bridging",
        skip(src, dst),
        fields(
            source = %src.network_id(),
            dest = %dst.network_id()
        )
    )]
    async fn tick_interop(
        src: Arc<dyn ChainStack>,
        dst: Arc<dyn ChainStack>,
    ) -> anyhow::Result<()> {
        let duty_seconds = 24 * 60 * 60;

        let dest_network = dst.network_enum();
        let resolver = paradapp_interop::resolver::InteropResolver {
            source: src,
            dest: dst,
            dest_network,
        };

        resolver.run_once(duty_seconds).await?;

        info!("Bridge sync pass completed");
        Ok(())
    }
}
