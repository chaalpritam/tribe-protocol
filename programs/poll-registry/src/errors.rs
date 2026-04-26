use anchor_lang::prelude::*;

#[error_code]
pub enum PollRegistryError {
    #[msg("Polls must have between 2 and 8 options")]
    BadOptionCount,
    #[msg("Selected option is out of range")]
    OptionOutOfRange,
    #[msg("Poll expiration must be in the future")]
    ExpiryInPast,
    #[msg("Poll has expired")]
    PollExpired,
    #[msg("Voter cannot be the poll creator")]
    SelfVote,
}
