use anchor_lang::prelude::*;

use crate::errors::ChannelRegistryError;
use crate::events::ChannelRegistered;
use crate::state::{ChannelRecord, CHANNEL_KIND_CITY, CHANNEL_KIND_INTEREST};

use super::validate_channel_id;

#[derive(Accounts)]
#[instruction(id: String)]
pub struct RegisterChannel<'info> {
    #[account(
        init,
        payer = owner,
        space = ChannelRecord::SIZE,
        seeds = [b"channel", id.as_bytes()],
        bump,
    )]
    pub channel: Account<'info, ChannelRecord>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[allow(clippy::too_many_arguments)]
pub fn handler(
    ctx: Context<RegisterChannel>,
    id: String,
    kind: u8,
    owner_tid: u64,
    latitude: f64,
    longitude: f64,
    has_location: bool,
    metadata_hash: [u8; 32],
) -> Result<()> {
    validate_channel_id(&id)?;
    require!(
        kind == CHANNEL_KIND_CITY || kind == CHANNEL_KIND_INTEREST,
        ChannelRegistryError::InvalidKind
    );

    let now = Clock::get()?.unix_timestamp;

    let channel = &mut ctx.accounts.channel;
    let id_bytes = id.as_bytes();
    channel.id = [0u8; 32];
    channel.id[..id_bytes.len()].copy_from_slice(id_bytes);
    channel.id_len = id_bytes.len() as u8;
    channel.kind = kind;
    channel.owner = ctx.accounts.owner.key();
    channel.owner_tid = owner_tid;
    channel.metadata_hash = metadata_hash;
    channel.latitude = latitude;
    channel.longitude = longitude;
    channel.has_location = has_location;
    channel.created_at = now;
    channel.updated_at = now;
    channel.bump = ctx.bumps.channel;

    emit!(ChannelRegistered {
        channel: channel.key(),
        owner: channel.owner,
        owner_tid,
        kind,
    });

    Ok(())
}
