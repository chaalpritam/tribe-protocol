use anchor_lang::prelude::*;

/// Core identity record — one per user.
#[account]
pub struct FidRecord {
    /// Unique numeric identity (auto-incremented).
    pub fid: u64,
    /// Primary wallet that controls this FID.
    pub custody_address: Pubkey,
    /// Can reclaim FID if custody key is lost.
    pub recovery_address: Pubkey,
    /// Unix timestamp of registration.
    pub registered_at: i64,
    pub bump: u8,
}

impl FidRecord {
    pub const SIZE: usize = 8 + 8 + 32 + 32 + 8 + 1;
}

/// Reverse lookup: custody_address → fid.
#[account]
pub struct CustodyLookup {
    pub fid: u64,
    pub bump: u8,
}

impl CustodyLookup {
    pub const SIZE: usize = 8 + 8 + 1;
}
