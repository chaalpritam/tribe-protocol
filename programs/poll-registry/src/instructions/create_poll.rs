use anchor_lang::prelude::*;

use crate::errors::PollRegistryError;
use crate::events::PollCreated;
use crate::state::{CreatorPollState, Poll, MAX_POLL_OPTIONS};

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(
        mut,
        seeds = [b"poll-creator", creator.key().as_ref()],
        bump = creator_state.bump,
        has_one = creator,
    )]
    pub creator_state: Account<'info, CreatorPollState>,

    #[account(
        init,
        payer = creator,
        space = Poll::SIZE,
        seeds = [
            b"poll",
            creator.key().as_ref(),
            &creator_state.next_poll_id.to_le_bytes(),
        ],
        bump,
    )]
    pub poll: Account<'info, Poll>,

    #[account(mut)]
    pub creator: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreatePoll>,
    option_count: u8,
    expires_at: i64,
    has_expiry: bool,
    metadata_hash: [u8; 32],
) -> Result<()> {
    require!(
        option_count >= 2 && (option_count as usize) <= MAX_POLL_OPTIONS,
        PollRegistryError::BadOptionCount
    );
    let now = Clock::get()?.unix_timestamp;
    if has_expiry {
        require!(expires_at > now, PollRegistryError::ExpiryInPast);
    }

    let poll_id = ctx.accounts.creator_state.next_poll_id;

    let poll = &mut ctx.accounts.poll;
    poll.creator = ctx.accounts.creator.key();
    poll.creator_tid = ctx.accounts.creator_state.creator_tid;
    poll.poll_id = poll_id;
    poll.option_count = option_count;
    poll.option_votes = [0u32; MAX_POLL_OPTIONS];
    poll.total_votes = 0;
    poll.expires_at = expires_at;
    poll.has_expiry = has_expiry;
    poll.created_at = now;
    poll.metadata_hash = metadata_hash;
    poll.bump = ctx.bumps.poll;

    ctx.accounts.creator_state.next_poll_id = poll_id
        .checked_add(1)
        .expect("poll_id u64 overflow is unreachable in practice");

    emit!(PollCreated {
        poll: poll.key(),
        creator: poll.creator,
        creator_tid: poll.creator_tid,
        poll_id,
        option_count,
    });

    Ok(())
}
