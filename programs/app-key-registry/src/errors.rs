use anchor_lang::prelude::*;

#[error_code]
pub enum AppKeyError {
    #[msg("Unauthorized: signer is not the FID custody address")]
    UnauthorizedCustody,
    #[msg("Invalid scope value (must be 0-3)")]
    InvalidScope,
    #[msg("App key is already revoked")]
    AlreadyRevoked,
    #[msg("App key has expired")]
    Expired,
}
