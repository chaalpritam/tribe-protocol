use anchor_lang::prelude::*;
use crate::state::FidRecord;
use crate::errors::FidError;
use crate::events::RecoveryChanged;

#[derive(Accounts)]
pub struct ChangeRecovery<'info> {
    #[account(
        mut,
        seeds = [b"fid", fid_record.fid.to_le_bytes().as_ref()],
        bump = fid_record.bump,
        constraint = fid_record.custody_address == custody.key() @ FidError::UnauthorizedCustody,
    )]
    pub fid_record: Account<'info, FidRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<ChangeRecovery>, new_recovery: Pubkey) -> Result<()> {
    let record = &mut ctx.accounts.fid_record;
    let old_recovery = record.recovery_address;

    require_keys_neq!(old_recovery, new_recovery, FidError::SameRecoveryAddress);

    record.recovery_address = new_recovery;

    emit!(RecoveryChanged {
        fid: record.fid,
        old_recovery,
        new_recovery,
    });

    Ok(())
}
