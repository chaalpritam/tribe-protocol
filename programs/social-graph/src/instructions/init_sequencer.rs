use anchor_lang::prelude::*;
use crate::state::SequencerConfig;

#[derive(Accounts)]
pub struct InitSequencer<'info> {
    #[account(
        init,
        payer = admin,
        space = SequencerConfig::SIZE,
        seeds = [b"sequencer_config"],
        bump,
    )]
    pub sequencer_config: Account<'info, SequencerConfig>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitSequencer>, authority: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.sequencer_config;
    config.authority = authority;
    config.admin = ctx.accounts.admin.key();
    config.bump = ctx.bumps.sequencer_config;
    Ok(())
}
