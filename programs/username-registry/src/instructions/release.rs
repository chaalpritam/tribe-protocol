use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, FidUsername};
use crate::errors::UsernameError;
use crate::events::UsernameReleased;

use super::register::FidRecord;

#[derive(Accounts)]
pub struct ReleaseUsername<'info> {
    #[account(
        constraint = fid_record.custody_address == custody.key() @ UsernameError::UnauthorizedCustody,
        constraint = fid_record.fid == username_record.fid @ UsernameError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    #[account(
        mut,
        close = custody,
    )]
    pub username_record: Account<'info, UsernameRecord>,

    #[account(
        mut,
        seeds = [b"fid_username", fid_record.fid.to_le_bytes().as_ref()],
        bump = fid_username.bump,
        close = custody,
    )]
    pub fid_username: Account<'info, FidUsername>,

    #[account(mut)]
    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<ReleaseUsername>) -> Result<()> {
    let record = &ctx.accounts.username_record;

    emit!(UsernameReleased {
        username: record.username_str().to_string(),
        fid: record.fid,
    });

    Ok(())
}
