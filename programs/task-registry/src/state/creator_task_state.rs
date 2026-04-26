use anchor_lang::prelude::*;

/// Per-creator counter. Lives at PDA `["task-creator", creator_pubkey]`.
#[account]
pub struct CreatorTaskState {
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub next_task_id: u64,
    pub bump: u8,
}

impl CreatorTaskState {
    // discriminator(8) + creator(32) + creator_tid(8) + next_task_id(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1;
}
