use anchor_lang::prelude::*;

#[event]
pub struct KarmaAccountInitialized {
    pub karma: Pubkey,
    pub tid: u64,
}

#[event]
pub struct TipKarmaRecorded {
    pub karma: Pubkey,
    pub tid: u64,
    pub tip_record: Pubkey,
    pub amount: u64,
    pub new_tip_count: u64,
    pub new_tip_lamports: u64,
}

#[event]
pub struct TaskKarmaRecorded {
    pub karma: Pubkey,
    pub tid: u64,
    pub task: Pubkey,
    pub reward_amount: u64,
    pub new_task_count: u64,
    pub new_task_reward_lamports: u64,
}
