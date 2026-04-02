use anchor_lang::prelude::*;
use crate::state::UsernameRecord;
use crate::errors::UsernameError;
use crate::events::UsernameTransferred;

use super::register::deserialize_fid_record;

#[derive(Accounts)]
pub struct TransferUsername<'info> {
    /// CHECK: Cross-program FID record from fid-registry. Validated in handler.
    pub fid_record: UncheckedAccount<'info>,

    #[account(mut)]
    pub username_record: Account<'info, UsernameRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<TransferUsername>, new_fid: u64) -> Result<()> {
    let fid_data = deserialize_fid_record(&ctx.accounts.fid_record)?;
    require!(fid_data.custody_address == ctx.accounts.custody.key(), UsernameError::UnauthorizedCustody);
    require!(fid_data.fid == ctx.accounts.username_record.fid, UsernameError::UnauthorizedCustody);

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
