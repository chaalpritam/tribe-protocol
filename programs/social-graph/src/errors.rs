use anchor_lang::prelude::*;

#[error_code]
pub enum SocialGraphError {
    #[msg("Unauthorized: signer is not the FID custody address")]
    UnauthorizedCustody,
    #[msg("Cannot follow yourself")]
    CannotFollowSelf,
    #[msg("Profile already initialized")]
    ProfileAlreadyExists,
}
