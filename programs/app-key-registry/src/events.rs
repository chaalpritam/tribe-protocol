use anchor_lang::prelude::*;

#[event]
pub struct AppKeyAdded {
    pub tid: u64,
    pub app_pubkey: Pubkey,
    pub scope: u8,
    pub expires_at: i64,
}

#[event]
pub struct AppKeyRevoked {
    pub tid: u64,
    pub app_pubkey: Pubkey,
}

#[event]
pub struct AppKeyRotated {
    pub tid: u64,
    pub old_app_pubkey: Pubkey,
    pub new_app_pubkey: Pubkey,
    pub scope: u8,
}
