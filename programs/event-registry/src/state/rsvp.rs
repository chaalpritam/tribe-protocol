use anchor_lang::prelude::*;

#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum RsvpStatus {
    Yes = 1,
    No = 2,
    Maybe = 3,
}

/// One per (event, attendee). Lives at PDA
/// `["rsvp", event_pubkey, attendee_pubkey]`. Created on first RSVP
/// (init constraint enforces uniqueness); status can be flipped via
/// `update_rsvp` without spawning a second account.
#[account]
pub struct Rsvp {
    pub event: Pubkey,
    pub attendee: Pubkey,
    pub attendee_tid: u64,
    pub status: u8,
    pub responded_at: i64,
    pub bump: u8,
}

impl Rsvp {
    // discriminator(8) + event(32) + attendee(32) + attendee_tid(8)
    // + status(1) + responded_at(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 1 + 8 + 1;
}
