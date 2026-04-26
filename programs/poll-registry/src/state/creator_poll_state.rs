use anchor_lang::prelude::*;

/// Per-creator counter. Lives at PDA `["poll-creator", creator_pubkey]`.
#[account]
pub struct CreatorPollState {
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub next_poll_id: u64,
    pub bump: u8,
}

impl CreatorPollState {
    // discriminator(8) + creator(32) + creator_tid(8) + next_poll_id(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1;
}
