use anchor_lang::prelude::*;

pub const MAX_USERNAME_LEN: usize = 20;
/// One year in seconds.
pub const REGISTRATION_DURATION: i64 = 365 * 24 * 60 * 60;

/// Username record — human-readable name bound to a TID.
#[account]
pub struct UsernameRecord {
    /// Fixed-size username (padded with zeros). Avoids realloc.
    pub username: [u8; MAX_USERNAME_LEN],
    /// Actual length of the username string.
    pub username_len: u8,
    /// The TID this username is bound to.
    pub tid: u64,
    /// Unix timestamp of registration.
    pub registered_at: i64,
    /// Expiry timestamp (annual renewal).
    pub expiry: i64,
    pub bump: u8,
}

impl UsernameRecord {
    pub const SIZE: usize = 8 + MAX_USERNAME_LEN + 1 + 8 + 8 + 8 + 1; // = 54

    pub fn username_str(&self) -> &str {
        std::str::from_utf8(&self.username[..self.username_len as usize]).unwrap_or("")
    }

    pub fn is_expired(&self, now: i64) -> bool {
        now > self.expiry
    }
}

/// Reverse lookup: TID → username hash.
#[account]
pub struct TidUsername {
    pub username_hash: [u8; 32],
    pub bump: u8,
}

impl TidUsername {
    pub const SIZE: usize = 8 + 32 + 1; // = 41
}
