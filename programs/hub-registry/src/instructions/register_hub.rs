use anchor_lang::prelude::*;
use crate::state::HubRecord;
use crate::errors::HubRegistryError;
use crate::events::HubRegistered;

#[derive(Accounts)]
pub struct RegisterHub<'info> {
    #[account(
        init,
        payer = operator,
        space = HubRecord::SIZE,
        seeds = [b"hub", operator.key().as_ref()],
        bump,
    )]
    pub hub_record: Account<'info, HubRecord>,

    #[account(mut)]
    pub operator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterHub>, url: String, gossip_key: Pubkey) -> Result<()> {
    require!(url.len() >= 10, HubRegistryError::UrlTooShort);
    require!(url.len() <= 128, HubRegistryError::UrlTooLong);

    let hub = &mut ctx.accounts.hub_record;
    hub.operator = ctx.accounts.operator.key();

    let url_bytes = url.as_bytes();
    hub.url[..url_bytes.len()].copy_from_slice(url_bytes);
    hub.url_len = url_bytes.len() as u8;

    hub.gossip_key = gossip_key;
    hub.registered_at = Clock::get()?.unix_timestamp;
    hub.last_heartbeat = hub.registered_at;
    hub.active = true;
    hub.bump = ctx.bumps.hub_record;

    emit!(HubRegistered {
        operator: hub.operator,
        gossip_key,
    });

    Ok(())
}
