use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount };
use anchor_spl::associated_token::{AssociatedToken};
use anchor_lang::solana_program::pubkey;
// use anchor_spl::token::{self, Burn, Mint, MintTo, SetAuthority, TokenAccount, Transfer};

declare_id!("5J8oWg8KbXvNqe64tpzCsbUdtVpBiHn6roFRrQ73FNMQ");

const SAVINGS_PDA: &[u8] = b"SAVINGS_PDA";

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum AccountType{
    MANUAL = 1,
}

#[program]
pub mod ned_wallet_program {

    use super::*;

    pub fn create_savings_vault(ctx: Context<CreateSavingsAccount>, name: Vec<u8>, account_type: AccountType) -> Result<()> {

        msg!("----> {:?}", account_type as u8);

        let default_pubkey  = pubkey!("11111111111111111111111111111111"); 
        
        let data_account_loaded = &mut ctx.accounts.data_account.load_init();
        

        if data_account_loaded.is_ok() {

            let vault_account = &mut ctx.accounts.vault_account;

            let data_account = &mut data_account_loaded.as_mut().unwrap();

            data_account.owner = ctx.accounts.owner.key();

            let user_accounts = &mut data_account.accounts;
            

            let first_available_slot_index = user_accounts.iter().position(|x| x.pub_key == default_pubkey);

            if first_available_slot_index.is_some() {

                    let account_to_replace = &mut user_accounts[first_available_slot_index.unwrap()];

                    account_to_replace.pub_key = vault_account.key();

                    let name_as_array:&[u8] = &name;


                    let mut fixed:[u8; 50] = [0; 50];
                    for (index, byte) in fixed.iter_mut().enumerate() {
                        if let Some(found) = name_as_array.get(index) {
                            *byte = *found
                        }
                    }
                    
                    account_to_replace.name = fixed;
                    account_to_replace.name_length = name_as_array.len() as u8;
                    account_to_replace.name_length = account_type as u8;
                    
                    msg!("---> {:?}", account_to_replace.name);
                    msg!("---> {:?}", account_to_replace.name_length);

                    return Ok(());
                
            } else {
                return Err(error!(SavingsAccountsError::MaxAccountsReached))
            }            

        } else {
            return Err(error!(SavingsAccountsError::AlreadyInitialized));
        };

    }
}

#[zero_copy]
#[derive(Debug)]
pub struct SavingAccount {
    name: [u8; 50],
    name_length: u8,
    pub_key: Pubkey,
    account_type: u8
}

impl SavingAccount {
    pub const SIZE: usize = 8 + 50 + 32;
}



#[account(zero_copy)]
pub struct UserSavingsManager {
    pub owner: Pubkey,
    pub accounts: [SavingAccount; 20],
}

impl UserSavingsManager {
    pub const MAX_ACCOUNTS:usize = 20;
    pub const MAX_SIZE_ACCOUNTS_ARRAY:usize = SavingAccount::SIZE * UserSavingsManager::MAX_ACCOUNTS; //20 * (150 + 32)
    pub const LEN: usize = 8 + 32 + UserSavingsManager::MAX_SIZE_ACCOUNTS_ARRAY;
}

#[derive(Accounts)]
#[instruction(name: Vec<u8>)]
pub struct CreateSavingsAccount<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [SAVINGS_PDA, owner.key.as_ref()],
        bump,
        payer = owner,
        space = UserSavingsManager::LEN + 8
    )]
    pub data_account: AccountLoader<'info, UserSavingsManager>, // Program account to store data

    pub mint: Account<'info, Mint>, 

    #[account(
        init_if_needed,
        seeds = [SAVINGS_PDA, owner.key.as_ref(), &name],
        bump,
        payer = owner,
        token::mint = mint, 
        token::authority = owner,
    )]
    pub vault_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}



#[error_code]
pub enum SavingsAccountsError {
    #[msg("This account was already initialized")]
    AlreadyInitialized,
    #[msg("Max accounts reached")]
    MaxAccountsReached,
}