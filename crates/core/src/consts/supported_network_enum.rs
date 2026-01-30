#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SupportedNetwork {
    BTC = 0,
    HEDERA = 1,
    ETH = 2,
}
