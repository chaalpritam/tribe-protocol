use anchor_lang::prelude::*;

#[event]
pub struct ChannelRegistered {
    pub channel: Pubkey,
    pub owner: Pubkey,
    pub owner_tid: u64,
    pub kind: u8,
}

#[event]
pub struct ChannelUpdated {
    pub channel: Pubkey,
    pub owner: Pubkey,
}

#[event]
pub struct ChannelTransferred {
    pub channel: Pubkey,
    pub previous_owner: Pubkey,
    pub previous_owner_tid: u64,
    pub new_owner: Pubkey,
    pub new_owner_tid: u64,
}
