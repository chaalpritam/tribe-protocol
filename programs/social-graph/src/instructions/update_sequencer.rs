use anchor_lang::prelude::*;
use crate::state::SequencerConfig;
use crate::errors::SocialGraphError;
use crate::events::SequencerRotated;

/// Rotate the sequencer authority. Only the admin recorded at
/// `init_sequencer` time can call this — protects against losing
/// the ER server wallet without bricking delegated follow/unfollow.
#[derive(Accounts)]
pub struct UpdateSequencer<'info> {
    #[account(
        mut,
        seeds = [b"sequencer_config"],
        bump = sequencer_config.bump,
    )]
    pub sequencer_config: Account<'info, SequencerConfig>,

    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateSequencer>, new_authority: Pubkey) -> Result<()> {
    require!(
        ctx.accounts.admin.key() == ctx.accounts.sequencer_config.admin,
        SocialGraphError::UnauthorizedAdmin
    );

    let previous_authority = ctx.accounts.sequencer_config.authority;
    ctx.accounts.sequencer_config.authority = new_authority;

    emit!(SequencerRotated {
        previous_authority,
        new_authority,
    });

    Ok(())
}
