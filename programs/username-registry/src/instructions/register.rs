use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, FidUsername, MAX_USERNAME_LEN, REGISTRATION_DURATION};
use crate::errors::UsernameError;
use crate::events::UsernameRegistered;

/// FID record from fid-registry (cross-program read).
#[account]
pub struct FidRecord {
    pub fid: u64,
    pub custody_address: Pubkey,
    pub recovery_address: Pubkey,
    pub registered_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct RegisterUsername<'info> {
    #[account(
        constraint = fid_record.custody_address == custody.key() @ UsernameError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    #[account(
        init,
        payer = custody,
        space = UsernameRecord::SIZE,
        seeds = [b"username", username.to_lowercase().as_bytes()],
        bump,
    )]
    pub username_record: Account<'info, UsernameRecord>,

    #[account(
        init,
        payer = custody,
        space = FidUsername::SIZE,
        seeds = [b"fid_username", fid_record.fid.to_le_bytes().as_ref()],
        bump,
    )]
    pub fid_username: Account<'info, FidUsername>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterUsername>, username: String) -> Result<()> {
    require!(username.len() >= 1, UsernameError::UsernameTooShort);
    require!(username.len() <= MAX_USERNAME_LEN, UsernameError::UsernameTooLong);
    require!(
        username.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
        UsernameError::InvalidCharacters
    );

    let now = Clock::get()?.unix_timestamp;
    let username_bytes = username.as_bytes();
    let username_hash = anchor_lang::solana_program::hash::hash(username.to_lowercase().as_bytes()).to_bytes();

    let record = &mut ctx.accounts.username_record;
    let mut name_buf = [0u8; MAX_USERNAME_LEN];
    name_buf[..username_bytes.len()].copy_from_slice(username_bytes);
    record.username = name_buf;
    record.username_len = username_bytes.len() as u8;
    record.fid = ctx.accounts.fid_record.fid;
    record.registered_at = now;
    record.expiry = now + REGISTRATION_DURATION;
    record.bump = ctx.bumps.username_record;

    let fid_uname = &mut ctx.accounts.fid_username;
    fid_uname.username_hash = username_hash;
    fid_uname.bump = ctx.bumps.fid_username;

    emit!(UsernameRegistered {
        username,
        fid: record.fid,
        expiry: record.expiry,
    });

    Ok(())
}
