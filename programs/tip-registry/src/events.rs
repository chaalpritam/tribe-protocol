use anchor_lang::prelude::*;

#[event]
pub struct SenderTipStateInitialized {
    pub sender: Pubkey,
    pub sender_tid: u64,
}

#[event]
pub struct TipSent {
    pub sender: Pubkey,
    pub recipient: Pubkey,
    pub sender_tid: u64,
    pub recipient_tid: u64,
    pub amount: u64,
    pub tip_id: u64,
    pub has_target: bool,
    pub target_hash: [u8; 32],
}
