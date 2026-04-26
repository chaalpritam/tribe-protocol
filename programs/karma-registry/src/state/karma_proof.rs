use anchor_lang::prelude::*;

/// One per (source record, karma account) pair. Lives at PDA
/// `["karma-proof", source_record_pubkey]`. Existence is the only
/// signal we read — `init` on the account guarantees that the same
/// source record can't be credited twice.
#[account]
pub struct KarmaProof {
    pub source: Pubkey,
    /// 1 = tip-registry::TipRecord, 2 = task-registry::Task.
    pub kind: u8,
    pub tid: u64,
    pub bump: u8,
}

impl KarmaProof {
    // discriminator(8) + source(32) + kind(1) + tid(8) + bump(1)
    pub const SIZE: usize = 8 + 32 + 1 + 8 + 1;
}

pub const KARMA_PROOF_KIND_TIP: u8 = 1;
pub const KARMA_PROOF_KIND_TASK: u8 = 2;
