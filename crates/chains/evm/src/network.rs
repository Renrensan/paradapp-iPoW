use paradapp_core::consts::supported_network_enum::SupportedNetwork;

#[derive(Debug, Clone, Copy)]
pub enum EvmNetwork {
    EthereumSepolia,
    Hedera,
}

// 1. Trait Implementation for conversion
impl From<EvmNetwork> for SupportedNetwork {
    fn from(network: EvmNetwork) -> Self {
        match network {
            EvmNetwork::EthereumSepolia => SupportedNetwork::ETH,
            EvmNetwork::Hedera => SupportedNetwork::HEDERA,
        }
    }
}

// 2. Standard Implementation
impl EvmNetwork {
    pub fn chain_id(&self) -> u64 {
        match self {
            Self::EthereumSepolia => 11155111,
            Self::Hedera => 296,
        }
    }

    pub fn rpc_env(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "ETH_SEPOLIA_RPC",
            Self::Hedera => "HEDERA_RPC",
        }
    }

    pub fn contract_env(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "ETH_SEPOLIA_CONTRACT",
            Self::Hedera => "HEDERA_CONTRACT",
        }
    }

    pub fn operator_private_key(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "ETH_SEPOLIA_OPERATOR_PRIVATE_KEY",
            Self::Hedera => "HEDERA_OPERATOR_PRIVATE_KEY",
        }
    }

    pub fn string_identifier(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "ethereum",
            Self::Hedera => "hedera",
        }
    }

    pub fn btc_root_xpub_env(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "ETH_SEPOLIA_BTC_ROOT_XPUB",
            Self::Hedera => "HEDERA_BTC_ROOT_XPUB",
        }
    }
}
