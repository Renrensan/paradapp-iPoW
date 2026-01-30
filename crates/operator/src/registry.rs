use paradapp_chain_evm::{network::EvmNetwork, stack::EvmStack};
use paradapp_core::context::CoreContext;
use std::sync::Arc;

pub struct Registry;

impl Registry {
    pub async fn get_stack(
        name: &str,
        core_ctx: Arc<CoreContext>,
    ) -> anyhow::Result<Arc<EvmStack>> {
        let network = match name.to_lowercase().as_str() {
            "hedera" => EvmNetwork::Hedera,
            "ethereum" | "ethereumsepolia" => EvmNetwork::EthereumSepolia,
            _ => return Err(anyhow::anyhow!("Unsupported network: {}", name)),
        };

        let stack = EvmStack::init(network, core_ctx).await?;
        Ok(Arc::new(stack))
    }
}
