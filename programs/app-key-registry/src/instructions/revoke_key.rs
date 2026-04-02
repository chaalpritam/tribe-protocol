use anchor_lang::prelude::*;
use crate::state::AppKeyRecord;
use crate::errors::AppKeyError;
use crate::events::AppKeyRevoked;

use super::add_key::deserialize_fid_record;

#[derive(Accounts)]
pub struct RevokeAppKey<'info> {
    /// CHECK: Cross-program FID record from fid-registry. Validated in handler.
    pub fid_record: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"app_key", &fid_record.try_borrow_data()?[8..16], app_key_record.app_pubkey.as_ref()],
        bump = app_key_record.bump,
        constraint = !app_key_record.revoked @ AppKeyError::AlreadyRevoked,
    )]
    pub app_key_record: Account<'info, AppKeyRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<RevokeAppKey>) -> Result<()> {
    let fid_data = deserialize_fid_record(&ctx.accounts.fid_record)?;
    require!(fid_data.custody_address == ctx.accounts.custody.key(), AppKeyError::UnauthorizedCustody);

    let record = &mut ctx.accounts.app_key_record;
    record.revoked = true;

    emit!(AppKeyRevoked {
        fid: record.fid,
        app_pubkey: record.app_pubkey,
    });

    Ok(())
}
