use anchor_lang::prelude::*;

#[event]
pub struct TidRegistered {
    pub tid: u64,
    pub custody_address: Pubkey,
    pub recovery_address: Pubkey,
}

#[event]
pub struct TidTransferred {
    pub tid: u64,
    pub old_custody: Pubkey,
    pub new_custody: Pubkey,
}

#[event]
pub struct TidRecovered {
    pub tid: u64,
    pub old_custody: Pubkey,
    pub new_custody: Pubkey,
}

#[event]
pub struct RecoveryChanged {
    pub tid: u64,
    pub old_recovery: Pubkey,
    pub new_recovery: Pubkey,
}
