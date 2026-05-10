use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SupportedNetwork {
    BTC = 0,
    HEDERA = 1,
    ETH = 2,
    SOLANA = 3,
}

impl SupportedNetwork {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::BTC),
            1 => Some(Self::HEDERA),
            2 => Some(Self::ETH),
            3 => Some(Self::SOLANA),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "btc" | "bitcoin" => Some(Self::BTC),
            "hedera" => Some(Self::HEDERA),
            "ethereum" | "eth" => Some(Self::ETH),
            "solana" | "sol" => Some(Self::SOLANA),
            _ => None,
        }
    }

    pub fn decimals(&self) -> u32 {
        match self {
            Self::BTC => 8,
            Self::HEDERA => 8,
            Self::ETH => 18,
            Self::SOLANA => 9,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BTC => "btc",
            Self::HEDERA => "hedera",
            Self::ETH => "ethereum",
            Self::SOLANA => "solana",
        }
    }
}

impl fmt::Display for SupportedNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
