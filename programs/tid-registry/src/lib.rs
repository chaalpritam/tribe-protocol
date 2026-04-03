use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("4BSmJmRGQWKgioP9DG2bUuRS9U3V6soRauU7Nv6yGvHD");

#[program]
pub mod tid_registry {
    use super::*;

    /// Initialize the global state (once, by deployer).
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Register a new TID for the signer.
    pub fn register(ctx: Context<Register>, recovery_address: Pubkey) -> Result<()> {
        instructions::register::handler(ctx, recovery_address)
    }

    /// Transfer TID custody to a new wallet.
    pub fn transfer(ctx: Context<Transfer>, new_custody: Pubkey) -> Result<()> {
        instructions::transfer::handler(ctx, new_custody)
    }

    /// Recover TID using the recovery address.
    pub fn recover(ctx: Context<Recover>, new_custody: Pubkey) -> Result<()> {
        instructions::recover::handler(ctx, new_custody)
    }

    /// Change the recovery address.
    pub fn change_recovery(ctx: Context<ChangeRecovery>, new_recovery: Pubkey) -> Result<()> {
        instructions::change_recovery::handler(ctx, new_recovery)
    }
}
