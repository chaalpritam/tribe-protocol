use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("HubReg1111111111111111111111111111111111111");

#[program]
pub mod hub_registry {
    use super::*;

    /// Register a new hub on the network.
    pub fn register_hub(ctx: Context<RegisterHub>, url: String, gossip_key: Pubkey) -> Result<()> {
        instructions::register_hub::handler(ctx, url, gossip_key)
    }

    /// Update hub URL and/or gossip key.
    pub fn update_hub(ctx: Context<UpdateHub>, url: Option<String>, gossip_key: Option<Pubkey>) -> Result<()> {
        instructions::update_hub::handler(ctx, url, gossip_key)
    }

    /// Send a heartbeat to prove hub liveness.
    pub fn heartbeat(ctx: Context<Heartbeat>) -> Result<()> {
        instructions::heartbeat::handler(ctx)
    }

    /// Deactivate and close hub account, reclaim rent.
    pub fn deactivate_hub(ctx: Context<DeactivateHub>) -> Result<()> {
        instructions::deactivate_hub::handler(ctx)
    }
}
