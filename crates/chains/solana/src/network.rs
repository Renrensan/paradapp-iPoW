use paradapp_core::consts::supported_network_enum::SupportedNetwork;

#[derive(Debug, Clone, Copy)]
pub enum SolanaNetwork {
    Mainnet,
    Devnet,
    Localnet,
}

// 1. Trait Implementation for conversion
impl From<SolanaNetwork> for SupportedNetwork {
    fn from(_network: SolanaNetwork) -> Self {
        SupportedNetwork::SOLANA
    }
}

// 2. Standard Implementation matching EVM pattern
impl SolanaNetwork {
    pub fn chain_id(&self) -> u64 {
        match self {
            Self::Mainnet => 101,  // Mainnet-beta
            Self::Devnet => 103,   // Devnet
            Self::Localnet => 104, // Testnet/Local
        }
    }

    pub fn string_identifier(&self) -> &'static str {
        match self {
            Self::Mainnet => "solana_mainnet",
            Self::Devnet => "solana_devnet",
            Self::Localnet => "solana_localnet",
        }
    }

    // --- config.yml PATHS ---

    pub fn rpc_config_path(&self) -> &'static str {
        match self {
            Self::Mainnet => "networks.solana_mainnet.rpc_url",
            Self::Devnet => "networks.solana_devnet.rpc_url",
            Self::Localnet => "networks.solana_localnet.rpc_url",
        }
    }

    pub fn min_limit_config_path(&self) -> &'static str {
        match self {
            Self::Mainnet => "networks.solana_mainnet.min_transaction_limit",
            Self::Devnet => "networks.solana_devnet.min_transaction_limit",
            Self::Localnet => "networks.solana_localnet.min_transaction_limit",
        }
    }

    pub fn max_limit_config_path(&self) -> &'static str {
        match self {
            Self::Mainnet => "networks.solana_mainnet.max_transaction_limit",
            Self::Devnet => "networks.solana_devnet.max_transaction_limit",
            Self::Localnet => "networks.solana_localnet.max_transaction_limit",
        }
    }

    pub fn contract_config_path(&self) -> &'static str {
        match self {
            Self::Mainnet => "networks.solana_mainnet.contract_address",
            Self::Devnet => "networks.solana_devnet.contract_address",
            Self::Localnet => "networks.solana_localnet.contract_address",
        }
    }

    // --- .env KEYS ---

    pub fn operator_private_key(&self) -> &'static str {
        match self {
            Self::Mainnet => "SOLANA_MAINNET_OPERATOR_KEY",
            Self::Devnet => "SOLANA_DEVNET_OPERATOR_KEY",
            Self::Localnet => "SOLANA_LOCALNET_OPERATOR_KEY",
        }
    }

    pub fn btc_root_xpub_env(&self) -> &'static str {
        match self {
            Self::Mainnet => "SOLANA_MAINNET_BTC_ROOT_XPUB",
            _ => "SOLANA_DEVNET_BTC_ROOT_XPUB",
        }
    }

    pub fn btc_mnemonic_env(&self) -> &'static str {
        match self {
            Self::Mainnet => "SOLANA_MAINNET_BTC_MNEMONIC",
            _ => "SOLANA_DEVNET_BTC_MNEMONIC",
        }
    }
}
