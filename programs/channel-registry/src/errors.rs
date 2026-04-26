use anchor_lang::prelude::*;

#[error_code]
pub enum ChannelRegistryError {
    #[msg("Channel id must not be empty")]
    IdEmpty,
    #[msg("Channel id is longer than 32 bytes")]
    IdTooLong,
    #[msg("Channel id must match /^[a-z0-9-]+$/")]
    InvalidIdChars,
    #[msg("Channel id \"general\" is reserved for the hub-seeded default channel")]
    ReservedId,
    #[msg("Kind must be CITY (2) or INTEREST (3); GENERAL (1) is reserved")]
    InvalidKind,
    #[msg("Signer is not the channel owner")]
    NotOwner,
}
