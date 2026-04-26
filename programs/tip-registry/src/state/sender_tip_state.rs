use anchor_lang::prelude::*;

/// Per-sender tip counter. Lives at PDA `["tip-sender", sender_pubkey]`.
/// Each tip the sender sends consumes the current `next_tip_id` and
/// increments it, so tip PDAs are deterministic and globally unique
/// per sender without needing a wall-clock seed.
#[account]
pub struct SenderTipState {
    pub sender: Pubkey,
    pub sender_tid: u64,
    pub next_tip_id: u64,
    pub bump: u8,
}

impl SenderTipState {
    // discriminator(8) + sender(32) + sender_tid(8) + next_tip_id(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 1;
}
