use crate::state::vaults::{VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER, VaultManager, VaultOwner, SpareType};
use crate::errors::vaults::VaultsAccountsError;
use crate::utils::vaults::{name_is_empty, get_name_array};

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount };
use anchor_spl::associated_token::AssociatedToken;
use anchor_lang::solana_program::pubkey;



#[derive(Accounts)]
#[instruction(name: Vec<u8>, identifier: [u8;22])]
pub struct CreateVault<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [VAULTS_PDA_DATA, owner.key.as_ref()],
        bump,
        payer = owner,
        space = VaultManager::LEN + 8
    )]
    pub data_account: AccountLoader<'info, VaultManager>, // Program account to store data

    pub mint: Account<'info, Mint>, 

    #[account(
        init_if_needed,
        seeds = [VAULTS_PDA_ACCOUNT, owner.key.as_ref(), &identifier],
        bump,
        payer = owner,
        // owner = vault_owner.key(),
        token::mint = mint, 
        token::authority = data_account,
    )]
    pub vault_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateVault>, name: Vec<u8>, identifier:[u8;22], spare_type: SpareType) -> Result<()> {
    // Accounts can't be empty
    if name_is_empty(&name) {
        return Err(error!(VaultsAccountsError::AccountNameEmpty));
    }

    let default_pubkey  = pubkey!("11111111111111111111111111111111"); 

    let data_account = &mut match ctx.accounts.data_account.load_init() {
        Ok(result) => result,
        Err(_) => ctx.accounts.data_account.load_mut().unwrap()
    };
    

    let vault_account = &mut ctx.accounts.vault_account;
    let mint = &mut ctx.accounts.mint;
    
    data_account.owner = ctx.accounts.owner.key();

    let user_accounts = &mut data_account.accounts;
    let first_available_slot_index = user_accounts.iter().position(|x| x.pub_key == default_pubkey);

    if first_available_slot_index.is_some() {

        let index = first_available_slot_index.unwrap();

            let account_to_replace = &mut user_accounts[index];

            account_to_replace.pub_key = vault_account.key();
            account_to_replace.token_pub_key = mint.key();
            
            account_to_replace.name = get_name_array(name.clone());
            account_to_replace.name_length = (&name).len() as u8;
            account_to_replace.spare_type = spare_type as u8;
            account_to_replace.identifier = identifier;

            return Ok(());
        
    } else {
        return Err(error!(VaultsAccountsError::MaxAccountsReached))
    }            

  
}