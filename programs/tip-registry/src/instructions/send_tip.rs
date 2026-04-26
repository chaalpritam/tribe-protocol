use anchor_lang::prelude::*;
use anchor_lang::system_program::{self, Transfer};

use crate::errors::TipRegistryError;
use crate::events::TipSent;
use crate::state::{SenderTipState, TipRecord};

#[derive(Accounts)]
pub struct SendTip<'info> {
    #[account(
        mut,
        seeds = [b"tip-sender", sender.key().as_ref()],
        bump = sender_state.bump,
        has_one = sender,
    )]
    pub sender_state: Account<'info, SenderTipState>,

    #[account(
        init,
        payer = sender,
        space = TipRecord::SIZE,
        seeds = [
            b"tip",
            sender.key().as_ref(),
            &sender_state.next_tip_id.to_le_bytes(),
        ],
        bump,
    )]
    pub tip_record: Account<'info, TipRecord>,

    #[account(mut)]
    pub sender: Signer<'info>,

    /// CHECK: Recipient is a plain SystemAccount that receives lamports
    /// via the System Program transfer below. The off-chain client is
    /// responsible for ensuring it matches `recipient_tid`'s custody
    /// address from tid-registry.
    #[account(mut)]
    pub recipient: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<SendTip>,
    recipient_tid: u64,
    amount: u64,
    target_hash: [u8; 32],
    has_target: bool,
) -> Result<()> {
    require!(amount > 0, TipRegistryError::ZeroAmount);
    require!(
        ctx.accounts.sender_state.sender_tid != recipient_tid,
        TipRegistryError::SelfTip
    );

    // Move the lamports first; if the transfer fails the whole tx
    // aborts and no TipRecord is created.
    let cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.sender.to_account_info(),
            to: ctx.accounts.recipient.to_account_info(),
        },
    );
    system_program::transfer(cpi_ctx, amount)?;

    let tip_id = ctx.accounts.sender_state.next_tip_id;
    let now = Clock::get()?.unix_timestamp;

    let record = &mut ctx.accounts.tip_record;
    record.sender = ctx.accounts.sender.key();
    record.recipient = ctx.accounts.recipient.key();
    record.sender_tid = ctx.accounts.sender_state.sender_tid;
    record.recipient_tid = recipient_tid;
    record.amount = amount;
    record.tip_id = tip_id;
    record.created_at = now;
    record.target_hash = if has_target {
        target_hash
    } else {
        [0u8; 32]
    };
    record.has_target = has_target;
    record.bump = ctx.bumps.tip_record;

    // Bump for the next tip from this sender.
    ctx.accounts.sender_state.next_tip_id = tip_id
        .checked_add(1)
        .expect("tip_id u64 overflow is unreachable in practice");

    emit!(TipSent {
        sender: record.sender,
        recipient: record.recipient,
        sender_tid: record.sender_tid,
        recipient_tid,
        amount,
        tip_id,
        has_target,
        target_hash: record.target_hash,
    });

    Ok(())
}
