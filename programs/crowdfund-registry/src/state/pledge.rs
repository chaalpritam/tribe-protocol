use anchor_lang::prelude::*;

/// One per (crowdfund, backer). Lives at PDA
/// `["pledge", crowdfund_pubkey, backer_pubkey]`. Re-pledging
/// accumulates onto the same record; refund closes the account and
/// returns rent to the backer.
#[account]
pub struct Pledge {
    pub crowdfund: Pubkey,
    pub backer: Pubkey,
    pub backer_tid: u64,
    pub amount: u64,
    pub pledged_at: i64,
    pub bump: u8,
}

impl Pledge {
    // discriminator(8) + crowdfund(32) + backer(32) + backer_tid(8)
    // + amount(8) + pledged_at(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 1;
}
