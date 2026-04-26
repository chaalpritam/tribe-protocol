use anchor_lang::prelude::*;

use crate::events::KarmaAccountInitialized;
use crate::state::KarmaAccount;

#[derive(Accounts)]
#[instruction(tid: u64)]
pub struct InitKarmaAccount<'info> {
    #[account(
        init,
        payer = payer,
        space = KarmaAccount::SIZE,
        seeds = [b"karma".as_ref(), &tid.to_le_bytes()],
        bump,
    )]
    pub karma: Account<'info, KarmaAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitKarmaAccount>, tid: u64) -> Result<()> {
    let karma = &mut ctx.accounts.karma;
    karma.tid = tid;
    karma.tips_received_count = 0;
    karma.tips_received_lamports = 0;
    karma.tasks_completed_count = 0;
    karma.tasks_completed_reward_lamports = 0;
    karma.bump = ctx.bumps.karma;

    emit!(KarmaAccountInitialized {
        karma: karma.key(),
        tid,
    });

    Ok(())
}
