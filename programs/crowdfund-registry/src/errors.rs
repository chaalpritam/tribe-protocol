use anchor_lang::prelude::*;

#[error_code]
pub enum CrowdfundRegistryError {
    #[msg("Goal amount must be greater than zero")]
    ZeroGoal,
    #[msg("Pledge amount must be greater than zero")]
    ZeroPledge,
    #[msg("Deadline must be in the future")]
    DeadlineInPast,
    #[msg("Campaign has already passed its deadline")]
    AfterDeadline,
    #[msg("Campaign deadline has not been reached yet")]
    BeforeDeadline,
    #[msg("Campaign is not active")]
    NotActive,
    #[msg("Campaign goal was not met; creator cannot claim funds")]
    GoalNotMet,
    #[msg("Campaign goal was met; backers cannot refund")]
    GoalMet,
    #[msg("Pledge has already been refunded")]
    AlreadyRefunded,
}
