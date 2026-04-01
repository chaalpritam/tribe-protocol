use anchor_lang::prelude::*;

/// Scoped delegation key — apps sign on behalf of users.
#[account]
pub struct AppKeyRecord {
    /// The FID this key belongs to.
    pub fid: u64,
    /// The ephemeral pubkey the app uses to sign messages.
    pub app_pubkey: Pubkey,
    /// Permission scope: 0=Full, 1=CastsOnly, 2=SocialOnly, 3=ReadOnly.
    pub scope: u8,
    /// When this key was created.
    pub created_at: i64,
    /// Expiry timestamp (0 = no expiry).
    pub expires_at: i64,
    /// Whether this key has been revoked.
    pub revoked: bool,
    pub bump: u8,
}

impl AppKeyRecord {
    pub const SIZE: usize = 8 + 8 + 32 + 1 + 8 + 8 + 1 + 1; // = 67

    pub fn is_active(&self, now: i64) -> bool {
        !self.revoked && (self.expires_at == 0 || self.expires_at > now)
    }
}

/// Scope constants.
pub mod scope {
    pub const FULL: u8 = 0;
    pub const CASTS_ONLY: u8 = 1;
    pub const SOCIAL_ONLY: u8 = 2;
    pub const READ_ONLY: u8 = 3;
}
