use anchor_lang::prelude::*;
use crate::state::{UsernameRecord, TidUsername, MAX_USERNAME_LEN, REGISTRATION_DURATION};
use crate::errors::UsernameError;
use crate::events::UsernameRegistered;

pub const TID_REGISTRY_ID: Pubkey = pubkey!("4BSmJmRGQWKgioP9DG2bUuRS9U3V6soRauU7Nv6yGvHD");

pub struct TidRecordData {
    pub tid: u64,
    pub custody_address: Pubkey,
}

pub fn deserialize_tid_record(info: &AccountInfo) -> Result<TidRecordData> {
    require!(*info.owner == TID_REGISTRY_ID, UsernameError::UnauthorizedCustody);
    let data = info.try_borrow_data()?;
    require!(data.len() >= 8 + 8 + 32, UsernameError::UnauthorizedCustody);
    let tid = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let custody_address = Pubkey::try_from(&data[16..48]).unwrap();
    Ok(TidRecordData { tid, custody_address })
}

#[derive(Accounts)]
#[instruction(username: String)]
pub struct RegisterUsername<'info> {
    /// CHECK: Cross-program TID record from tid-registry. Validated in handler.
    pub tid_record: UncheckedAccount<'info>,

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
        space = TidUsername::SIZE,
        seeds = [b"tid_username", &tid_record.try_borrow_data()?[8..16]],
        bump,
    )]
    pub tid_username: Account<'info, TidUsername>,

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

    let tid_data = deserialize_tid_record(&ctx.accounts.tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), UsernameError::UnauthorizedCustody);

    let now = Clock::get()?.unix_timestamp;
    let username_bytes = username.as_bytes();
    let username_hash = anchor_lang::solana_program::hash::hash(username.to_lowercase().as_bytes()).to_bytes();

    let record = &mut ctx.accounts.username_record;
    let mut name_buf = [0u8; MAX_USERNAME_LEN];
    name_buf[..username_bytes.len()].copy_from_slice(username_bytes);
    record.username = name_buf;
    record.username_len = username_bytes.len() as u8;
    record.tid = tid_data.tid;
    record.registered_at = now;
    record.expiry = now + REGISTRATION_DURATION;
    record.bump = ctx.bumps.username_record;

    let tid_uname = &mut ctx.accounts.tid_username;
    tid_uname.username_hash = username_hash;
    tid_uname.bump = ctx.bumps.tid_username;

    emit!(UsernameRegistered {
        username,
        tid: record.tid,
        expiry: record.expiry,
    });

    Ok(())
}
