use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, REGISTRATION_DURATION};
use crate::errors::UsernameError;
use crate::events::UsernameRenewed;

use super::register::deserialize_tid_record;

#[derive(Accounts)]
pub struct RenewUsername<'info> {
    /// CHECK: Cross-program TID record from tid-registry. Validated in handler.
    pub tid_record: UncheckedAccount<'info>,

    #[account(mut)]
    pub username_record: Account<'info, UsernameRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<RenewUsername>) -> Result<()> {
    let tid_data = deserialize_tid_record(&ctx.accounts.tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), UsernameError::UnauthorizedCustody);
    require!(tid_data.tid == ctx.accounts.username_record.tid, UsernameError::UnauthorizedCustody);

    let now = Clock::get()?.unix_timestamp;
    let record = &mut ctx.accounts.username_record;

    let username_bytes = &record.username[..record.username_len as usize];
    let username = String::from_utf8_lossy(username_bytes).to_string();

    // Extend from current expiry if not expired, otherwise from now
    let base = if record.expiry > now { record.expiry } else { now };
    record.expiry = base + REGISTRATION_DURATION;

    emit!(UsernameRenewed {
        username,
        tid: record.tid,
        new_expiry: record.expiry,
    });

    Ok(())
}
