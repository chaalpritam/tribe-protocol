use anchor_lang::prelude::*;
use crate::state::{SocialProfile, Link};
use crate::errors::SocialGraphError;
use crate::events::Followed;

use super::init_profile::deserialize_tid_record;

#[derive(Accounts)]
pub struct Follow<'info> {
    /// CHECK: Cross-program TID record from tid-registry. Validated in handler.
    pub follower_tid_record: UncheckedAccount<'info>,

    /// Follower's social profile — following_count incremented.
    #[account(
        mut,
        seeds = [b"social_profile", &follower_tid_record.try_borrow_data()?[8..16]],
        bump = follower_profile.bump,
    )]
    pub follower_profile: Account<'info, SocialProfile>,

    /// Target's social profile — followers_count incremented.
    #[account(
        mut,
        seeds = [b"social_profile", following_profile.tid.to_le_bytes().as_ref()],
        bump = following_profile.bump,
    )]
    pub following_profile: Account<'info, SocialProfile>,

    /// The Link PDA — created on follow.
    #[account(
        init,
        payer = custody,
        space = Link::SIZE,
        seeds = [
            b"link",
            &follower_tid_record.try_borrow_data()?[8..16],
            following_profile.tid.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub link: Account<'info, Link>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Follow>) -> Result<()> {
    let tid_data = deserialize_tid_record(&ctx.accounts.follower_tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), SocialGraphError::UnauthorizedCustody);

    let follower_tid = tid_data.tid;
    let following_tid = ctx.accounts.following_profile.tid;

    require!(follower_tid != following_tid, SocialGraphError::CannotFollowSelf);

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
