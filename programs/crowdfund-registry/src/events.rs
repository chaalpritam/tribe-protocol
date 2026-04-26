use anchor_lang::prelude::*;

#[event]
pub struct CreatorStateInitialized {
    pub creator: Pubkey,
    pub creator_tid: u64,
}

#[event]
pub struct CrowdfundCreated {
    pub crowdfund: Pubkey,
    pub creator: Pubkey,
    pub creator_tid: u64,
    pub crowdfund_id: u64,
    pub goal_amount: u64,
    pub deadline_at: i64,
}

#[event]
pub struct CrowdfundPledged {
    pub crowdfund: Pubkey,
    pub backer: Pubkey,
    pub backer_tid: u64,
    pub amount: u64,
    pub total_pledged: u64,
    pub pledge_count: u32,
}

#[event]
pub struct CrowdfundClaimed {
    pub crowdfund: Pubkey,
    pub creator: Pubkey,
    pub total_pledged: u64,
}

#[event]
pub struct CrowdfundRefunded {
    pub crowdfund: Pubkey,
    pub backer: Pubkey,
    pub amount: u64,
}
