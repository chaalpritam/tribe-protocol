use anchor_lang::prelude::*;

#[error_code]
pub enum SocialGraphError {
    #[msg("Unauthorized: signer is not the TID custody address")]
    UnauthorizedCustody,
    #[msg("Cannot follow yourself")]
    CannotFollowSelf,
    #[msg("Profile already initialized")]
    ProfileAlreadyExists,
    #[msg("Unauthorized: signer is not the registered sequencer")]
    UnauthorizedSequencer,
    #[msg("Unauthorized: signer is not the sequencer config admin")]
    UnauthorizedAdmin,
}
