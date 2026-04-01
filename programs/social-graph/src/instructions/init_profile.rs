use anchor_lang::prelude::*;
use crate::state::SocialProfile;
use crate::errors::SocialGraphError;
use crate::events::ProfileInitialized;

/// FID record from fid-registry (cross-program read).
#[account]
pub struct FidRecord {
    pub fid: u64,
    pub custody_address: Pubkey,
    pub recovery_address: Pubkey,
    pub registered_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    #[account(
        constraint = fid_record.custody_address == custody.key() @ SocialGraphError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    #[account(
        init,
        payer = custody,
        space = SocialProfile::SIZE,
        seeds = [b"social_profile", fid_record.fid.to_le_bytes().as_ref()],
        bump,
    )]
    pub social_profile: Account<'info, SocialProfile>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitProfile>) -> Result<()> {
    let profile = &mut ctx.accounts.social_profile;
    profile.fid = ctx.accounts.fid_record.fid;
    profile.following_count = 0;
    profile.followers_count = 0;
    profile.bump = ctx.bumps.social_profile;

    emit!(ProfileInitialized {
        fid: profile.fid,
    });

    Ok(())
}
