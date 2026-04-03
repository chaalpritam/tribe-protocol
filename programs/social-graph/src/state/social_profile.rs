use anchor_lang::prelude::*;

/// Social profile — stores only counters. One per TID.
#[account]
pub struct SocialProfile {
    pub tid: u64,
    pub following_count: u32,
    pub followers_count: u32,
    pub bump: u8,
}

impl SocialProfile {
    pub const SIZE: usize = 8 + 8 + 4 + 4 + 1; // = 25
}
