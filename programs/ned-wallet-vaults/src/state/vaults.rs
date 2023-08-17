use anchor_lang::prelude::*;

pub const VAULTS_PDA_DATA: &[u8] = b"VAULTS_PDA_DATA";
pub const VAULTS_PDA_ACCOUNT: &[u8] = b"VAULTS_PDA_ACCOUNT";

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum SpareType {
    NONE = 0,
    SPARE = 1,
    SPARE2X = 2,
    SPARE3X = 3,
}

#[zero_copy]
#[derive(Debug)]
pub struct VaultAccount {
    pub name: [u8; 30],
    pub name_length: u8,
    pub pub_key: Pubkey,
    pub token_pub_key: Pubkey,
    pub spare_type: u8,
    pub automatic_days_period: u8,
    pub identifier: [u8; 22],
}

impl VaultAccount {
    pub const SIZE: usize = 30 + 1 + 32 + 32 + 1 + 1 + 22;
}

#[account(zero_copy)]
pub struct VaultManager {
    pub owner: Pubkey,
    pub accounts: [VaultAccount; 20],
}

impl VaultManager {
    pub const MAX_ACCOUNTS: usize = 20;
    pub const MAX_SIZE_ACCOUNTS_ARRAY: usize = VaultAccount::SIZE * VaultManager::MAX_ACCOUNTS; //20 * (150 + 32)
    pub const LEN: usize = 8 + 32 + VaultManager::MAX_SIZE_ACCOUNTS_ARRAY;
}

#[account]
pub struct VaultOwner {
    pub owner: Pubkey,
}

impl VaultOwner {
    pub const LEN: usize = 8 + 32;
}
