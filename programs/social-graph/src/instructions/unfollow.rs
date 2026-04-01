use anchor_lang::prelude::*;
use crate::state::{SocialProfile, Link};
use crate::errors::SocialGraphError;
use crate::events::Unfollowed;

use super::init_profile::FidRecord;

#[derive(Accounts)]
pub struct Unfollow<'info> {
    /// Follower's FID record — proves custody ownership.
    #[account(
        constraint = follower_fid_record.custody_address == custody.key() @ SocialGraphError::UnauthorizedCustody,
    )]
    pub follower_fid_record: Account<'info, FidRecord>,

    /// Follower's social profile — following_count decremented.
    #[account(
        mut,
        seeds = [b"social_profile", follower_fid_record.fid.to_le_bytes().as_ref()],
        bump = follower_profile.bump,
    )]
    pub follower_profile: Account<'info, SocialProfile>,

    /// Target's social profile — followers_count decremented.
    #[account(
        mut,
        seeds = [b"social_profile", following_profile.fid.to_le_bytes().as_ref()],
        bump = following_profile.bump,
    )]
    pub following_profile: Account<'info, SocialProfile>,

    /// The Link PDA — closed on unfollow, rent reclaimed.
    #[account(
        mut,
        seeds = [
            b"link",
            follower_fid_record.fid.to_le_bytes().as_ref(),
            following_profile.fid.to_le_bytes().as_ref(),
        ],
        bump = link.bump,
        close = custody,
    )]
    pub link: Account<'info, Link>,

    #[account(mut)]
    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<Unfollow>) -> Result<()> {
    let follower_fid = ctx.accounts.follower_fid_record.fid;
    let following_fid = ctx.accounts.following_profile.fid;

    // Update counters.
    ctx.accounts.follower_profile.following_count = ctx.accounts.follower_profile.following_count.saturating_sub(1);
    ctx.accounts.following_profile.followers_count = ctx.accounts.following_profile.followers_count.saturating_sub(1);

    emit!(Unfollowed {
        follower_fid,
        following_fid,
    });

    Ok(())
}
