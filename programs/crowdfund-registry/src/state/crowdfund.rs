use anchor_lang::prelude::*;

/// Campaign status. `Active` is the only state where pledges are
/// accepted; once the deadline passes, the campaign moves to either
/// `Succeeded` (creator can claim) or stays `Active` until the
/// creator/backers settle, at which point individual refunds flip it
/// to `Failed`. We don't auto-transition on deadline; the next
/// settlement instruction reads the clock.
#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum CrowdfundStatus {
    Active = 0,
    Succeeded = 1,
    Failed = 2,
}

/// One per campaign. Lives at PDA
/// `["crowdfund", creator_pubkey, crowdfund_id_le]`. This PDA
/// doubles as the lamport vault — the program owns it, so pledges
/// transfer in via System CPI and claim/refund move lamports out
/// via direct mutation. The on-chain rent-exempt minimum sits
/// alongside `total_pledged` and is never touched.
#[account]
pub struct Crowdfund {
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub crowdfund_id: u64,
    pub goal_amount: u64,
    pub total_pledged: u64,
    pub pledge_count: u32,
    pub deadline_at: i64,
    pub created_at: i64,
    pub status: u8,
    pub bump: u8,
    /// Hash of the off-chain CROWDFUND_ADD message that carries
    /// title / description / image_url / currency, etc.
    pub metadata_hash: [u8; 32],
}

impl Crowdfund {
    // discriminator(8) + creator(32) + creator_tid(8) + crowdfund_id(8)
    // + goal_amount(8) + total_pledged(8) + pledge_count(4)
    // + deadline_at(8) + created_at(8) + status(1) + bump(1)
    // + metadata_hash(32)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 4 + 8 + 8 + 1 + 1 + 32;
}
