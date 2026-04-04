use anchor_lang::prelude::*;

/// A registered hub on the Tribe network.
/// PDA seeded by the operator's pubkey.
#[account]
pub struct HubRecord {
    /// The hub operator's wallet.
    pub operator: Pubkey,
    /// Hub URL (e.g., "https://hub1.tribe.protocol")
    pub url: [u8; 128],
    /// Length of the URL string.
    pub url_len: u8,
    /// Hub's gossip pubkey (for peer authentication).
    pub gossip_key: Pubkey,
    /// Timestamp when the hub was registered.
    pub registered_at: i64,
    /// Last time the operator refreshed (proves liveness).
    pub last_heartbeat: i64,
    /// Whether the hub is active.
    pub active: bool,
    pub bump: u8,
}

impl HubRecord {
    // discriminator(8) + operator(32) + url(128) + url_len(1) + gossip_key(32)
    // + registered_at(8) + last_heartbeat(8) + active(1) + bump(1)
    pub const SIZE: usize = 8 + 32 + 128 + 1 + 32 + 8 + 8 + 1 + 1;
}
