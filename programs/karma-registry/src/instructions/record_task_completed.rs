use anchor_lang::prelude::*;

use task_registry::state::{Task, TaskStatus};

use crate::errors::KarmaRegistryError;
use crate::events::TaskKarmaRecorded;
use crate::state::{KarmaAccount, KarmaProof, KARMA_PROOF_KIND_TASK};

#[derive(Accounts)]
pub struct RecordTaskCompleted<'info> {
    #[account(
        mut,
        seeds = [b"karma".as_ref(), &karma.tid.to_le_bytes()],
        bump = karma.bump,
    )]
    pub karma: Account<'info, KarmaAccount>,

    /// The Task to credit. Must be in the Completed state and have
    /// a claimer_tid matching the karma account.
    pub task: Account<'info, Task>,

    #[account(
        init,
        payer = payer,
        space = KarmaProof::SIZE,
        seeds = [b"karma-proof", task.key().as_ref()],
        bump,
    )]
    pub karma_proof: Account<'info, KarmaProof>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RecordTaskCompleted>) -> Result<()> {
    let task = &ctx.accounts.task;
    let karma = &mut ctx.accounts.karma;

    require!(
        task.status == TaskStatus::Completed as u8,
        KarmaRegistryError::TaskNotCompleted
    );
    require!(
        task.claimer_tid == karma.tid,
        KarmaRegistryError::TidMismatch
    );

    karma.tasks_completed_count = karma
        .tasks_completed_count
        .checked_add(1)
        .expect("tasks_completed_count overflow unreachable in practice");
    karma.tasks_completed_reward_lamports = karma
        .tasks_completed_reward_lamports
        .checked_add(task.reward_amount)
        .expect("tasks_completed_reward_lamports overflow unreachable in practice");

    let proof = &mut ctx.accounts.karma_proof;
    proof.source = task.key();
    proof.kind = KARMA_PROOF_KIND_TASK;
    proof.tid = karma.tid;
    proof.bump = ctx.bumps.karma_proof;

    emit!(TaskKarmaRecorded {
        karma: karma.key(),
        tid: karma.tid,
        task: task.key(),
        reward_amount: task.reward_amount,
        new_task_count: karma.tasks_completed_count,
        new_task_reward_lamports: karma.tasks_completed_reward_lamports,
    });

    Ok(())
}
