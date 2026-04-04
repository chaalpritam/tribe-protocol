use anchor_lang::prelude::*;
use crate::state::HubRecord;
use crate::errors::HubRegistryError;
use crate::events::HubDeactivated;

#[derive(Accounts)]
pub struct DeactivateHub<'info> {
    #[account(
        mut,
        seeds = [b"hub", operator.key().as_ref()],
        bump = hub_record.bump,
        close = operator,
    )]
    pub hub_record: Account<'info, HubRecord>,

    #[account(mut)]
    pub operator: Signer<'info>,
}

pub fn handler(ctx: Context<DeactivateHub>) -> Result<()> {
    require!(
        ctx.accounts.hub_record.operator == ctx.accounts.operator.key(),
        HubRegistryError::UnauthorizedOperator
    );

    emit!(HubDeactivated {
        operator: ctx.accounts.hub_record.operator,
    });

    // Account is closed via `close = operator`, rent reclaimed
    Ok(())
}
