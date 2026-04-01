use anchor_lang::prelude::*;
use crate::state::AppKeyRecord;
use crate::errors::AppKeyError;
use crate::events::AppKeyRevoked;

// Re-use FidRecord definition for cross-program read.
use super::add_key::FidRecord;

#[derive(Accounts)]
pub struct RevokeAppKey<'info> {
    #[account(
        constraint = fid_record.custody_address == custody.key() @ AppKeyError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    #[account(
        mut,
        seeds = [b"app_key", fid_record.fid.to_le_bytes().as_ref(), app_key_record.app_pubkey.as_ref()],
        bump = app_key_record.bump,
        constraint = !app_key_record.revoked @ AppKeyError::AlreadyRevoked,
    )]
    pub app_key_record: Account<'info, AppKeyRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<RevokeAppKey>) -> Result<()> {
    let record = &mut ctx.accounts.app_key_record;
    record.revoked = true;

    emit!(AppKeyRevoked {
        fid: record.fid,
        app_pubkey: record.app_pubkey,
    });

    Ok(())
}
