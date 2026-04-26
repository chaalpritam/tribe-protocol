use anchor_lang::prelude::*;

use crate::errors::EventRegistryError;
use crate::events::EventCreated;
use crate::state::{CreatorEventState, Event};

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(
        mut,
        seeds = [b"event-creator", creator.key().as_ref()],
        bump = creator_state.bump,
        has_one = creator,
    )]
    pub creator_state: Account<'info, CreatorEventState>,

    #[account(
        init,
        payer = creator,
        space = Event::SIZE,
        seeds = [
            b"event",
            creator.key().as_ref(),
            &creator_state.next_event_id.to_le_bytes(),
        ],
        bump,
    )]
    pub event: Account<'info, Event>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[allow(clippy::too_many_arguments)]
pub fn handler(
    ctx: Context<CreateEvent>,
    starts_at: i64,
    ends_at: i64,
    has_end: bool,
    latitude: f64,
    longitude: f64,
    has_location: bool,
    metadata_hash: [u8; 32],
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    require!(starts_at > now, EventRegistryError::StartInPast);
    if has_end {
        require!(ends_at > starts_at, EventRegistryError::EndBeforeStart);
    }

    let event_id = ctx.accounts.creator_state.next_event_id;

    let event = &mut ctx.accounts.event;
    event.creator = ctx.accounts.creator.key();
    event.creator_tid = ctx.accounts.creator_state.creator_tid;
    event.event_id = event_id;
    event.starts_at = starts_at;
    event.ends_at = ends_at;
    event.has_end = has_end;
    event.latitude = latitude;
    event.longitude = longitude;
    event.has_location = has_location;
    event.yes_count = 0;
    event.no_count = 0;
    event.maybe_count = 0;
    event.created_at = now;
    event.metadata_hash = metadata_hash;
    event.bump = ctx.bumps.event;

    ctx.accounts.creator_state.next_event_id = event_id
        .checked_add(1)
        .expect("event_id u64 overflow is unreachable in practice");

    emit!(EventCreated {
        event: event.key(),
        creator: event.creator,
        creator_tid: event.creator_tid,
        event_id,
        starts_at,
    });

    Ok(())
}
