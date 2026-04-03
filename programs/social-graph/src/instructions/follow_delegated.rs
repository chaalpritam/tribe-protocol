use anchor_lang::prelude::*;
use crate::state::{SocialProfile, Link, SequencerConfig};
use crate::errors::SocialGraphError;
use crate::events::Followed;

/// Follow via the sequencer authority — used by the ER server for batched settlement.
/// Same logic as `follow`, but the signer is the sequencer instead of the user's custody wallet.
#[derive(Accounts)]
#[instruction(follower_tid: u64, following_tid: u64)]
pub struct FollowDelegated<'info> {
    /// Sequencer config — validates the authority signer.
    #[account(
        seeds = [b"sequencer_config"],
        bump = sequencer_config.bump,
    )]
    pub sequencer_config: Account<'info, SequencerConfig>,

    /// Follower's social profile — following_count incremented.
    #[account(
        mut,
        seeds = [b"social_profile", follower_tid.to_le_bytes().as_ref()],
        bump = follower_profile.bump,
    )]
    pub follower_profile: Account<'info, SocialProfile>,

    /// Target's social profile — followers_count incremented.
    #[account(
        mut,
        seeds = [b"social_profile", following_tid.to_le_bytes().as_ref()],
        bump = following_profile.bump,
    )]
    pub following_profile: Account<'info, SocialProfile>,

    /// The Link PDA — created on follow.
    #[account(
        init,
        payer = authority,
        space = Link::SIZE,
        seeds = [b"link", follower_tid.to_le_bytes().as_ref(), following_tid.to_le_bytes().as_ref()],
        bump,
    )]
    pub link: Account<'info, Link>,

    /// The sequencer authority — must match sequencer_config.authority.
    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FollowDelegated>, follower_tid: u64, following_tid: u64) -> Result<()> {
    require!(
        ctx.accounts.authority.key() == ctx.accounts.sequencer_config.authority,
        SocialGraphError::UnauthorizedSequencer
    );
    require!(follower_tid != following_tid, SocialGraphError::CannotFollowSelf);

    // Verify profile TIDs match the arguments.
    require!(ctx.accounts.follower_profile.tid == follower_tid, SocialGraphError::UnauthorizedCustody);
    require!(ctx.accounts.following_profile.tid == following_tid, SocialGraphError::UnauthorizedCustody);

    // Create the link.
    let link = &mut ctx.accounts.link;
    link.follower_tid = follower_tid;
    link.following_tid = following_tid;
    link.created_at = Clock::get()?.unix_timestamp;
    link.bump = ctx.bumps.link;

    // Update counters.
    ctx.accounts.follower_profile.following_count += 1;
    ctx.accounts.following_profile.followers_count += 1;

    emit!(Followed {
        follower_tid,
        following_tid,
    });

    Ok(())
}
