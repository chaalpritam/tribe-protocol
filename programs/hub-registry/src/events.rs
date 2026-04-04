use anchor_lang::prelude::*;

#[event]
pub struct HubRegistered {
    pub operator: Pubkey,
    pub gossip_key: Pubkey,
}

#[event]
pub struct HubUpdated {
    pub operator: Pubkey,
}

#[event]
pub struct HubDeactivated {
    pub operator: Pubkey,
}

#[event]
pub struct HubHeartbeat {
    pub operator: Pubkey,
}
