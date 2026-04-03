use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, TidUsername};
use crate::errors::UsernameError;
use crate::events::UsernameReleased;

use super::register::deserialize_tid_record;

#[derive(Accounts)]
pub struct ReleaseUsername<'info> {
    /// CHECK: Cross-program TID record from tid-registry. Validated in handler.
    pub tid_record: UncheckedAccount<'info>,

    #[account(
        mut,
        close = custody,
    )]
    pub username_record: Account<'info, UsernameRecord>,

    #[account(
        mut,
        seeds = [b"tid_username", &tid_record.try_borrow_data()?[8..16]],
        bump = tid_username.bump,
        close = custody,
    )]
    pub tid_username: Account<'info, TidUsername>,

    #[account(mut)]
    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<ReleaseUsername>) -> Result<()> {
    let tid_data = deserialize_tid_record(&ctx.accounts.tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), UsernameError::UnauthorizedCustody);
    require!(tid_data.tid == ctx.accounts.username_record.tid, UsernameError::UnauthorizedCustody);

    let record = &ctx.accounts.username_record;
    let username_bytes = &record.username[..record.username_len as usize];
    let username = String::from_utf8_lossy(username_bytes).to_string();

    emit!(UsernameReleased {
        username,
        tid: record.tid,
    });

    Ok(())
}
