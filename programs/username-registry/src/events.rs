use anchor_lang::prelude::*;

#[event]
pub struct UsernameRegistered {
    pub username: String,
    pub tid: u64,
    pub expiry: i64,
}

#[event]
pub struct UsernameRenewed {
    pub username: String,
    pub tid: u64,
    pub new_expiry: i64,
}

#[event]
pub struct UsernameTransferred {
    pub username: String,
    pub old_tid: u64,
    pub new_tid: u64,
}

#[event]
pub struct UsernameReleased {
    pub username: String,
    pub tid: u64,
}
