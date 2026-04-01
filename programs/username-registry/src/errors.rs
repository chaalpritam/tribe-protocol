use anchor_lang::prelude::*;

#[error_code]
pub enum UsernameError {
    #[msg("Unauthorized: signer is not the FID custody address")]
    UnauthorizedCustody,
    #[msg("Username too long (max 20 characters)")]
    UsernameTooLong,
    #[msg("Username too short (min 1 character)")]
    UsernameTooShort,
    #[msg("Username contains invalid characters (alphanumeric + underscore only)")]
    InvalidCharacters,
    #[msg("Username has not expired yet")]
    NotExpired,
    #[msg("Username has expired")]
    Expired,
}
