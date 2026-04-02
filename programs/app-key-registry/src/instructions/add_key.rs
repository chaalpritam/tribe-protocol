use anchor_lang::prelude::*;
use crate::state::AppKeyRecord;
use crate::errors::AppKeyError;
use crate::events::AppKeyAdded;

pub const FID_REGISTRY_ID: Pubkey = pubkey!("4BSmJmRGQWKgioP9DG2bUuRS9U3V6soRauU7Nv6yGvHD");

/// Deserialized FID record fields from fid-registry.
pub struct FidRecordData {
    pub fid: u64,
    pub custody_address: Pubkey,
}

/// Deserialize a FidRecord from an AccountInfo owned by fid-registry.
/// Layout after 8-byte Anchor discriminator: u64 fid + Pubkey custody_address + ...
pub fn deserialize_fid_record(info: &AccountInfo) -> Result<FidRecordData> {
    require!(*info.owner == FID_REGISTRY_ID, AppKeyError::UnauthorizedCustody);
    let data = info.try_borrow_data()?;
    require!(data.len() >= 8 + 8 + 32, AppKeyError::UnauthorizedCustody);
    let fid = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let custody_address = Pubkey::try_from(&data[16..48]).unwrap();
    Ok(FidRecordData { fid, custody_address })
}

#[derive(Accounts)]
#[instruction(app_pubkey: Pubkey)]
pub struct AddAppKey<'info> {
    /// The FID record from fid-registry (cross-program read).
    /// CHECK: Manually deserialized and owner-checked in handler.
    pub fid_record: UncheckedAccount<'info>,

    #[account(
        init,
        payer = custody,
        space = AppKeyRecord::SIZE,
        seeds = [
            b"app_key",
            &fid_record.try_borrow_data()?[8..16],
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

    let fid_data = deserialize_fid_record(&ctx.accounts.fid_record)?;
    require!(fid_data.custody_address == ctx.accounts.custody.key(), AppKeyError::UnauthorizedCustody);

    let record = &mut ctx.accounts.app_key_record;
    record.fid = fid_data.fid;
    record.app_pubkey = app_pubkey;
    record.scope = scope;
    record.created_at = Clock::get()?.unix_timestamp;
    record.expires_at = expires_at;
    record.revoked = false;
    record.bump = ctx.bumps.app_key_record;

    emit!(AppKeyAdded {
        fid: record.fid,
        app_pubkey,
        scope,
        expires_at,
    });

    Ok(())
}
