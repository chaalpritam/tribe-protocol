use anchor_lang::prelude::*;
use crate::state::HubRecord;
use crate::errors::HubRegistryError;
use crate::events::HubUpdated;

#[derive(Accounts)]
pub struct UpdateHub<'info> {
    #[account(
        mut,
        seeds = [b"hub", operator.key().as_ref()],
        bump = hub_record.bump,
    )]
    pub hub_record: Account<'info, HubRecord>,

    #[account(mut)]
    pub operator: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateHub>, url: Option<String>, gossip_key: Option<Pubkey>) -> Result<()> {
    let hub = &mut ctx.accounts.hub_record;
    require!(hub.operator == ctx.accounts.operator.key(), HubRegistryError::UnauthorizedOperator);

    if let Some(new_url) = url {
        require!(new_url.len() >= 10, HubRegistryError::UrlTooShort);
        require!(new_url.len() <= 128, HubRegistryError::UrlTooLong);

        // Clear old URL bytes
        hub.url = [0u8; 128];
        let url_bytes = new_url.as_bytes();
        hub.url[..url_bytes.len()].copy_from_slice(url_bytes);
        hub.url_len = url_bytes.len() as u8;
    }

    if let Some(new_key) = gossip_key {
        hub.gossip_key = new_key;
    }

    emit!(HubUpdated {
        operator: hub.operator,
    });

    Ok(())
}
