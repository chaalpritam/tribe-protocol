use anchor_lang::prelude::*;
use crate::state::{TidRecord, CustodyLookup};
use crate::errors::TidError;
use crate::events::TidTransferred;

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(
        mut,
        seeds = [b"tid", tid_record.tid.to_le_bytes().as_ref()],
        bump = tid_record.bump,
        constraint = tid_record.custody_address == custody.key() @ TidError::UnauthorizedCustody,
    )]
    pub tid_record: Account<'info, TidRecord>,

    /// Old custody lookup — will be closed.
    #[account(
        mut,
        seeds = [b"custody", custody.key().as_ref()],
        bump = old_custody_lookup.bump,
        close = custody,
    )]
    pub old_custody_lookup: Account<'info, CustodyLookup>,

    /// New custody lookup — will be created.
    #[account(
        init,
        payer = custody,
        space = CustodyLookup::SIZE,
        seeds = [b"custody", new_custody.key().as_ref()],
        bump,
    )]
    pub new_custody_lookup: Account<'info, CustodyLookup>,

    /// CHECK: The new custody address (not required to sign).
    pub new_custody: UncheckedAccount<'info>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Transfer>, new_custody: Pubkey) -> Result<()> {
    let record = &mut ctx.accounts.tid_record;
    let old_custody = record.custody_address;

    require_keys_neq!(old_custody, new_custody, TidError::SameCustodyAddress);

    record.custody_address = new_custody;

    let lookup = &mut ctx.accounts.new_custody_lookup;
    lookup.tid = record.tid;
    lookup.bump = ctx.bumps.new_custody_lookup;

    emit!(TidTransferred {
        tid: record.tid,
        old_custody,
        new_custody,
    });

    Ok(())
}
