use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("TipReg1111111111111111111111111111111111111");

#[program]
pub mod tip_registry {
    use super::*;

    /// One-time per-sender setup. Creates the SenderTipState PDA which
    /// tracks a monotonic `next_tip_id` so each tip has a deterministic
    /// PDA without needing a wall-clock seed.
    pub fn init_sender_state(ctx: Context<InitSenderState>, sender_tid: u64) -> Result<()> {
        instructions::init_sender_state::handler(ctx, sender_tid)
    }

    /// Send a tip from the signer to `recipient`, transferring `amount`
    /// lamports via the System Program and recording a TipRecord PDA
    /// keyed by (sender, tip_id). Optionally anchor the tip to a
    /// content hash (e.g. the blake3 hash of a tweet).
    pub fn send_tip(
        ctx: Context<SendTip>,
        recipient_tid: u64,
        amount: u64,
        target_hash: [u8; 32],
        has_target: bool,
    ) -> Result<()> {
        instructions::send_tip::handler(ctx, recipient_tid, amount, target_hash, has_target)
    }
}
