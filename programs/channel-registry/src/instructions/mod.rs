pub mod register_channel;
pub mod transfer_channel;
pub mod update_channel;

pub use register_channel::*;
pub use transfer_channel::*;
pub use update_channel::*;

use anchor_lang::prelude::*;

use crate::errors::ChannelRegistryError;
use crate::state::RESERVED_GENERAL_ID;

/// Validate a channel id against `/^[a-z0-9-]+$/` capped at 32 bytes
/// and reject the reserved `general` slug.
pub fn validate_channel_id(id: &str) -> Result<()> {
    require!(!id.is_empty(), ChannelRegistryError::IdEmpty);
    require!(id.len() <= 32, ChannelRegistryError::IdTooLong);
    for &b in id.as_bytes() {
        let valid = b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-';
        require!(valid, ChannelRegistryError::InvalidIdChars);
    }
    require!(id != RESERVED_GENERAL_ID, ChannelRegistryError::ReservedId);
    Ok(())
}
