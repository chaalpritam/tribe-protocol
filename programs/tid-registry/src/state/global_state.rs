use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    /// Auto-incrementing TID counter.
    pub tid_counter: u64,
    /// Authority that initialized the program (governance).
    pub authority: Pubkey,
    pub bump: u8,
}

impl GlobalState {
    pub const SIZE: usize = 8 + 8 + 32 + 1; // discriminator + fields

    pub fn next_tid(&mut self) -> u64 {
        self.tid_counter += 1;
        self.tid_counter
    }
}
