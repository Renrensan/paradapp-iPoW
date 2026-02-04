use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SupportedNetwork {
    BTC = 0,
    HEDERA = 1,
    ETH = 2,
}

impl SupportedNetwork {
    pub fn from_u8(val: u8) -> Option<Self> {
        match val {
            0 => Some(Self::BTC),
            1 => Some(Self::HEDERA),
            2 => Some(Self::ETH),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BTC => "btc",
            Self::HEDERA => "hedera",
            Self::ETH => "ethereum",
        }
    }
}

impl fmt::Display for SupportedNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
