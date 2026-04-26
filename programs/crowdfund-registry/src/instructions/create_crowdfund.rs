use anchor_lang::prelude::*;

use crate::errors::CrowdfundRegistryError;
use crate::events::CrowdfundCreated;
use crate::state::{CreatorCrowdfundState, Crowdfund, CrowdfundStatus};

#[derive(Accounts)]
pub struct CreateCrowdfund<'info> {
    #[account(
        mut,
        seeds = [b"cf-creator", creator.key().as_ref()],
        bump = creator_state.bump,
        has_one = creator,
    )]
    pub creator_state: Account<'info, CreatorCrowdfundState>,

    #[account(
        init,
        payer = creator,
        space = Crowdfund::SIZE,
        seeds = [
            b"crowdfund",
            creator.key().as_ref(),
            &creator_state.next_crowdfund_id.to_le_bytes(),
        ],
        bump,
    )]
    pub crowdfund: Account<'info, Crowdfund>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateCrowdfund>,
    goal_amount: u64,
    deadline_at: i64,
    metadata_hash: [u8; 32],
) -> Result<()> {
    require!(goal_amount > 0, CrowdfundRegistryError::ZeroGoal);
    let now = Clock::get()?.unix_timestamp;
    require!(deadline_at > now, CrowdfundRegistryError::DeadlineInPast);

    let crowdfund_id = ctx.accounts.creator_state.next_crowdfund_id;

    let cf = &mut ctx.accounts.crowdfund;
    cf.creator = ctx.accounts.creator.key();
    cf.creator_tid = ctx.accounts.creator_state.creator_tid;
    cf.crowdfund_id = crowdfund_id;
    cf.goal_amount = goal_amount;
    cf.total_pledged = 0;
    cf.pledge_count = 0;
    cf.deadline_at = deadline_at;
    cf.created_at = now;
    cf.status = CrowdfundStatus::Active as u8;
    cf.bump = ctx.bumps.crowdfund;
    cf.metadata_hash = metadata_hash;

    ctx.accounts.creator_state.next_crowdfund_id = crowdfund_id
        .checked_add(1)
        .expect("crowdfund_id u64 overflow is unreachable in practice");

    emit!(CrowdfundCreated {
        crowdfund: cf.key(),
        creator: cf.creator,
        creator_tid: cf.creator_tid,
        crowdfund_id,
        goal_amount,
        deadline_at,
    });

    Ok(())
}
