use anchor_lang::prelude::*;

/// One per (poll, voter). Lives at PDA
/// `["poll-vote", poll_pubkey, voter_pubkey]`. Existence proves the
/// voter has voted; the `init` constraint at vote time enforces
/// one-vote-per-TID.
#[account]
pub struct Vote {
    pub poll: Pubkey,
    pub voter: Pubkey,
    pub voter_tid: u64,
    pub option_index: u8,
    pub voted_at: i64,
    pub bump: u8,
}

impl Vote {
    // discriminator(8) + poll(32) + voter(32) + voter_tid(8)
    // + option_index(1) + voted_at(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 1 + 8 + 1;
}
