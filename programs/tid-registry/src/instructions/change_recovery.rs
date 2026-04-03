use anchor_lang::prelude::*;
use crate::state::TidRecord;
use crate::errors::TidError;
use crate::events::RecoveryChanged;

#[derive(Accounts)]
pub struct ChangeRecovery<'info> {
    #[account(
        mut,
        seeds = [b"tid", tid_record.tid.to_le_bytes().as_ref()],
        bump = tid_record.bump,
        constraint = tid_record.custody_address == custody.key() @ TidError::UnauthorizedCustody,
    )]
    pub tid_record: Account<'info, TidRecord>,

    pub custody: Signer<'info>,
}

pub fn handler(ctx: Context<ChangeRecovery>, new_recovery: Pubkey) -> Result<()> {
    let record = &mut ctx.accounts.tid_record;
    let old_recovery = record.recovery_address;

    require_keys_neq!(old_recovery, new_recovery, TidError::SameRecoveryAddress);

    record.recovery_address = new_recovery;

    emit!(RecoveryChanged {
        tid: record.tid,
        old_recovery,
        new_recovery,
    });

    Ok(())
}
