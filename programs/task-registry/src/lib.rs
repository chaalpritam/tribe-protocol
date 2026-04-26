use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("TaskReg111111111111111111111111111111111111");

#[program]
pub mod task_registry {
    use super::*;

    /// One-time per-creator setup. Creates the CreatorTaskState PDA
    /// tracking a monotonic `next_task_id`.
    pub fn init_creator_state(ctx: Context<InitCreatorState>, creator_tid: u64) -> Result<()> {
        instructions::init_creator_state::handler(ctx, creator_tid)
    }

    /// Create a task. If `reward_amount > 0`, the creator escrows
    /// that many lamports into the Task PDA; the reward is paid out
    /// to the claimer on completion or returned to the creator on
    /// cancel.
    pub fn create_task(
        ctx: Context<CreateTask>,
        reward_amount: u64,
        metadata_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_task::handler(ctx, reward_amount, metadata_hash)
    }

    /// Claim an open task. Locks the task to a single claimer until
    /// the creator marks it complete or cancels.
    pub fn claim_task(ctx: Context<ClaimTask>, claimer_tid: u64) -> Result<()> {
        instructions::claim_task::handler(ctx, claimer_tid)
    }

    /// Creator-only. Marks the task complete and releases the escrow
    /// (if any) to the claimer.
    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
        instructions::complete_task::handler(ctx)
    }

    /// Creator-only. Cancel an open task and refund any escrow.
    /// Cannot be used once the task has been claimed.
    pub fn cancel_task(ctx: Context<CancelTask>) -> Result<()> {
        instructions::cancel_task::handler(ctx)
    }
}
