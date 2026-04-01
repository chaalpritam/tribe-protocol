use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, REGISTRATION_DURATION};
use crate::errors::UsernameError;
use crate::events::UsernameRenewed;

use super::register::FidRecord;

#[derive(Accounts)]
pub struct RenewUsername<'info> {
    #[account(
        constraint = fid_record.custody_address == custody.key() @ UsernameError::UnauthorizedCustody,
        constraint = fid_record.fid == username_record.fid @ UsernameError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    #[account(mut)]
    pub username_record: Account<'info, UsernameRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<RenewUsername>) -> Result<()> {
    let record = &mut ctx.accounts.username_record;
    let now = Clock::get()?.unix_timestamp;

    // Extend from current expiry or from now (whichever is later).
    let base = if record.expiry > now { record.expiry } else { now };
    record.expiry = base + REGISTRATION_DURATION;

    emit!(UsernameRenewed {
        username: record.username_str().to_string(),
        fid: record.fid,
        new_expiry: record.expiry,
    });

    Ok(())
}
