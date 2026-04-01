use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("5LtbFUeAoXWRovGpyWnRJhiCS62XsTYKVErT9kPpv4hN");

#[program]
pub mod app_key_registry {
    use super::*;

    /// Add a new app key for an FID.
    pub fn add_app_key(
        ctx: Context<AddAppKey>,
        app_pubkey: Pubkey,
        scope: u8,
        expires_at: i64,
    ) -> Result<()> {
        instructions::add_key::handler(ctx, app_pubkey, scope, expires_at)
    }

    /// Revoke an existing app key.
    pub fn revoke_app_key(ctx: Context<RevokeAppKey>) -> Result<()> {
        instructions::revoke_key::handler(ctx)
    }

    /// Rotate an app key (revoke old + add new atomically).
    pub fn rotate_app_key(
        ctx: Context<RotateAppKey>,
        new_app_pubkey: Pubkey,
        scope: u8,
        expires_at: i64,
    ) -> Result<()> {
        instructions::rotate_key::handler(ctx, new_app_pubkey, scope, expires_at)
    }
}
