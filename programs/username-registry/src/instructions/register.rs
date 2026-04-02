use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, FidUsername, MAX_USERNAME_LEN, REGISTRATION_DURATION};
use crate::errors::UsernameError;
use crate::events::UsernameRegistered;

pub const FID_REGISTRY_ID: Pubkey = pubkey!("4BSmJmRGQWKgioP9DG2bUuRS9U3V6soRauU7Nv6yGvHD");

pub struct FidRecordData {
    pub fid: u64,
    pub custody_address: Pubkey,
}

pub fn deserialize_fid_record(info: &AccountInfo) -> Result<FidRecordData> {
    require!(*info.owner == FID_REGISTRY_ID, UsernameError::UnauthorizedCustody);
    let data = info.try_borrow_data()?;
    require!(data.len() >= 8 + 8 + 32, UsernameError::UnauthorizedCustody);
    let fid = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let custody_address = Pubkey::try_from(&data[16..48]).unwrap();
    Ok(FidRecordData { fid, custody_address })
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct RegisterUsername<'info> {
    /// CHECK: Cross-program FID record from fid-registry. Validated in handler.
    pub fid_record: UncheckedAccount<'info>,

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
        seeds = [b"fid_username", &fid_record.try_borrow_data()?[8..16]],
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

    let fid_data = deserialize_fid_record(&ctx.accounts.fid_record)?;
    require!(fid_data.custody_address == ctx.accounts.custody.key(), UsernameError::UnauthorizedCustody);

    let now = Clock::get()?.unix_timestamp;
    let username_bytes = username.as_bytes();
    let username_hash = anchor_lang::solana_program::hash::hash(username.to_lowercase().as_bytes()).to_bytes();

    let record = &mut ctx.accounts.username_record;
    let mut name_buf = [0u8; MAX_USERNAME_LEN];
    name_buf[..username_bytes.len()].copy_from_slice(username_bytes);
    record.username = name_buf;
    record.username_len = username_bytes.len() as u8;
    record.fid = fid_data.fid;
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
