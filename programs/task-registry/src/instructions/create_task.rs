use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::events::TaskCreated;
use crate::state::{CreatorTaskState, Task, TaskStatus};

#[derive(Accounts)]
pub struct CreateTask<'info> {
    #[account(
        mut,
        seeds = [b"task-creator", creator.key().as_ref()],
        bump = creator_state.bump,
        has_one = creator,
    )]
    pub creator_state: Account<'info, CreatorTaskState>,

    #[account(
        init,
        payer = creator,
        space = Task::SIZE,
        seeds = [
            b"task",
            creator.key().as_ref(),
            &creator_state.next_task_id.to_le_bytes(),
        ],
        bump,
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateTask>,
    reward_amount: u64,
    metadata_hash: [u8; 32],
) -> Result<()> {
    let task_id = ctx.accounts.creator_state.next_task_id;
    let now = Clock::get()?.unix_timestamp;

    let task = &mut ctx.accounts.task;
    task.creator = ctx.accounts.creator.key();
    task.creator_tid = ctx.accounts.creator_state.creator_tid;
    task.task_id = task_id;
    task.status = TaskStatus::Open as u8;
    task.reward_amount = reward_amount;
    task.claimer = Pubkey::default();
    task.claimer_tid = 0;
    task.has_claimer = false;
    task.created_at = now;
    task.claimed_at = 0;
    task.completed_at = 0;
    task.metadata_hash = metadata_hash;
    task.bump = ctx.bumps.task;

    // Escrow the reward into the Task PDA up front so the claimer
    // knows there's something real to be paid out on completion.
    if reward_amount > 0 {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.creator.to_account_info(),
                to: ctx.accounts.task.to_account_info(),
            },
        );
        system_program::transfer(cpi_ctx, reward_amount)?;
    }

    ctx.accounts.creator_state.next_task_id = task_id
        .checked_add(1)
        .expect("task_id u64 overflow is unreachable in practice");

    emit!(TaskCreated {
        task: ctx.accounts.task.key(),
        creator: ctx.accounts.task.creator,
        creator_tid: ctx.accounts.task.creator_tid,
        task_id,
        reward_amount,
    });

    Ok(())
}
