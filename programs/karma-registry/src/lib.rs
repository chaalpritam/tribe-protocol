use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("KarmaReg11111111111111111111111111111111111");

#[program]
pub mod karma_registry {
    use super::*;

    /// One-time per TID. Anyone can pay the rent to bring a karma
    /// account into existence — karma is public and counters all
    /// start at zero, so there's no permission to gate.
    pub fn init_karma_account(ctx: Context<InitKarmaAccount>, tid: u64) -> Result<()> {
        instructions::init_karma_account::handler(ctx, tid)
    }

    /// Credit karma for a tip the recipient received. The handler
    /// verifies the TipRecord's recipient_tid matches the karma
    /// account, and the per-record KarmaProof PDA's `init`
    /// constraint guarantees each tip can only be counted once.
    pub fn record_tip_received(ctx: Context<RecordTipReceived>) -> Result<()> {
        instructions::record_tip_received::handler(ctx)
    }

    /// Credit karma for a completed task. Verifies status =
    /// Completed and claimer_tid matches the karma account; same
    /// per-record KarmaProof double-count protection.
    pub fn record_task_completed(ctx: Context<RecordTaskCompleted>) -> Result<()> {
        instructions::record_task_completed::handler(ctx)
    }
}
