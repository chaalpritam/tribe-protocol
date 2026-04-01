use anchor_lang::prelude::*;
use crate::state::AppKeyRecord;
use crate::errors::AppKeyError;
use crate::events::AppKeyAdded;

/// FID record from the fid-registry program (read-only cross-program reference).
#[account]
pub struct FidRecord {
    pub fid: u64,
    pub custody_address: Pubkey,
    pub recovery_address: Pubkey,
    pub registered_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(app_pubkey: Pubkey)]
pub struct AddAppKey<'info> {
    /// The FID record — verifies custody ownership.
    /// CHECK: Deserialized manually to avoid cross-program dependency.
    #[account(
        constraint = fid_record.custody_address == custody.key() @ AppKeyError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    #[account(
        init,
        payer = custody,
        space = AppKeyRecord::SIZE,
        seeds = [b"app_key", fid_record.fid.to_le_bytes().as_ref(), app_pubkey.as_ref()],
        bump,
    )]
    pub app_key_record: Account<'info, AppKeyRecord>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddAppKey>, app_pubkey: Pubkey, scope: u8, expires_at: i64) -> Result<()> {
    require!(scope <= 3, AppKeyError::InvalidScope);

    let record = &mut ctx.accounts.app_key_record;
    record.fid = ctx.accounts.fid_record.fid;
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
