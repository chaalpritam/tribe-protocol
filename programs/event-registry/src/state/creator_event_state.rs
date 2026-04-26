use anchor_lang::prelude::*;

/// Per-creator counter. Lives at PDA `["event-creator", creator_pubkey]`.
#[account]
pub struct CreatorEventState {
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub next_event_id: u64,
    pub bump: u8,
}

impl CreatorEventState {
    // discriminator(8) + creator(32) + creator_tid(8) + next_event_id(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1;
}
