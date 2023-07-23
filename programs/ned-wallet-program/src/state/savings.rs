use anchor_lang::prelude::*;

pub const SAVINGS_PDA: &[u8] = b"SAVINGS_PDA";

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum SpareType {
    NONE = 0,
    SPARE = 1,
    SPARE2X = 2,
    SPARE3X = 3,
}

#[zero_copy]
#[derive(Debug)]
pub struct SavingAccount {
    pub name: [u8; 30],
    pub name_length: u8,
    pub pub_key: Pubkey,
    pub spare_type: u8,
    pub automatic_days_period: u8,
    pub identifier: [u8; 22],
}

impl SavingAccount {
    pub const SIZE: usize = 30 + 1 + 32 + 1 + 1 + 22;
}

#[account(zero_copy)]
pub struct UserSavingsManager {
    pub owner: Pubkey,
    pub accounts: [SavingAccount; 20],
}

impl UserSavingsManager {
    pub const MAX_ACCOUNTS: usize = 20;
    pub const MAX_SIZE_ACCOUNTS_ARRAY: usize =
        SavingAccount::SIZE * UserSavingsManager::MAX_ACCOUNTS; //20 * (150 + 32)
    pub const LEN: usize = 8 + 32 + UserSavingsManager::MAX_SIZE_ACCOUNTS_ARRAY;
}
