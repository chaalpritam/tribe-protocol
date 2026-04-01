use anchor_lang::prelude::*;

#[event]
pub struct ProfileInitialized {
    pub fid: u64,
}

#[event]
pub struct Followed {
    pub follower_fid: u64,
    pub following_fid: u64,
}

#[event]
pub struct Unfollowed {
    pub follower_fid: u64,
    pub following_fid: u64,
}
