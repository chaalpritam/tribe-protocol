use anchor_lang::prelude::*;
use crate::state::UsernameRecord;
use crate::errors::UsernameError;
use crate::events::UsernameTransferred;

use super::register::FidRecord;

#[derive(Accounts)]
pub struct TransferUsername<'info> {
    /// Current owner's FID record.
    #[account(
        constraint = fid_record.custody_address == custody.key() @ UsernameError::UnauthorizedCustody,
        constraint = fid_record.fid == username_record.fid @ UsernameError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    #[account(mut)]
    pub username_record: Account<'info, UsernameRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<TransferUsername>, new_fid: u64) -> Result<()> {
    let record = &mut ctx.accounts.username_record;
    let now = Clock::get()?.unix_timestamp;
    require!(!record.is_expired(now), UsernameError::Expired);

    let old_fid = record.fid;
    record.fid = new_fid;

    emit!(UsernameTransferred {
        username: record.username_str().to_string(),
        old_fid,
        new_fid,
    });

    Ok(())
}
