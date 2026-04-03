use anchor_lang::prelude::*;

/// Core identity record — one per user.
#[account]
pub struct TidRecord {
    /// Unique numeric identity (auto-incremented).
    pub tid: u64,
    /// Primary wallet that controls this TID.
    pub custody_address: Pubkey,
    /// Can reclaim TID if custody key is lost.
    pub recovery_address: Pubkey,
    /// Unix timestamp of registration.
    pub registered_at: i64,
    pub bump: u8,
}

impl TidRecord {
    pub const SIZE: usize = 8 + 8 + 32 + 32 + 8 + 1;
}

/// Reverse lookup: custody_address → tid.
#[account]
pub struct CustodyLookup {
    pub tid: u64,
    pub bump: u8,
}

impl CustodyLookup {
    pub const SIZE: usize = 8 + 8 + 1;
}
