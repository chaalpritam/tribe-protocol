use anchor_lang::prelude::*;

#[error_code]
pub enum HubRegistryError {
    #[msg("URL is too long (max 128 bytes)")]
    UrlTooLong,
    #[msg("URL is too short (min 10 bytes)")]
    UrlTooShort,
    #[msg("Unauthorized: signer is not the hub operator")]
    UnauthorizedOperator,
    #[msg("Hub is already deactivated")]
    AlreadyDeactivated,
}
