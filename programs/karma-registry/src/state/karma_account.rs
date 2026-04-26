use anchor_lang::prelude::*;

/// Public, append-only karma counters for a TID. Lives at PDA
/// `["karma", tid_le_bytes]`. Anyone can fund the account into
/// existence; only the per-source `record_*` instructions can
/// increment the counters, and each source record (a tip, a task)
/// can only be counted once thanks to the KarmaProof PDA.
#[account]
pub struct KarmaAccount {
    pub tid: u64,
    pub tips_received_count: u64,
    pub tips_received_lamports: u64,
    pub tasks_completed_count: u64,
    pub tasks_completed_reward_lamports: u64,
    pub bump: u8,
}

impl KarmaAccount {
    // discriminator(8) + tid(8) + tips_received_count(8)
    // + tips_received_lamports(8) + tasks_completed_count(8)
    // + tasks_completed_reward_lamports(8) + bump(1)
    pub const SIZE: usize = 8 + 8 + 8 + 8 + 8 + 8 + 1;
}
