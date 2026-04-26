use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("ChanReg111111111111111111111111111111111111");

#[program]
pub mod channel_registry {
    use super::*;

    /// First-write-wins registration of a channel id. The PDA is
    /// seeded by the literal id bytes, so each id maps to exactly one
    /// ChannelRecord across the entire network — no two hubs can
    /// disagree on who owns "san-francisco". On-chain ids are capped
    /// at 32 bytes; longer slugs route only off-chain in the hub.
    pub fn register_channel(
        ctx: Context<RegisterChannel>,
        id: String,
        kind: u8,
        owner_tid: u64,
        latitude: f64,
        longitude: f64,
        has_location: bool,
        metadata_hash: [u8; 32],
    ) -> Result<()> {
        instructions::register_channel::handler(
            ctx,
            id,
            kind,
            owner_tid,
            latitude,
            longitude,
            has_location,
            metadata_hash,
        )
    }

    /// Owner-only. Update location and / or off-chain metadata hash.
    /// The kind cannot change once registered. The `id` argument is
    /// used to derive the channel's PDA.
    pub fn update_channel(
        ctx: Context<UpdateChannel>,
        id: String,
        latitude: f64,
        longitude: f64,
        has_location: bool,
        metadata_hash: [u8; 32],
    ) -> Result<()> {
        instructions::update_channel::handler(
            ctx,
            id,
            latitude,
            longitude,
            has_location,
            metadata_hash,
        )
    }

    /// Owner-only. Hand the channel to a new TID + wallet. The
    /// outgoing owner signs; the incoming owner does not need to
    /// accept (V1 simple transfer).
    pub fn transfer_channel(
        ctx: Context<TransferChannel>,
        id: String,
        new_owner_tid: u64,
    ) -> Result<()> {
        instructions::transfer_channel::handler(ctx, id, new_owner_tid)
    }
}
