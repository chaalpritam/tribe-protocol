use anchor_lang::prelude::*;

use crate::errors::PollRegistryError;
use crate::events::PollVoted;
use crate::state::{Poll, Vote as VoteState};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [
            b"poll",
            poll.creator.as_ref(),
            &poll.poll_id.to_le_bytes(),
        ],
        bump = poll.bump,
    )]
    pub poll: Account<'info, Poll>,

    /// `init` is the one-vote-per-TID guard: a second vote attempt
    /// from the same wallet on the same poll fails because the PDA
    /// already exists.
    #[account(
        init,
        payer = voter,
        space = VoteState::SIZE,
        seeds = [b"poll-vote", poll.key().as_ref(), voter.key().as_ref()],
        bump,
    )]
    pub vote_record: Account<'info, VoteState>,

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Vote>, voter_tid: u64, option_index: u8) -> Result<()> {
    let poll = &ctx.accounts.poll;
    require!(
        option_index < poll.option_count,
        PollRegistryError::OptionOutOfRange
    );
    require!(
        poll.creator != ctx.accounts.voter.key(),
        PollRegistryError::SelfVote
    );
    if poll.has_expiry {
        let now = Clock::get()?.unix_timestamp;
        require!(now < poll.expires_at, PollRegistryError::PollExpired);
    }

    let now = Clock::get()?.unix_timestamp;

    let vote_record = &mut ctx.accounts.vote_record;
    vote_record.poll = ctx.accounts.poll.key();
    vote_record.voter = ctx.accounts.voter.key();
    vote_record.voter_tid = voter_tid;
    vote_record.option_index = option_index;
    vote_record.voted_at = now;
    vote_record.bump = ctx.bumps.vote_record;

    let poll = &mut ctx.accounts.poll;
    let idx = option_index as usize;
    poll.option_votes[idx] = poll.option_votes[idx]
        .checked_add(1)
        .expect("option vote count u32 overflow unreachable in practice");
    poll.total_votes = poll
        .total_votes
        .checked_add(1)
        .expect("total_votes u32 overflow unreachable in practice");

    emit!(PollVoted {
        poll: poll.key(),
        voter: vote_record.voter,
        voter_tid,
        option_index,
        new_total_for_option: poll.option_votes[idx],
    });

    Ok(())
}
