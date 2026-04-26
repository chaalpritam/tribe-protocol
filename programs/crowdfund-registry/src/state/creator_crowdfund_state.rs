use anchor_lang::prelude::*;

/// Per-creator counter. Lives at PDA `["cf-creator", creator_pubkey]`.
/// Each crowdfund the creator creates consumes the current
/// `next_crowdfund_id` and increments it, so campaign PDAs are
/// deterministic and globally unique per creator.
#[account]
pub struct CreatorCrowdfundState {
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub next_crowdfund_id: u64,
    pub bump: u8,
}

impl CreatorCrowdfundState {
    // discriminator(8) + creator(32) + creator_tid(8) + next_crowdfund_id(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1;
}
