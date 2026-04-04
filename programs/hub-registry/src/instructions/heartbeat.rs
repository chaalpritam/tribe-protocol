use anchor_lang::prelude::*;
use crate::state::HubRecord;
use crate::errors::HubRegistryError;
use crate::events::HubHeartbeat;

/// Hub operators call this periodically to prove liveness.
/// Apps can filter hubs by last_heartbeat to find active ones.
#[derive(Accounts)]
pub struct Heartbeat<'info> {
    #[account(
        mut,
        seeds = [b"hub", operator.key().as_ref()],
        bump = hub_record.bump,
    )]
    pub hub_record: Account<'info, HubRecord>,

    pub operator: Signer<'info>,
}

pub fn handler(ctx: Context<Heartbeat>) -> Result<()> {
    let hub = &mut ctx.accounts.hub_record;
    require!(hub.operator == ctx.accounts.operator.key(), HubRegistryError::UnauthorizedOperator);

    hub.last_heartbeat = Clock::get()?.unix_timestamp;

    emit!(HubHeartbeat {
        operator: hub.operator,
    });

    Ok(())
}
