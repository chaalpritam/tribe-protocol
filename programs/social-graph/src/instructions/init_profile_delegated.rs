use anchor_lang::prelude::*;
use crate::state::{SocialProfile, SequencerConfig};
use crate::errors::SocialGraphError;
use crate::events::ProfileInitialized;

/// Initialize a social profile via the sequencer authority.
/// Allows the ER server to create profiles during settlement.
#[derive(Accounts)]
#[instruction(tid: u64)]
pub struct InitProfileDelegated<'info> {
    /// Sequencer config — validates the authority signer.
    #[account(
        seeds = [b"sequencer_config"],
        bump = sequencer_config.bump,
    )]
    pub sequencer_config: Account<'info, SequencerConfig>,

    #[account(
        init,
        payer = authority,
        space = SocialProfile::SIZE,
        seeds = [b"social_profile", tid.to_le_bytes().as_ref()],
        bump,
    )]
    pub social_profile: Account<'info, SocialProfile>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitProfileDelegated>, tid: u64) -> Result<()> {
    require!(
        ctx.accounts.authority.key() == ctx.accounts.sequencer_config.authority,
        SocialGraphError::UnauthorizedSequencer
    );

    let profile = &mut ctx.accounts.social_profile;
    profile.tid = tid;
    profile.following_count = 0;
    profile.followers_count = 0;
    profile.bump = ctx.bumps.social_profile;

    emit!(ProfileInitialized { tid });

    Ok(())
}
