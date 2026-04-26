use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("CrowdF11111111111111111111111111111111111111");

#[program]
pub mod crowdfund_registry {
    use super::*;

    /// One-time per-creator setup. Creates the CreatorCrowdfundState
    /// PDA tracking a monotonic `next_crowdfund_id` so each campaign
    /// gets a deterministic PDA.
    pub fn init_creator_state(
        ctx: Context<InitCreatorState>,
        creator_tid: u64,
    ) -> Result<()> {
        instructions::init_creator_state::handler(ctx, creator_tid)
    }

    /// Create a new crowdfund campaign. Initializes the Crowdfund PDA
    /// and an empty vault PDA that will hold escrowed lamports until
    /// the deadline.
    pub fn create_crowdfund(
        ctx: Context<CreateCrowdfund>,
        goal_amount: u64,
        deadline_at: i64,
        metadata_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_crowdfund::handler(ctx, goal_amount, deadline_at, metadata_hash)
    }

    /// Pledge `amount` lamports to a campaign. Transfers from the
    /// backer's wallet into the campaign vault and creates (or
    /// accumulates onto) the backer's Pledge PDA. Rejected after the
    /// deadline or once the campaign has been claimed/refunded.
    pub fn pledge(ctx: Context<Pledge>, backer_tid: u64, amount: u64) -> Result<()> {
        instructions::pledge::handler(ctx, backer_tid, amount)
    }

    /// Creator-only. Once the deadline has passed and the goal is
    /// met, sweep the vault into the creator's wallet and mark the
    /// campaign succeeded.
    pub fn claim_funds(ctx: Context<ClaimFunds>) -> Result<()> {
        instructions::claim_funds::handler(ctx)
    }

    /// Backer-only. Once the deadline has passed and the goal was
    /// NOT met, return the backer's pledge from the vault. Closes the
    /// Pledge PDA and reclaims its rent to the backer.
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        instructions::refund::handler(ctx)
    }
}
