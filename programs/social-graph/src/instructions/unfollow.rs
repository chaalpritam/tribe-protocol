use anchor_lang::prelude::*;
use crate::state::{SocialProfile, Link};
use crate::errors::SocialGraphError;
use crate::events::Unfollowed;

use super::init_profile::deserialize_tid_record;

#[derive(Accounts)]
pub struct Unfollow<'info> {
    /// CHECK: Cross-program TID record from tid-registry. Validated in handler.
    pub follower_tid_record: UncheckedAccount<'info>,

    /// Follower's social profile — following_count decremented.
    #[account(
        mut,
        seeds = [b"social_profile", &follower_tid_record.try_borrow_data()?[8..16]],
        bump = follower_profile.bump,
    )]
    pub follower_profile: Account<'info, SocialProfile>,

    /// Target's social profile — followers_count decremented.
    #[account(
        mut,
        seeds = [b"social_profile", following_profile.tid.to_le_bytes().as_ref()],
        bump = following_profile.bump,
    )]
    pub following_profile: Account<'info, SocialProfile>,

    /// The Link PDA — closed on unfollow, rent reclaimed.
    #[account(
        mut,
        seeds = [
            b"link",
            &follower_tid_record.try_borrow_data()?[8..16],
            following_profile.tid.to_le_bytes().as_ref(),
        ],
        bump = link.bump,
        close = custody,
    )]
    pub link: Account<'info, Link>,

    #[account(mut)]
    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<Unfollow>) -> Result<()> {
    let tid_data = deserialize_tid_record(&ctx.accounts.follower_tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), SocialGraphError::UnauthorizedCustody);

    let follower_tid = tid_data.tid;
    let following_tid = ctx.accounts.following_profile.tid;

    // Update counters.
    ctx.accounts.follower_profile.following_count = ctx.accounts.follower_profile.following_count.saturating_sub(1);
    ctx.accounts.following_profile.followers_count = ctx.accounts.following_profile.followers_count.saturating_sub(1);

    emit!(Unfollowed {
        follower_tid,
        following_tid,
    });

    Ok(())
}
