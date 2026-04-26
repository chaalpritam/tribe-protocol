use anchor_lang::prelude::*;

use crate::errors::EventRegistryError;
use crate::events::EventRsvpUpdated;
use crate::state::{Event, Rsvp as RsvpState, RsvpStatus};

use super::rsvp::{decrement_counter, increment_counter};

#[derive(Accounts)]
pub struct UpdateRsvp<'info> {
    #[account(
        mut,
        seeds = [
            b"event",
            event.creator.as_ref(),
            &event.event_id.to_le_bytes(),
        ],
        bump = event.bump,
    )]
    pub event: Account<'info, Event>,

    #[account(
        mut,
        seeds = [b"rsvp", event.key().as_ref(), attendee.key().as_ref()],
        bump = rsvp_record.bump,
        has_one = attendee,
    )]
    pub rsvp_record: Account<'info, RsvpState>,

    pub attendee: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateRsvp>, status: u8) -> Result<()> {
    require!(
        status == RsvpStatus::Yes as u8
            || status == RsvpStatus::No as u8
            || status == RsvpStatus::Maybe as u8,
        EventRegistryError::InvalidRsvpStatus
    );
    let previous = ctx.accounts.rsvp_record.status;
    require!(previous != status, EventRegistryError::NoOpUpdate);

    let event = &mut ctx.accounts.event;
    decrement_counter(event, previous)?;
    increment_counter(event, status)?;

    let record = &mut ctx.accounts.rsvp_record;
    record.status = status;
    record.responded_at = Clock::get()?.unix_timestamp;

    emit!(EventRsvpUpdated {
        event: event.key(),
        attendee: record.attendee,
        previous_status: previous,
        new_status: status,
    });

    Ok(())
}
