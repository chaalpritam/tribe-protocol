use anchor_lang::prelude::*;

#[event]
pub struct ProfileInitialized {
    pub tid: u64,
}

#[event]
pub struct Followed {
    pub follower_tid: u64,
    pub following_tid: u64,
}

#[event]
pub struct Unfollowed {
    pub follower_tid: u64,
    pub following_tid: u64,
}
