pub struct TransactionType;

impl TransactionType {
    pub const ANY: u8 = 0;
    pub const BITCOIN_TO_NATIVE: u8 = 1;
    pub const NATIVE_TO_BITCOIN: u8 = 2;
    pub const NATIVE_TO_NATIVE_IN: u8 = 3;
    pub const NATIVE_TO_NATIVE_OUT: u8 = 4;
}
