use anchor_lang::prelude::*;

/// Single source of truth for a channel id. Lives at PDA seeds
/// `["channel", id_bytes]`. On-chain ids are capped at 32 bytes to fit
/// inside Solana's per-seed limit; that's plenty for slugs like
/// `san-francisco`, `solana-devs`, or `interest-rust`. Hubs may still
/// route longer ids off-chain, but only ids in this registry have
/// global ownership guarantees.
#[account]
pub struct ChannelRecord {
    /// Fixed-size buffer holding the literal channel id bytes
    /// (left-aligned, zero-padded). Use `id_len` to slice.
    pub id: [u8; 32],
    pub id_len: u8,
    /// 1 = GENERAL (reserved, never written by this program),
    /// 2 = CITY, 3 = INTEREST.
    pub kind: u8,
    pub owner: Pubkey,
    pub owner_tid: u64,
    /// Hash of the off-chain CHANNEL_ADD envelope so apps can fetch
    /// title / description / image without a second round trip.
    pub metadata_hash: [u8; 32],
    pub latitude: f64,
    pub longitude: f64,
    /// True iff `latitude`/`longitude` are meaningful. Cities almost
    /// always set this; interest groups normally don't.
    pub has_location: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

impl ChannelRecord {
    // discriminator(8) + id(32) + id_len(1) + kind(1) + owner(32)
    // + owner_tid(8) + metadata_hash(32) + latitude(8) + longitude(8)
    // + has_location(1) + created_at(8) + updated_at(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 1 + 1 + 32 + 8 + 32 + 8 + 8 + 1 + 8 + 8 + 1;

    /// Returns the readable channel id slice.
    pub fn id_str(&self) -> &str {
        let len = self.id_len as usize;
        // The handler validates id is ASCII before writing, so this
        // unwrap is safe for any record this program produced.
        std::str::from_utf8(&self.id[..len]).unwrap_or("")
    }
}

pub const CHANNEL_KIND_GENERAL: u8 = 1;
pub const CHANNEL_KIND_CITY: u8 = 2;
pub const CHANNEL_KIND_INTEREST: u8 = 3;
pub const RESERVED_GENERAL_ID: &str = "general";
