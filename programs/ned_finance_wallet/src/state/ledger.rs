use anchor_lang::prelude::*;

pub const LEDGER_PDA_DATA: &[u8] = b"LEDGER_PDA_DATA";

#[account]
pub struct LedgerStore {
    pub amount: u64,
}

impl LedgerStore {
    pub const SIZE: usize = 8 + 64;
}
