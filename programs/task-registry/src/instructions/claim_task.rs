use anchor_lang::prelude::*;

use crate::errors::TaskRegistryError;
use crate::events::TaskClaimed;
use crate::state::{Task, TaskStatus};

#[derive(Accounts)]
pub struct ClaimTask<'info> {
    #[account(
        mut,
        seeds = [
            b"task",
            task.creator.as_ref(),
            &task.task_id.to_le_bytes(),
        ],
        bump = task.bump,
    )]
    pub task: Account<'info, Task>,

    pub claimer: Signer<'info>,
}

pub fn handler(ctx: Context<ClaimTask>, claimer_tid: u64) -> Result<()> {
    let task = &mut ctx.accounts.task;
    require!(
        task.status == TaskStatus::Open as u8,
        TaskRegistryError::NotOpen
    );
    require!(
        task.creator != ctx.accounts.claimer.key(),
        TaskRegistryError::SelfClaim
    );

    task.status = TaskStatus::Claimed as u8;
    task.claimer = ctx.accounts.claimer.key();
    task.claimer_tid = claimer_tid;
    task.has_claimer = true;
    task.claimed_at = Clock::get()?.unix_timestamp;

    emit!(TaskClaimed {
        task: task.key(),
        claimer: task.claimer,
        claimer_tid,
    });

    Ok(())
}
