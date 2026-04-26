use anchor_lang::prelude::*;

use crate::errors::CrowdfundRegistryError;
use crate::events::CrowdfundRefunded;
use crate::state::{Crowdfund, CrowdfundStatus, Pledge as PledgeState};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(
        mut,
        seeds = [
            b"crowdfund",
            crowdfund.creator.as_ref(),
            &crowdfund.crowdfund_id.to_le_bytes(),
        ],
        bump = crowdfund.bump,
    )]
    pub crowdfund: Account<'info, Crowdfund>,

    /// Closes on success — Pledge rent goes back to the backer.
    #[account(
        mut,
        seeds = [b"pledge", crowdfund.key().as_ref(), backer.key().as_ref()],
        bump = pledge.bump,
        has_one = backer,
        close = backer,
    )]
    pub pledge: Account<'info, PledgeState>,

    #[account(mut)]
    pub backer: Signer<'info>,
}

pub fn handler(ctx: Context<Refund>) -> Result<()> {
    let cf = &ctx.accounts.crowdfund;
    require!(
        cf.status != CrowdfundStatus::Succeeded as u8,
        CrowdfundRegistryError::GoalMet,
    );
    let now = Clock::get()?.unix_timestamp;
    require!(now >= cf.deadline_at, CrowdfundRegistryError::BeforeDeadline);
    require!(
        cf.total_pledged < cf.goal_amount,
        CrowdfundRegistryError::GoalMet
    );

    let amount = ctx.accounts.pledge.amount;

    // Move the pledged lamports out of the Crowdfund PDA back to
    // the backer via direct lamport mutation (the program owns the
    // PDA). The Pledge account is closed by the `close = backer`
    // constraint above, returning its rent to the backer too.
    let cf_info = ctx.accounts.crowdfund.to_account_info();
    let backer_info = ctx.accounts.backer.to_account_info();
    **cf_info.try_borrow_mut_lamports()? = cf_info
        .lamports()
        .checked_sub(amount)
        .ok_or(error!(CrowdfundRegistryError::GoalMet))?;
    **backer_info.try_borrow_mut_lamports()? = backer_info
        .lamports()
        .checked_add(amount)
        .expect("backer lamport credit overflow unreachable in practice");

    let cf = &mut ctx.accounts.crowdfund;
    cf.total_pledged = cf.total_pledged.saturating_sub(amount);
    cf.pledge_count = cf.pledge_count.saturating_sub(1);
    // First refund flips Active → Failed; subsequent refunds are idempotent.
    if cf.status == CrowdfundStatus::Active as u8 {
        cf.status = CrowdfundStatus::Failed as u8;
    }

    emit!(CrowdfundRefunded {
        crowdfund: cf.key(),
        backer: ctx.accounts.backer.key(),
        amount,
    });

    Ok(())
}
