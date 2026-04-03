use anchor_lang::prelude::*;
use crate::state::UsernameRecord;
use crate::errors::UsernameError;
use crate::events::UsernameTransferred;

use super::register::deserialize_tid_record;

#[derive(Accounts)]
pub struct TransferUsername<'info> {
    /// CHECK: Cross-program TID record from tid-registry. Validated in handler.
    pub tid_record: UncheckedAccount<'info>,

    #[account(mut)]
    pub username_record: Account<'info, UsernameRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<TransferUsername>, new_tid: u64) -> Result<()> {
    let tid_data = deserialize_tid_record(&ctx.accounts.tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), UsernameError::UnauthorizedCustody);
    require!(tid_data.tid == ctx.accounts.username_record.tid, UsernameError::UnauthorizedCustody);

    let record = &mut ctx.accounts.username_record;
    let now = Clock::get()?.unix_timestamp;
    require!(!record.is_expired(now), UsernameError::Expired);

    let old_tid = record.tid;
    record.tid = new_tid;

    emit!(UsernameTransferred {
        username: record.username_str().to_string(),
        old_tid,
        new_tid,
    });

    Ok(())
}
