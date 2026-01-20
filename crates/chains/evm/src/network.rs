#[derive(Debug, Clone, Copy)]
pub enum EvmNetwork {
    EthereumSepolia,
    Hedera,
}

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
}
