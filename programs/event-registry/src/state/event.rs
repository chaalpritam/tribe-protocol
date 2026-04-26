use anchor_lang::prelude::*;

/// One per event. Lives at PDA
/// `["event", creator_pubkey, event_id_le]`.
#[account]
pub struct Event {
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub event_id: u64,
    pub starts_at: i64,
    pub ends_at: i64,
    pub has_end: bool,
    pub latitude: f64,
    pub longitude: f64,
    pub has_location: bool,
    pub yes_count: u32,
    pub no_count: u32,
    pub maybe_count: u32,
    pub created_at: i64,
    pub metadata_hash: [u8; 32],
    pub bump: u8,
}

impl Event {
    // discriminator(8) + creator(32) + creator_tid(8) + event_id(8)
    // + starts_at(8) + ends_at(8) + has_end(1) + latitude(8) + longitude(8)
    // + has_location(1) + yes_count(4) + no_count(4) + maybe_count(4)
    // + created_at(8) + metadata_hash(32) + bump(1)
    pub const SIZE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 1 + 8 + 8 + 1 + 4 + 4 + 4 + 8 + 32 + 1;
}
