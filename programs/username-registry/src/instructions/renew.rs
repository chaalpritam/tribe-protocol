use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, REGISTRATION_DURATION};
use crate::errors::UsernameError;
use crate::events::UsernameRenewed;

use super::register::deserialize_fid_record;

#[derive(Accounts)]
pub struct RenewUsername<'info> {
    /// CHECK: Cross-program FID record from fid-registry. Validated in handler.
    pub fid_record: UncheckedAccount<'info>,

    #[account(mut)]
    pub username_record: Account<'info, UsernameRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<RenewUsername>) -> Result<()> {
    let fid_data = deserialize_fid_record(&ctx.accounts.fid_record)?;
    require!(fid_data.custody_address == ctx.accounts.custody.key(), UsernameError::UnauthorizedCustody);
    require!(fid_data.fid == ctx.accounts.username_record.fid, UsernameError::UnauthorizedCustody);

    let now = Clock::get()?.unix_timestamp;
    let record = &mut ctx.accounts.username_record;

    let username_bytes = &record.username[..record.username_len as usize];
    let username = String::from_utf8_lossy(username_bytes).to_string();

    // Extend from current expiry if not expired, otherwise from now
    let base = if record.expiry > now { record.expiry } else { now };
    record.expiry = base + REGISTRATION_DURATION;

    emit!(UsernameRenewed {
        username,
        fid: record.fid,
        new_expiry: record.expiry,
    });

    Ok(())
}
