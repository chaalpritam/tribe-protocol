use anchor_lang::prelude::*;

#[error_code]
pub enum TaskRegistryError {
    #[msg("Task is not open")]
    NotOpen,
    #[msg("Task has not been claimed")]
    NotClaimed,
    #[msg("Task creator cannot claim their own task")]
    SelfClaim,
    #[msg("Signer is not the task's claimer")]
    NotClaimer,
}
