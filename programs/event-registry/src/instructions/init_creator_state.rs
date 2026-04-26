use anchor_lang::prelude::*;

use crate::events::CreatorStateInitialized;
use crate::state::CreatorEventState;

#[derive(Accounts)]
pub struct InitCreatorState<'info> {
    #[account(
        init,
        payer = creator,
        space = CreatorEventState::SIZE,
        seeds = [b"event-creator", creator.key().as_ref()],
        bump,
    )]
    pub creator_state: Account<'info, CreatorEventState>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitCreatorState>, creator_tid: u64) -> Result<()> {
    let state = &mut ctx.accounts.creator_state;
    state.creator = ctx.accounts.creator.key();
    state.creator_tid = creator_tid;
    state.next_event_id = 0;
    state.bump = ctx.bumps.creator_state;

    emit!(CreatorStateInitialized {
        creator: state.creator,
        creator_tid,
    });

    Ok(())
}
