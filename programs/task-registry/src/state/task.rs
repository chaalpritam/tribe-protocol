use anchor_lang::prelude::*;

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Open = 0,
    Claimed = 1,
    Completed = 2,
    Cancelled = 3,
}

/// One per task. Lives at PDA
/// `["task", creator_pubkey, task_id_le]`. Like Crowdfund, this PDA
/// doubles as the reward escrow vault — when `reward_amount > 0` the
/// creator transfers that many lamports into the PDA on creation,
/// and complete/cancel mutate them out via direct lamport mutation.
#[account]
pub struct Task {
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub task_id: u64,
    pub status: u8,
    pub reward_amount: u64,
    pub claimer: Pubkey,
    pub claimer_tid: u64,
    pub has_claimer: bool,
    pub created_at: i64,
    pub claimed_at: i64,
    pub completed_at: i64,
    pub metadata_hash: [u8; 32],
    pub bump: u8,
}

impl Task {
    // discriminator(8) + creator(32) + creator_tid(8) + task_id(8)
    // + status(1) + reward_amount(8) + claimer(32) + claimer_tid(8)
    // + has_claimer(1) + created_at(8) + claimed_at(8) + completed_at(8)
    // + metadata_hash(32) + bump(1)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1 + 8 + 32 + 8 + 1 + 8 + 8 + 8 + 32 + 1;
}
