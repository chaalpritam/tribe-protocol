use anchor_lang::prelude::*;

#[error_code]
pub enum KarmaRegistryError {
    #[msg("TID on the karma account does not match the source record")]
    TidMismatch,
    #[msg("Task has not been completed yet")]
    TaskNotCompleted,
}
