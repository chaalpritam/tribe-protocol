use anchor_lang::prelude::*;
use crate::state::AppKeyRecord;
use crate::errors::AppKeyError;
use crate::events::AppKeyRotated;

use super::add_key::FidRecord;

#[derive(Accounts)]
#[instruction(new_app_pubkey: Pubkey)]
pub struct RotateAppKey<'info> {
    #[account(
        constraint = fid_record.custody_address == custody.key() @ AppKeyError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    /// Old app key — will be marked revoked.
    #[account(
        mut,
        seeds = [b"app_key", fid_record.fid.to_le_bytes().as_ref(), old_app_key_record.app_pubkey.as_ref()],
        bump = old_app_key_record.bump,
        constraint = !old_app_key_record.revoked @ AppKeyError::AlreadyRevoked,
    )]
    pub old_app_key_record: Account<'info, AppKeyRecord>,

    /// New app key — will be created.
    #[account(
        init,
        payer = custody,
        space = AppKeyRecord::SIZE,
        seeds = [b"app_key", fid_record.fid.to_le_bytes().as_ref(), new_app_pubkey.as_ref()],
        bump,
    )]
    pub new_app_key_record: Account<'info, AppKeyRecord>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RotateAppKey>,
    new_app_pubkey: Pubkey,
    scope: u8,
    expires_at: i64,
) -> Result<()> {
    require!(scope <= 3, AppKeyError::InvalidScope);

    let old = &mut ctx.accounts.old_app_key_record;
    let old_pubkey = old.app_pubkey;
    old.revoked = true;

    let new = &mut ctx.accounts.new_app_key_record;
    new.fid = ctx.accounts.fid_record.fid;
    new.app_pubkey = new_app_pubkey;
    new.scope = scope;
    new.created_at = Clock::get()?.unix_timestamp;
    new.expires_at = expires_at;
    new.revoked = false;
    new.bump = ctx.bumps.new_app_key_record;

    emit!(AppKeyRotated {
        fid: new.fid,
        old_app_pubkey: old_pubkey,
        new_app_pubkey,
        scope,
    });

    Ok(())
}
