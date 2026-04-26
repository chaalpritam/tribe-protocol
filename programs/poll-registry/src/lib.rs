use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("HPd8FqxVfoeBxwBr7wuKDeahgGX1V9UewxEWzjZY2SAm");

#[program]
pub mod poll_registry {
    use super::*;

    /// One-time per-creator setup. Tracks `next_poll_id` so each
    /// poll the creator publishes has a deterministic PDA.
    pub fn init_creator_state(ctx: Context<InitCreatorState>, creator_tid: u64) -> Result<()> {
        instructions::init_creator_state::handler(ctx, creator_tid)
    }

    /// Create a poll. The poll body (question / options) lives off
    /// chain in the corresponding POLL_ADD message; this instruction
    /// only stores its hash and the per-option tally counters
    /// (capped at 8 options).
    pub fn create_poll(
        ctx: Context<CreatePoll>,
        option_count: u8,
        expires_at: i64,
        has_expiry: bool,
        metadata_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_poll::handler(ctx, option_count, expires_at, has_expiry, metadata_hash)
    }

    /// Cast a vote. The Vote PDA seeded by (poll, voter) has an
    /// `init` constraint, so each TID can only vote once per poll.
    pub fn vote(ctx: Context<Vote>, voter_tid: u64, option_index: u8) -> Result<()> {
        instructions::vote::handler(ctx, voter_tid, option_index)
    }
}
