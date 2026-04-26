use anchor_lang::prelude::*;

use crate::events::SenderTipStateInitialized;
use crate::state::SenderTipState;

#[derive(Accounts)]
pub struct InitSenderState<'info> {
    #[account(
        init,
        payer = sender,
        space = SenderTipState::SIZE,
        seeds = [b"tip-sender", sender.key().as_ref()],
        bump,
    )]
    pub sender_state: Account<'info, SenderTipState>,

    #[account(mut)]
    pub sender: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitSenderState>, sender_tid: u64) -> Result<()> {
    let state = &mut ctx.accounts.sender_state;
    state.sender = ctx.accounts.sender.key();
    state.sender_tid = sender_tid;
    state.next_tip_id = 0;
    state.bump = ctx.bumps.sender_state;

    emit!(SenderTipStateInitialized {
        sender: state.sender,
        sender_tid,
    });

    Ok(())
}
