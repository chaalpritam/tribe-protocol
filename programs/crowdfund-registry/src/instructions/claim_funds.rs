use anchor_lang::prelude::*;

use crate::errors::CrowdfundRegistryError;
use crate::events::CrowdfundClaimed;
use crate::state::{Crowdfund, CrowdfundStatus};

#[derive(Accounts)]
pub struct ClaimFunds<'info> {
    #[account(
        mut,
        seeds = [
            b"crowdfund",
            creator.key().as_ref(),
            &crowdfund.crowdfund_id.to_le_bytes(),
        ],
        bump = crowdfund.bump,
        has_one = creator,
    )]
    pub crowdfund: Account<'info, Crowdfund>,

    #[account(mut)]
    pub creator: Signer<'info>,
}

pub fn handler(ctx: Context<ClaimFunds>) -> Result<()> {
    let cf = &ctx.accounts.crowdfund;
    require!(
        cf.status == CrowdfundStatus::Active as u8,
        CrowdfundRegistryError::NotActive
    );

    let now = Clock::get()?.unix_timestamp;
    require!(now >= cf.deadline_at, CrowdfundRegistryError::BeforeDeadline);
    require!(
        cf.total_pledged >= cf.goal_amount,
        CrowdfundRegistryError::GoalNotMet
    );

    let amount = cf.total_pledged;

    // The Crowdfund PDA is owned by this program, so we move lamports
    // out by directly mutating account balances. The PDA's own
    // rent-exempt minimum stays untouched because `total_pledged`
    // tracks only pledged amounts, never rent.
    let cf_info = ctx.accounts.crowdfund.to_account_info();
    let creator_info = ctx.accounts.creator.to_account_info();
    **cf_info.try_borrow_mut_lamports()? = cf_info
        .lamports()
        .checked_sub(amount)
        .ok_or(error!(CrowdfundRegistryError::GoalNotMet))?;
    **creator_info.try_borrow_mut_lamports()? = creator_info
        .lamports()
        .checked_add(amount)
        .expect("creator lamport credit overflow unreachable in practice");

    let cf = &mut ctx.accounts.crowdfund;
    cf.total_pledged = 0;
    cf.status = CrowdfundStatus::Succeeded as u8;

    emit!(CrowdfundClaimed {
        crowdfund: cf.key(),
        creator: cf.creator,
        total_pledged: amount,
    });

    Ok(())
}
