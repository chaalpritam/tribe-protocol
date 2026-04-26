use anchor_lang::prelude::*;

use crate::errors::EventRegistryError;
use crate::events::EventRsvped;
use crate::state::{Event, Rsvp as RsvpState, RsvpStatus};

#[derive(Accounts)]
pub struct Rsvp<'info> {
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

    /// `init` is the one-RSVP-per-TID guard: a second RSVP from the
    /// same wallet on the same event fails because the PDA already
    /// exists. To change a response, use `update_rsvp`.
    #[account(
        init,
        payer = attendee,
        space = RsvpState::SIZE,
        seeds = [b"rsvp", event.key().as_ref(), attendee.key().as_ref()],
        bump,
    )]
    pub rsvp_record: Account<'info, RsvpState>,

    #[account(mut)]
    pub attendee: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Rsvp>, attendee_tid: u64, status: u8) -> Result<()> {
    require!(
        status == RsvpStatus::Yes as u8
            || status == RsvpStatus::No as u8
            || status == RsvpStatus::Maybe as u8,
        EventRegistryError::InvalidRsvpStatus
    );

    let now = Clock::get()?.unix_timestamp;

    let record = &mut ctx.accounts.rsvp_record;
    record.event = ctx.accounts.event.key();
    record.attendee = ctx.accounts.attendee.key();
    record.attendee_tid = attendee_tid;
    record.status = status;
    record.responded_at = now;
    record.bump = ctx.bumps.rsvp_record;

    let event = &mut ctx.accounts.event;
    increment_counter(event, status)?;

    emit!(EventRsvped {
        event: event.key(),
        attendee: record.attendee,
        attendee_tid,
        status,
    });

    Ok(())
}

pub(crate) fn increment_counter(event: &mut Account<Event>, status: u8) -> Result<()> {
    if status == RsvpStatus::Yes as u8 {
        event.yes_count = event.yes_count.checked_add(1).unwrap();
    } else if status == RsvpStatus::No as u8 {
        event.no_count = event.no_count.checked_add(1).unwrap();
    } else if status == RsvpStatus::Maybe as u8 {
        event.maybe_count = event.maybe_count.checked_add(1).unwrap();
    } else {
        return err!(EventRegistryError::InvalidRsvpStatus);
    }
    Ok(())
}

pub(crate) fn decrement_counter(event: &mut Account<Event>, status: u8) -> Result<()> {
    if status == RsvpStatus::Yes as u8 {
        event.yes_count = event.yes_count.saturating_sub(1);
    } else if status == RsvpStatus::No as u8 {
        event.no_count = event.no_count.saturating_sub(1);
    } else if status == RsvpStatus::Maybe as u8 {
        event.maybe_count = event.maybe_count.saturating_sub(1);
    } else {
        return err!(EventRegistryError::InvalidRsvpStatus);
    }
    Ok(())
}
