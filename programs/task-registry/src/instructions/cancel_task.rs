use anchor_lang::prelude::*;

use crate::errors::TaskRegistryError;
use crate::events::TaskCancelled;
use crate::state::{Task, TaskStatus};

#[derive(Accounts)]
pub struct CancelTask<'info> {
    #[account(
        mut,
        seeds = [
            b"task",
            creator.key().as_ref(),
            &task.task_id.to_le_bytes(),
        ],
        bump = task.bump,
        has_one = creator,
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub creator: Signer<'info>,
}

pub fn handler(ctx: Context<CancelTask>) -> Result<()> {
    let task = &ctx.accounts.task;
    // Cancel only makes sense before the task has been claimed; once
    // a claimer has locked it, completion/escrow is the only path
    // forward (otherwise creators could grief claimers).
    require!(
        task.status == TaskStatus::Open as u8,
        TaskRegistryError::NotOpen
    );

    let reward = task.reward_amount;

    if reward > 0 {
        let task_info = ctx.accounts.task.to_account_info();
        let creator_info = ctx.accounts.creator.to_account_info();
        **task_info.try_borrow_mut_lamports()? = task_info
            .lamports()
            .checked_sub(reward)
            .ok_or(error!(TaskRegistryError::NotOpen))?;
        **creator_info.try_borrow_mut_lamports()? = creator_info
            .lamports()
            .checked_add(reward)
            .expect("creator lamport credit overflow unreachable in practice");
    }

    let task = &mut ctx.accounts.task;
    task.status = TaskStatus::Cancelled as u8;

    emit!(TaskCancelled {
        task: task.key(),
        creator: task.creator,
        refunded: reward,
    });

    Ok(())
}
