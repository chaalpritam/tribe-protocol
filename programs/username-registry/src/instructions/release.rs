use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, FidUsername};
use crate::errors::UsernameError;
use crate::events::UsernameReleased;

use super::register::deserialize_fid_record;

#[derive(Accounts)]
pub struct ReleaseUsername<'info> {
    /// CHECK: Cross-program FID record from fid-registry. Validated in handler.
    pub fid_record: UncheckedAccount<'info>,

    #[account(
        mut,
        close = custody,
    )]
    pub username_record: Account<'info, UsernameRecord>,

    #[account(
        mut,
        seeds = [b"fid_username", &fid_record.try_borrow_data()?[8..16]],
        bump = fid_username.bump,
        close = custody,
    )]
    pub fid_username: Account<'info, FidUsername>,

    #[account(mut)]
    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<ReleaseUsername>) -> Result<()> {
    let fid_data = deserialize_fid_record(&ctx.accounts.fid_record)?;
    require!(fid_data.custody_address == ctx.accounts.custody.key(), UsernameError::UnauthorizedCustody);
    require!(fid_data.fid == ctx.accounts.username_record.fid, UsernameError::UnauthorizedCustody);

    let record = &ctx.accounts.username_record;
    let username_bytes = &record.username[..record.username_len as usize];
    let username = String::from_utf8_lossy(username_bytes).to_string();

    emit!(UsernameReleased {
        username,
        fid: record.fid,
    });

    Ok(())
}
