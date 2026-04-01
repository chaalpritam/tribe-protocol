use anchor_lang::prelude::*;

#[event]
pub struct FidRegistered {
    pub fid: u64,
    pub custody_address: Pubkey,
    pub recovery_address: Pubkey,
}

#[event]
pub struct FidTransferred {
    pub fid: u64,
    pub old_custody: Pubkey,
    pub new_custody: Pubkey,
}

#[event]
pub struct FidRecovered {
    pub fid: u64,
    pub old_custody: Pubkey,
    pub new_custody: Pubkey,
}

#[event]
pub struct RecoveryChanged {
    pub fid: u64,
    pub old_recovery: Pubkey,
    pub new_recovery: Pubkey,
}
