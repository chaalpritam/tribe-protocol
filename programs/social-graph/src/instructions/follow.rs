use anchor_lang::prelude::*;
use crate::state::{SocialProfile, Link};
use crate::errors::SocialGraphError;
use crate::events::Followed;

use super::init_profile::deserialize_fid_record;

#[derive(Accounts)]
pub struct Follow<'info> {
    /// CHECK: Cross-program FID record from fid-registry. Validated in handler.
    pub follower_fid_record: UncheckedAccount<'info>,

    /// Follower's social profile — following_count incremented.
    #[account(
        mut,
        seeds = [b"social_profile", &follower_fid_record.try_borrow_data()?[8..16]],
        bump = follower_profile.bump,
    )]
    pub follower_profile: Account<'info, SocialProfile>,

    /// Target's social profile — followers_count incremented.
    #[account(
        mut,
        seeds = [b"social_profile", following_profile.fid.to_le_bytes().as_ref()],
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
            &follower_fid_record.try_borrow_data()?[8..16],
            following_profile.fid.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub link: Account<'info, Link>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Follow>) -> Result<()> {
    let fid_data = deserialize_fid_record(&ctx.accounts.follower_fid_record)?;
    require!(fid_data.custody_address == ctx.accounts.custody.key(), SocialGraphError::UnauthorizedCustody);

    let follower_fid = fid_data.fid;
    let following_fid = ctx.accounts.following_profile.fid;

    require!(follower_fid != following_fid, SocialGraphError::CannotFollowSelf);

    // Create the link.
    let link = &mut ctx.accounts.link;
    link.follower_fid = follower_fid;
    link.following_fid = following_fid;
    link.created_at = Clock::get()?.unix_timestamp;
    link.bump = ctx.bumps.link;

    // Update counters.
    ctx.accounts.follower_profile.following_count += 1;
    ctx.accounts.following_profile.followers_count += 1;

    emit!(Followed {
        follower_fid,
        following_fid,
    });

    Ok(())
}
