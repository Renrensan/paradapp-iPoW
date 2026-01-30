use paradapp_chain_evm::stack::EvmStack;
use paradapp_core::{
    consts::supported_network_enum::SupportedNetwork, traits::interop_resolver::InteropResolver,
};
use std::sync::Arc;
use tracing::{info, warn};

pub struct BridgeOperator;

impl BridgeOperator {
    /// Spawns the bridge loop between two specific chains
    pub async fn run(src_stack: Arc<EvmStack>, dst_stack: Arc<EvmStack>) -> anyhow::Result<()> {
        let bridge_name = format!("{}_to_{}", src_stack.network_id, dst_stack.network_id);
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
            source = %src.network_id,
            dest = %dst.network_id
        )
    )]
    async fn tick_interop(src: Arc<EvmStack>, dst: Arc<EvmStack>) -> anyhow::Result<()> {
        let duty_seconds = 24 * 60 * 60;

        let dest_network_enum = match dst.network_id.as_str() {
            "hedera" => SupportedNetwork::HEDERA,
            "ethereum" => SupportedNetwork::ETH,
            _ => {
                return Err(anyhow::anyhow!(
                    "Unsupported destination network for bridge: {}",
                    dst.network_id
                ));
            }
        };

        let resolver = paradapp_interop::resolver::InteropResolver {
            source_helper: src.helper.clone(),
            source_approver: src.approving.clone(),
            source_streaming: src.streaming.clone(),

            dest_helper: dst.helper.clone(),
            dest_approver: dst.approving.clone(),
            dest_streaming: dst.streaming.clone(),

            dest_network: dest_network_enum,
        };

        resolver.run_once(duty_seconds).await?;

        info!("Bridge sync pass completed");
        Ok(())
    }
}
