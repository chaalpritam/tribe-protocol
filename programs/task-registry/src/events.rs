use anchor_lang::prelude::*;

#[event]
pub struct CreatorStateInitialized {
    pub creator: Pubkey,
    pub creator_tid: u64,
}

#[event]
pub struct TaskCreated {
    pub task: Pubkey,
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub task_id: u64,
    pub reward_amount: u64,
}

#[event]
pub struct TaskClaimed {
    pub task: Pubkey,
    pub claimer: Pubkey,
    pub claimer_tid: u64,
}

#[event]
pub struct TaskCompleted {
    pub task: Pubkey,
    pub creator: Pubkey,
    pub claimer: Pubkey,
    pub reward_amount: u64,
}

#[event]
pub struct TaskCancelled {
    pub task: Pubkey,
    pub creator: Pubkey,
    pub refunded: u64,
}
