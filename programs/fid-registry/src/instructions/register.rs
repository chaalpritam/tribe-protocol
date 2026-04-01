use anchor_lang::prelude::*;
use crate::state::{GlobalState, FidRecord, CustodyLookup};
use crate::events::FidRegistered;

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(
        mut,
        seeds = [b"global_state"],
        bump = global_state.bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(
        init,
        payer = custody,
        space = FidRecord::SIZE,
        seeds = [b"fid", (global_state.fid_counter + 1).to_le_bytes().as_ref()],
        bump,
    )]
    pub fid_record: Account<'info, FidRecord>,

    #[account(
        init,
        payer = custody,
        space = CustodyLookup::SIZE,
        seeds = [b"custody", custody.key().as_ref()],
        bump,
    )]
    pub custody_lookup: Account<'info, CustodyLookup>,

    #[account(mut)]
    pub custody: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Register>, recovery_address: Pubkey) -> Result<()> {
    let state = &mut ctx.accounts.global_state;
    let fid = state.next_fid();

    let record = &mut ctx.accounts.fid_record;
    record.fid = fid;
    record.custody_address = ctx.accounts.custody.key();
    record.recovery_address = recovery_address;
    record.registered_at = Clock::get()?.unix_timestamp;
    record.bump = ctx.bumps.fid_record;

    let lookup = &mut ctx.accounts.custody_lookup;
    lookup.fid = fid;
    lookup.bump = ctx.bumps.custody_lookup;

    emit!(FidRegistered {
        fid,
        custody_address: ctx.accounts.custody.key(),
        recovery_address,
    });

    Ok(())
}
