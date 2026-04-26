use anchor_lang::prelude::*;

use tip_registry::state::TipRecord;

use crate::errors::KarmaRegistryError;
use crate::events::TipKarmaRecorded;
use crate::state::{KarmaAccount, KarmaProof, KARMA_PROOF_KIND_TIP};

#[derive(Accounts)]
pub struct RecordTipReceived<'info> {
    #[account(
        mut,
        seeds = [b"karma".as_ref(), &karma.tid.to_le_bytes()],
        bump = karma.bump,
    )]
    pub karma: Account<'info, KarmaAccount>,

    /// The TipRecord to credit. Anchor deserializes the bytes
    /// against tip_registry's account schema; we don't write to it.
    pub tip_record: Account<'info, TipRecord>,

    /// `init` here is the double-count guard: a second call against
    /// the same TipRecord finds the proof already exists and aborts.
    #[account(
        init,
        payer = payer,
        space = KarmaProof::SIZE,
        seeds = [b"karma-proof", tip_record.key().as_ref()],
        bump,
    )]
    pub karma_proof: Account<'info, KarmaProof>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RecordTipReceived>) -> Result<()> {
    let tip = &ctx.accounts.tip_record;
    let karma = &mut ctx.accounts.karma;

    require!(
        tip.recipient_tid == karma.tid,
        KarmaRegistryError::TidMismatch
    );

    karma.tips_received_count = karma
        .tips_received_count
        .checked_add(1)
        .expect("tips_received_count overflow unreachable in practice");
    karma.tips_received_lamports = karma
        .tips_received_lamports
        .checked_add(tip.amount)
        .expect("tips_received_lamports overflow unreachable in practice");

    let proof = &mut ctx.accounts.karma_proof;
    proof.source = tip.key();
    proof.kind = KARMA_PROOF_KIND_TIP;
    proof.tid = karma.tid;
    proof.bump = ctx.bumps.karma_proof;

    emit!(TipKarmaRecorded {
        karma: karma.key(),
        tid: karma.tid,
        tip_record: tip.key(),
        amount: tip.amount,
        new_tip_count: karma.tips_received_count,
        new_tip_lamports: karma.tips_received_lamports,
    });

    Ok(())
}
