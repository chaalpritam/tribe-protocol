use anchor_lang::prelude::*;

/// Maximum options per poll. Keeps the on-chain account fixed-size
/// without bringing in dynamic Vec serialization.
pub const MAX_POLL_OPTIONS: usize = 8;

/// One per poll. Lives at PDA
/// `["poll", creator_pubkey, poll_id_le]`.
///
/// Question text and option labels live in the off-chain POLL_ADD
/// envelope; this account stores their hash plus the per-option
/// tally counters.
#[account]
pub struct Poll {
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub poll_id: u64,
    pub option_count: u8,
    /// option_count <= MAX_POLL_OPTIONS; trailing entries left at zero.
    pub option_votes: [u32; MAX_POLL_OPTIONS],
    pub total_votes: u32,
    pub expires_at: i64,
    pub has_expiry: bool,
    pub created_at: i64,
    pub metadata_hash: [u8; 32],
    pub bump: u8,
}

impl Poll {
    // discriminator(8) + creator(32) + creator_tid(8) + poll_id(8)
    // + option_count(1) + option_votes(4*8) + total_votes(4)
    // + expires_at(8) + has_expiry(1) + created_at(8)
    // + metadata_hash(32) + bump(1)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1 + (4 * MAX_POLL_OPTIONS) + 4 + 8 + 1 + 8 + 32 + 1;
}
