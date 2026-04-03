use anchor_lang::prelude::*;
use crate::state::AppKeyRecord;
use crate::errors::AppKeyError;
use crate::events::AppKeyAdded;

pub const TID_REGISTRY_ID: Pubkey = pubkey!("4BSmJmRGQWKgioP9DG2bUuRS9U3V6soRauU7Nv6yGvHD");

/// Deserialized TID record fields from tid-registry.
pub struct TidRecordData {
    pub tid: u64,
    pub custody_address: Pubkey,
}

/// Deserialize a TidRecord from an AccountInfo owned by tid-registry.
/// Layout after 8-byte Anchor discriminator: u64 tid + Pubkey custody_address + ...
pub fn deserialize_tid_record(info: &AccountInfo) -> Result<TidRecordData> {
    require!(*info.owner == TID_REGISTRY_ID, AppKeyError::UnauthorizedCustody);
    let data = info.try_borrow_data()?;
    require!(data.len() >= 8 + 8 + 32, AppKeyError::UnauthorizedCustody);
    let tid = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let custody_address = Pubkey::try_from(&data[16..48]).unwrap();
    Ok(TidRecordData { tid, custody_address })
}

#[derive(Accounts)]
#[instruction(app_pubkey: Pubkey)]
pub struct AddAppKey<'info> {
    /// The TID record from tid-registry (cross-program read).
    /// CHECK: Manually deserialized and owner-checked in handler.
    pub tid_record: UncheckedAccount<'info>,

    #[account(
        init,
        payer = custody,
        space = AppKeyRecord::SIZE,
        seeds = [
            b"app_key",
            &tid_record.try_borrow_data()?[8..16],
            app_pubkey.as_ref(),
        ],
        bump,
    )]
    pub app_key_record: Account<'info, AppKeyRecord>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddAppKey>, app_pubkey: Pubkey, scope: u8, expires_at: i64) -> Result<()> {
    require!(scope <= 3, AppKeyError::InvalidScope);

    let tid_data = deserialize_tid_record(&ctx.accounts.tid_record)?;
    require!(tid_data.custody_address == ctx.accounts.custody.key(), AppKeyError::UnauthorizedCustody);

    let record = &mut ctx.accounts.app_key_record;
    record.tid = tid_data.tid;
    record.app_pubkey = app_pubkey;
    record.scope = scope;
    record.created_at = Clock::get()?.unix_timestamp;
    record.expires_at = expires_at;
    record.revoked = false;
    record.bump = ctx.bumps.app_key_record;

    emit!(AppKeyAdded {
        tid: record.tid,
        app_pubkey,
        scope,
        expires_at,
    });

    Ok(())
}
