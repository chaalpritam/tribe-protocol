use anchor_lang::prelude::*;

#[event]
pub struct CreatorStateInitialized {
    pub creator: Pubkey,
    pub creator_tid: u64,
}

#[event]
pub struct EventCreated {
    pub event: Pubkey,
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub event_id: u64,
    pub starts_at: i64,
}

#[event]
pub struct EventRsvped {
    pub event: Pubkey,
    pub attendee: Pubkey,
    pub attendee_tid: u64,
    pub status: u8,
}

#[event]
pub struct EventRsvpUpdated {
    pub event: Pubkey,
    pub attendee: Pubkey,
    pub previous_status: u8,
    pub new_status: u8,
}
