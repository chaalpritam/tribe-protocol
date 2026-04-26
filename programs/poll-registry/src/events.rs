use anchor_lang::prelude::*;

#[event]
pub struct CreatorStateInitialized {
    pub creator: Pubkey,
    pub creator_tid: u64,
}

#[event]
pub struct PollCreated {
    pub poll: Pubkey,
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub poll_id: u64,
    pub option_count: u8,
}

#[event]
pub struct PollVoted {
    pub poll: Pubkey,
    pub voter: Pubkey,
    pub voter_tid: u64,
    pub option_index: u8,
    pub new_total_for_option: u32,
}
