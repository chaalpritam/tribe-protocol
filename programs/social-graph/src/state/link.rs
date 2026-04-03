use anchor_lang::prelude::*;

/// A single follow relationship. PDA-per-relationship model.
///
/// - O(1) follow / unfollow (create / close one small PDA)
/// - O(1) "does A follow B?" (derive PDA, check if it exists)
/// - Unlimited follows per user (no Vec, no account size limit)
/// - Unfollow reclaims rent (~0.001 SOL back to follower)
#[account]
pub struct Link {
    pub follower_tid: u64,
    pub following_tid: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Link {
    pub const SIZE: usize = 8 + 8 + 8 + 8 + 1; // = 33
}
