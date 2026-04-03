use anchor_lang::prelude::*;
use crate::state::AppKeyRecord;
use crate::errors::AppKeyError;
use crate::events::AppKeyRevoked;

use super::add_key::deserialize_tid_record;

#[derive(Accounts)]
pub struct RevokeAppKey<'info> {
    /// CHECK: Cross-program TID record from tid-registry. Validated in handler.
    pub tid_record: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"app_key", &tid_record.try_borrow_data()?[8..16], app_key_record.app_pubkey.as_ref()],
        bump = app_key_record.bump,
        constraint = !app_key_record.revoked @ AppKeyError::AlreadyRevoked,
    )]
    pub app_key_record: Account<'info, AppKeyRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<RevokeAppKey>) -> Result<()> {
    let tid_data = deserialize_tid_record(&ctx.accounts.tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), AppKeyError::UnauthorizedCustody);

    let record = &mut ctx.accounts.app_key_record;
    record.revoked = true;

    emit!(AppKeyRevoked {
        tid: record.tid,
        app_pubkey: record.app_pubkey,
    });

    Ok(())
}
