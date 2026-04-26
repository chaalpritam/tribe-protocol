use anchor_lang::prelude::*;

use crate::events::ChannelTransferred;
use crate::state::ChannelRecord;

#[derive(Accounts)]
#[instruction(id: String)]
pub struct TransferChannel<'info> {
    #[account(
        mut,
        seeds = [b"channel", id.as_bytes()],
        bump = channel.bump,
        has_one = owner,
    )]
    pub channel: Account<'info, ChannelRecord>,

    pub owner: Signer<'info>,

    /// CHECK: New owner is just a Pubkey we record — they don't need
    /// to sign in this V1 transfer. A future iteration could require
    /// a two-step accept handshake.
    pub new_owner: AccountInfo<'info>,
}

pub fn handler(
    ctx: Context<TransferChannel>,
    _id: String,
    new_owner_tid: u64,
) -> Result<()> {
    let channel = &mut ctx.accounts.channel;
    let previous_owner = channel.owner;
    let previous_owner_tid = channel.owner_tid;

    channel.owner = ctx.accounts.new_owner.key();
    channel.owner_tid = new_owner_tid;
    channel.updated_at = Clock::get()?.unix_timestamp;

    emit!(ChannelTransferred {
        channel: channel.key(),
        previous_owner,
        previous_owner_tid,
        new_owner: channel.owner,
        new_owner_tid,
    });

    Ok(())
}
