use anchor_lang::prelude::*;

#[error_code]
pub enum EventRegistryError {
    #[msg("Event must start in the future")]
    StartInPast,
    #[msg("Event end time must be after the start time")]
    EndBeforeStart,
    #[msg("RSVP status must be 1 (yes), 2 (no), or 3 (maybe)")]
    InvalidRsvpStatus,
    #[msg("RSVP status is unchanged")]
    NoOpUpdate,
}
