use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::errors::CrowdfundRegistryError;
use crate::events::CrowdfundPledged;
use crate::state::{Crowdfund, CrowdfundStatus, Pledge as PledgeState};

#[derive(Accounts)]
pub struct Pledge<'info> {
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

    /// init_if_needed lets the same backer pledge multiple times,
    /// accumulating onto the same Pledge record.
    #[account(
        init_if_needed,
        payer = backer,
        space = PledgeState::SIZE,
        seeds = [b"pledge", crowdfund.key().as_ref(), backer.key().as_ref()],
        bump,
    )]
    pub pledge: Account<'info, PledgeState>,

    #[account(mut)]
    pub backer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Pledge>, backer_tid: u64, amount: u64) -> Result<()> {
    require!(amount > 0, CrowdfundRegistryError::ZeroPledge);

    let cf = &ctx.accounts.crowdfund;
    require!(
        cf.status == CrowdfundStatus::Active as u8,
        CrowdfundRegistryError::NotActive
    );
    let now = Clock::get()?.unix_timestamp;
    require!(now < cf.deadline_at, CrowdfundRegistryError::AfterDeadline);

    // Transfer pledged lamports into the Crowdfund PDA. System
    // Program transfers can credit any account; the source (backer)
    // is system-owned, which is what System CPI requires.
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.backer.to_account_info(),
            to: ctx.accounts.crowdfund.to_account_info(),
        },
    );
    system_program::transfer(cpi_ctx, amount)?;

    // Update / initialize the Pledge record.
    let pledge = &mut ctx.accounts.pledge;
    let is_first_time = pledge.amount == 0;
    if is_first_time {
        pledge.crowdfund = ctx.accounts.crowdfund.key();
        pledge.backer = ctx.accounts.backer.key();
        pledge.backer_tid = backer_tid;
        pledge.pledged_at = now;
        pledge.bump = ctx.bumps.pledge;
    }
    pledge.amount = pledge
        .amount
        .checked_add(amount)
        .expect("pledge amount overflow is unreachable in practice");

    // Update aggregates on the campaign.
    let cf = &mut ctx.accounts.crowdfund;
    cf.total_pledged = cf
        .total_pledged
        .checked_add(amount)
        .expect("total_pledged overflow is unreachable in practice");
    if is_first_time {
        cf.pledge_count = cf
            .pledge_count
            .checked_add(1)
            .expect("pledge_count overflow is unreachable in practice");
    }

    emit!(CrowdfundPledged {
        crowdfund: cf.key(),
        backer: pledge.backer,
        backer_tid,
        amount,
        total_pledged: cf.total_pledged,
        pledge_count: cf.pledge_count,
    });

    Ok(())
}
