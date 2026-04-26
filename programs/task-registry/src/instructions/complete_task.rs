use anchor_lang::prelude::*;

use crate::errors::TaskRegistryError;
use crate::events::TaskCompleted;
use crate::state::{Task, TaskStatus};

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(
        mut,
        seeds = [
            b"task",
            creator.key().as_ref(),
            &task.task_id.to_le_bytes(),
        ],
        bump = task.bump,
        has_one = creator,
        has_one = claimer,
    )]
    pub task: Account<'info, Task>,

    /// CHECK: The claimer of record. Must match `task.claimer`. We
    /// only credit lamports to it; no signature required from the
    /// claimer (creator attests to completion in this V1).
    #[account(mut)]
    pub claimer: SystemAccount<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,
}

pub fn handler(ctx: Context<CompleteTask>) -> Result<()> {
    let task = &ctx.accounts.task;
    require!(
        task.status == TaskStatus::Claimed as u8,
        TaskRegistryError::NotClaimed
    );

    let reward = task.reward_amount;

    // Pay the escrowed reward to the claimer via direct lamport
    // mutation. The Task PDA is owned by this program, so we move
    // lamports from its account to the claimer's account in place.
    if reward > 0 {
        let task_info = ctx.accounts.task.to_account_info();
        let claimer_info = ctx.accounts.claimer.to_account_info();
        **task_info.try_borrow_mut_lamports()? = task_info
            .lamports()
            .checked_sub(reward)
            .ok_or(error!(TaskRegistryError::NotClaimed))?;
        **claimer_info.try_borrow_mut_lamports()? = claimer_info
            .lamports()
            .checked_add(reward)
            .expect("claimer lamport credit overflow unreachable in practice");
    }

    let task = &mut ctx.accounts.task;
    task.status = TaskStatus::Completed as u8;
    task.completed_at = Clock::get()?.unix_timestamp;

    emit!(TaskCompleted {
        task: task.key(),
        creator: task.creator,
        claimer: task.claimer,
        reward_amount: reward,
    });

    Ok(())
}
