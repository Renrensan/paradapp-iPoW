use paradapp_chain_evm::{evm_stack::EvmStack, network::EvmNetwork};
use paradapp_core::{context::CoreContext, traits::chain_stack::ChainStack};
use std::sync::Arc;

pub struct Registry;

impl Registry {
    pub async fn get_stack(
        name: &str,
        core_ctx: Arc<CoreContext>,
    ) -> anyhow::Result<Arc<dyn ChainStack>> {
        let name_lc = name.to_lowercase();

        match name_lc.as_str() {
            // EVM-based networks
            "hedera" | "ethereum" | "ethereumsepolia" => {
                let network = match name_lc.as_str() {
                    "hedera" => EvmNetwork::Hedera,
                    _ => EvmNetwork::EthereumSepolia,
                };

                let stack = EvmStack::init(network, core_ctx).await?;
                Ok(Arc::new(stack))
            }

            /* Future expansion:
            "solana" => {
                let stack = SolanaStack::init(core_ctx).await?;
                Ok(Arc::new(stack))
            }
            */
            _ => Err(anyhow::anyhow!("Unsupported network: {}", name)),
        }
    }
}
