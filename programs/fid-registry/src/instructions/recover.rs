use anchor_lang::prelude::*;
use crate::state::{FidRecord, CustodyLookup};
use crate::errors::FidError;
use crate::events::FidRecovered;

#[derive(Accounts)]
pub struct Recover<'info> {
    #[account(
        mut,
        seeds = [b"fid", fid_record.fid.to_le_bytes().as_ref()],
        bump = fid_record.bump,
        constraint = fid_record.recovery_address == recovery.key() @ FidError::UnauthorizedRecovery,
    )]
    pub fid_record: Account<'info, FidRecord>,

    /// Old custody lookup — will be closed.
    #[account(
        mut,
        seeds = [b"custody", fid_record.custody_address.as_ref()],
        bump = old_custody_lookup.bump,
        close = recovery,
    )]
    pub old_custody_lookup: Account<'info, CustodyLookup>,

    /// New custody lookup.
    #[account(
        init,
        payer = recovery,
        space = CustodyLookup::SIZE,
        seeds = [b"custody", new_custody.key().as_ref()],
        bump,
    )]
    pub new_custody_lookup: Account<'info, CustodyLookup>,

    /// CHECK: The new custody address.
    pub new_custody: UncheckedAccount<'info>,

    #[account(mut)]
    pub recovery: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Recover>, new_custody: Pubkey) -> Result<()> {
    let record = &mut ctx.accounts.fid_record;
    let old_custody = record.custody_address;

    record.custody_address = new_custody;

    let lookup = &mut ctx.accounts.new_custody_lookup;
    lookup.fid = record.fid;
    lookup.bump = ctx.bumps.new_custody_lookup;

    emit!(FidRecovered {
        fid: record.fid,
        old_custody,
        new_custody,
    });

    Ok(())
}
