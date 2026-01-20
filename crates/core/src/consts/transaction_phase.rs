pub struct TransactionPhase;

impl TransactionPhase {
    pub const NONE: u8 = 0;
    pub const WAITING_OPERATOR_APPROVAL: u8 = 1;
    pub const OPERATOR_APPROVAL_EXPIRED: u8 = 2;
    pub const WAITING_USER_ACTION: u8 = 3;
    pub const ACTIVE_WAITING_PROOF: u8 = 4;
    pub const OPERATOR_DUTY_EXPIRED: u8 = 5;
    pub const COMPLETED: u8 = 6;
    pub const REFUNDED: u8 = 7;
}
