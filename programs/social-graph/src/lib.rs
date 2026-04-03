use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("8kKnWvbmTjWq5uPePk79RRbQMAXCszNFzHdRwUS4N74w");

#[program]
pub mod social_graph {
    use super::*;

    /// Initialize a social profile for a TID (once per user).
    pub fn init_profile(ctx: Context<InitProfile>) -> Result<()> {
        instructions::init_profile::handler(ctx)
    }

    /// Follow another user. Creates a Link PDA + increments counters.
    pub fn follow(ctx: Context<Follow>) -> Result<()> {
        instructions::follow::handler(ctx)
    }

    /// Unfollow a user. Closes Link PDA + decrements counters + reclaims rent.
    pub fn unfollow(ctx: Context<Unfollow>) -> Result<()> {
        instructions::unfollow::handler(ctx)
    }

    /// Initialize the sequencer config (one-time setup by admin).
    pub fn init_sequencer(ctx: Context<InitSequencer>, authority: Pubkey) -> Result<()> {
        instructions::init_sequencer::handler(ctx, authority)
    }

    /// Initialize a social profile via sequencer authority — used by ER server.
    pub fn init_profile_delegated(ctx: Context<InitProfileDelegated>, tid: u64) -> Result<()> {
        instructions::init_profile_delegated::handler(ctx, tid)
    }

    /// Follow via sequencer authority — used by ER server for batched settlement.
    pub fn follow_delegated(ctx: Context<FollowDelegated>, follower_tid: u64, following_tid: u64) -> Result<()> {
        instructions::follow_delegated::handler(ctx, follower_tid, following_tid)
    }

    /// Unfollow via sequencer authority — used by ER server for batched settlement.
    pub fn unfollow_delegated(ctx: Context<UnfollowDelegated>, follower_tid: u64, following_tid: u64) -> Result<()> {
        instructions::unfollow_delegated::handler(ctx, follower_tid, following_tid)
    }
}
