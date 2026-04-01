use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("65oKjSjcGYR61ASzDYczbodz6H8TARtJyQGvb5V9y9W1");

#[program]
pub mod username_registry {
    use super::*;

    /// Register a username for an FID.
    pub fn register_username(ctx: Context<RegisterUsername>, username: String) -> Result<()> {
        instructions::register::handler(ctx, username)
    }

    /// Renew a username (extend expiry).
    pub fn renew_username(ctx: Context<RenewUsername>) -> Result<()> {
        instructions::renew::handler(ctx)
    }

    /// Transfer a username to a different FID.
    pub fn transfer_username(ctx: Context<TransferUsername>, new_fid: u64) -> Result<()> {
        instructions::transfer::handler(ctx, new_fid)
    }

    /// Release a username (close account, reclaim rent).
    pub fn release_username(ctx: Context<ReleaseUsername>) -> Result<()> {
        instructions::release::handler(ctx)
    }
}
