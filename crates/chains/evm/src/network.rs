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

    pub fn string_identifier(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "ethereum",
            Self::Hedera => "hedera",
        }
    }

    // --- config.yml PATHS ---

    pub fn rpc_config_path(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "networks.eth_sepolia.rpc_url",
            Self::Hedera => "networks.hedera.rpc_url",
        }
    }

    pub fn min_limit_config_path(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => {
                "networks.eth_sepolia.min_transaction_limit"
            },
            Self::Hedera => "networks.hedera.min_transaction_limit",
        }
    }

    pub fn max_limit_config_path(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => {
                "networks.eth_sepolia.max_transaction_limit"
            },
            Self::Hedera => "networks.hedera.max_transaction_limit",
        }
    }

    // --- .env KEYS ---

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

    pub fn btc_root_xpub_env(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "ETH_SEPOLIA_BTC_ROOT_XPUB",
            Self::Hedera => "HEDERA_BTC_ROOT_XPUB",
        }
    }

    pub fn btc_mnemonic_env(&self) -> &'static str {
        match self {
            Self::EthereumSepolia => "ETH_SEPOLIA_BTC_MNEMONIC",
            Self::Hedera => "HEDERA_BTC_MNEMONIC",
        }
    }
}
