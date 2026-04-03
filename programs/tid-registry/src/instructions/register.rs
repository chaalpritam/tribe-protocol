use anchor_lang::prelude::*;
use crate::state::{GlobalState, TidRecord, CustodyLookup};
use crate::events::TidRegistered;

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
        space = TidRecord::SIZE,
        seeds = [b"tid", (global_state.tid_counter + 1).to_le_bytes().as_ref()],
        bump,
    )]
    pub tid_record: Account<'info, TidRecord>,

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
    let tid = state.next_tid();

    let record = &mut ctx.accounts.tid_record;
    record.tid = tid;
    record.custody_address = ctx.accounts.custody.key();
    record.recovery_address = recovery_address;
    record.registered_at = Clock::get()?.unix_timestamp;
    record.bump = ctx.bumps.tid_record;

    let lookup = &mut ctx.accounts.custody_lookup;
    lookup.tid = tid;
    lookup.bump = ctx.bumps.custody_lookup;

    emit!(TidRegistered {
        tid,
        custody_address: ctx.accounts.custody.key(),
        recovery_address,
    });

    Ok(())
}
