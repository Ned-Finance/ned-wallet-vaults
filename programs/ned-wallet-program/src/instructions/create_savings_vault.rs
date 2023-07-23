use crate::state::savings::{SAVINGS_PDA, UserSavingsManager, SpareType};
use crate::errors::savings::SavingsAccountsError;
use crate::utils::savings::{name_is_empty, get_name_array};

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount };
use anchor_spl::associated_token::AssociatedToken;
use anchor_lang::solana_program::pubkey;



#[derive(Accounts)]
#[instruction(name: Vec<u8>, identifier: [u8;22])]
pub struct CreateSavingsVault<'info> {

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
        seeds = [SAVINGS_PDA, owner.key.as_ref(), &identifier],
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

pub fn handler(ctx: Context<CreateSavingsVault>, name: Vec<u8>, identifier:[u8;22], spare_type: SpareType) -> Result<()> {
    // Accounts can't be empty
    if name_is_empty(&name) {
        return Err(error!(SavingsAccountsError::AccountNameEmpty));
    }

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
                
                account_to_replace.name = get_name_array(&name);
                account_to_replace.name_length = (&name).len() as u8;
                account_to_replace.spare_type = spare_type as u8;
                account_to_replace.identifier = identifier;

                return Ok(());
            
        } else {
            return Err(error!(SavingsAccountsError::MaxAccountsReached))
        }            

    } else {
        return Err(error!(SavingsAccountsError::AlreadyInitialized));
    };
}