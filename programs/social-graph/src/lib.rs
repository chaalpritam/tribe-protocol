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
}
