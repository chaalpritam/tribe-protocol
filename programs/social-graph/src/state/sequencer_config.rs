use anchor_lang::prelude::*;

/// Sequencer configuration — stores the authorized ER server pubkey.
/// Only the sequencer can call delegated follow/unfollow instructions.
#[account]
pub struct SequencerConfig {
    pub authority: Pubkey,
    pub admin: Pubkey,
    pub bump: u8,
}

impl SequencerConfig {
    pub const SIZE: usize = 8 + 32 + 32 + 1; // discriminator + authority + admin + bump
}
