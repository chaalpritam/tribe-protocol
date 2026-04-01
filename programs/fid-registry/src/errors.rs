use anchor_lang::prelude::*;

#[error_code]
pub enum FidError {
    #[msg("Unauthorized: signer is not the custody address")]
    UnauthorizedCustody,
    #[msg("Unauthorized: signer is not the recovery address")]
    UnauthorizedRecovery,
    #[msg("Cannot transfer to the same custody address")]
    SameCustodyAddress,
    #[msg("Cannot set recovery to the same address")]
    SameRecoveryAddress,
}
