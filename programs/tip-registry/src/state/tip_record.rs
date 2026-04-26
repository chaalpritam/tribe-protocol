use anchor_lang::prelude::*;

/// On-chain receipt for a tip. Lives at PDA
/// `["tip", sender_pubkey, tip_id_le_bytes]`. Sender, recipient, and
/// amount are immutable once written; this is a settled history row.
#[account]
pub struct TipRecord {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub sender_tid: u64,
    pub recipient_tid: u64,
    pub amount: u64,
    pub tip_id: u64,
    pub created_at: i64,
    /// Optional anchor to a piece of content the tip is for (e.g. the
    /// blake3 hash of a tweet). Zeroed when `has_target` is false.
    pub target_hash: [u8; 32],
    pub has_target: bool,
    pub bump: u8,
}

impl TipRecord {
    // discriminator(8) + sender(32) + recipient(32) + sender_tid(8)
    // + recipient_tid(8) + amount(8) + tip_id(8) + created_at(8)
    // + target_hash(32) + has_target(1) + bump(1)
    pub const SIZE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 8 + 32 + 1 + 1;
}
