use anchor_lang::prelude::*;
use crate::state::GlobalState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GlobalState::SIZE,
        seeds = [b"global_state"],
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.global_state;
    state.tid_counter = 0;
    state.authority = ctx.accounts.authority.key();
    state.bump = ctx.bumps.global_state;
    Ok(())
}
