use anchor_lang::prelude::*;
use crate::state::SocialProfile;
use crate::errors::SocialGraphError;
use crate::events::ProfileInitialized;

pub const TID_REGISTRY_ID: Pubkey = pubkey!("4BSmJmRGQWKgioP9DG2bUuRS9U3V6soRauU7Nv6yGvHD");

pub struct TidRecordData {
    pub tid: u64,
    pub custody_address: Pubkey,
}

pub fn deserialize_tid_record(info: &AccountInfo) -> Result<TidRecordData> {
    require!(*info.owner == TID_REGISTRY_ID, SocialGraphError::UnauthorizedCustody);
    let data = info.try_borrow_data()?;
    require!(data.len() >= 8 + 8 + 32, SocialGraphError::UnauthorizedCustody);
    let tid = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let custody_address = Pubkey::try_from(&data[16..48]).unwrap();
    Ok(TidRecordData { tid, custody_address })
}

#[derive(Accounts)]
pub struct InitProfile<'info> {
    /// CHECK: Cross-program TID record from tid-registry. Validated in handler.
    pub tid_record: UncheckedAccount<'info>,

    #[account(
        init,
        payer = custody,
        space = SocialProfile::SIZE,
        seeds = [b"social_profile", &tid_record.try_borrow_data()?[8..16]],
        bump,
    )]
    pub social_profile: Account<'info, SocialProfile>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitProfile>) -> Result<()> {
    let tid_data = deserialize_tid_record(&ctx.accounts.tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), SocialGraphError::UnauthorizedCustody);

    let profile = &mut ctx.accounts.social_profile;
    profile.tid = tid_data.tid;
    profile.following_count = 0;
    profile.followers_count = 0;
    profile.bump = ctx.bumps.social_profile;

    emit!(ProfileInitialized {
        tid: profile.tid,
    });

    Ok(())
}
