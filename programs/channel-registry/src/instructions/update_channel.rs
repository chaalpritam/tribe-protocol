use anchor_lang::prelude::*;

use crate::events::ChannelUpdated;
use crate::state::ChannelRecord;

#[derive(Accounts)]
#[instruction(id: String)]
pub struct UpdateChannel<'info> {
    #[account(
        mut,
        seeds = [b"channel", id.as_bytes()],
        bump = channel.bump,
        has_one = owner,
    )]
    pub channel: Account<'info, ChannelRecord>,

    pub owner: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateChannel>,
    _id: String,
    latitude: f64,
    longitude: f64,
    has_location: bool,
    metadata_hash: [u8; 32],
) -> Result<()> {
    let channel = &mut ctx.accounts.channel;
    channel.latitude = latitude;
    channel.longitude = longitude;
    channel.has_location = has_location;
    channel.metadata_hash = metadata_hash;
    channel.updated_at = Clock::get()?.unix_timestamp;

    emit!(ChannelUpdated {
        channel: channel.key(),
        owner: channel.owner,
    });

    Ok(())
}
