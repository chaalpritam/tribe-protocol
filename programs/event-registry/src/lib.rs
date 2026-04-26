use anchor_lang::prelude::*;

pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("D2Gt2qkNAa8gZAmvqt3PWH39ydBL1cpwuXqeogkCoPRk");

#[program]
pub mod event_registry {
    use super::*;

    /// One-time per-creator setup. Tracks `next_event_id` so each
    /// event the creator publishes has a deterministic PDA.
    pub fn init_creator_state(ctx: Context<InitCreatorState>, creator_tid: u64) -> Result<()> {
        instructions::init_creator_state::handler(ctx, creator_tid)
    }

    /// Create an event. Title / description / location_text live in
    /// the off-chain EVENT_ADD envelope; this account stores its
    /// hash plus the timing window, optional lat/lon, and the
    /// running RSVP counters (yes / no / maybe).
    pub fn create_event(
        ctx: Context<CreateEvent>,
        starts_at: i64,
        ends_at: i64,
        has_end: bool,
        latitude: f64,
        longitude: f64,
        has_location: bool,
        metadata_hash: [u8; 32],
    ) -> Result<()> {
        instructions::create_event::handler(
            ctx,
            starts_at,
            ends_at,
            has_end,
            latitude,
            longitude,
            has_location,
            metadata_hash,
        )
    }

    /// RSVP yes / no / maybe to an event. The Rsvp PDA seeded by
    /// (event, attendee) has an `init` constraint, so each TID can
    /// only RSVP once per event. Use `update_rsvp` to change a
    /// previous response.
    pub fn rsvp(ctx: Context<Rsvp>, attendee_tid: u64, status: u8) -> Result<()> {
        instructions::rsvp::handler(ctx, attendee_tid, status)
    }

    /// Change an existing RSVP. Decrements the previous status's
    /// counter and increments the new one.
    pub fn update_rsvp(ctx: Context<UpdateRsvp>, status: u8) -> Result<()> {
        instructions::update_rsvp::handler(ctx, status)
    }
}
