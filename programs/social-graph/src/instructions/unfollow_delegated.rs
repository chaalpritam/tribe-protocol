use anchor_lang::prelude::*;
use crate::state::{SocialProfile, Link, SequencerConfig};
use crate::errors::SocialGraphError;
use crate::events::Unfollowed;

/// Unfollow via the sequencer authority — used by the ER server for batched settlement.
/// Same logic as `unfollow`, but the signer is the sequencer. Rent reclaimed to authority.
#[derive(Accounts)]
#[instruction(follower_tid: u64, following_tid: u64)]
pub struct UnfollowDelegated<'info> {
    /// Sequencer config — validates the authority signer.
    #[account(
        seeds = [b"sequencer_config"],
        bump = sequencer_config.bump,
    )]
    pub sequencer_config: Account<'info, SequencerConfig>,

    /// Follower's social profile — following_count decremented.
    #[account(
        mut,
        seeds = [b"social_profile", follower_tid.to_le_bytes().as_ref()],
        bump = follower_profile.bump,
    )]
    pub follower_profile: Account<'info, SocialProfile>,

    /// Target's social profile — followers_count decremented.
    #[account(
        mut,
        seeds = [b"social_profile", following_tid.to_le_bytes().as_ref()],
        bump = following_profile.bump,
    )]
    pub following_profile: Account<'info, SocialProfile>,

    /// The Link PDA — closed on unfollow, rent reclaimed to authority.
    #[account(
        mut,
        seeds = [b"link", follower_tid.to_le_bytes().as_ref(), following_tid.to_le_bytes().as_ref()],
        bump = link.bump,
        close = authority,
    )]
    pub link: Account<'info, Link>,

    /// The sequencer authority — must match sequencer_config.authority.
    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<UnfollowDelegated>, follower_tid: u64, following_tid: u64) -> Result<()> {
    require!(
        ctx.accounts.authority.key() == ctx.accounts.sequencer_config.authority,
        SocialGraphError::UnauthorizedSequencer
    );

    // Verify profile TIDs match.
    require!(ctx.accounts.follower_profile.tid == follower_tid, SocialGraphError::UnauthorizedCustody);
    require!(ctx.accounts.following_profile.tid == following_tid, SocialGraphError::UnauthorizedCustody);

    // Update counters.
    ctx.accounts.follower_profile.following_count = ctx.accounts.follower_profile.following_count.saturating_sub(1);
    ctx.accounts.following_profile.followers_count = ctx.accounts.following_profile.followers_count.saturating_sub(1);

    emit!(Unfollowed {
        follower_tid,
        following_tid,
    });

    Ok(())
}
