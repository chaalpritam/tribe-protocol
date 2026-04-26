use anchor_lang::prelude::*;

#[error_code]
pub enum TipRegistryError {
    #[msg("Tip amount must be greater than zero")]
    ZeroAmount,
    #[msg("Sender and recipient TIDs must differ")]
    SelfTip,
    #[msg("Sender state TID does not match the seed-bound sender")]
    SenderTidMismatch,
}
