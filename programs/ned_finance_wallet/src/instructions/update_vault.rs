use crate::errors::vaults::VaultsAccountsError;
use crate::state::vaults::{VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER, SpareType, VaultManager, VaultOwner};
use crate::utils::vaults::{get_name_array, name_is_empty};
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint};

#[derive(Accounts)]
#[instruction(identifier: [u8;22])]
pub struct UpdateSavingsAccountVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULTS_PDA_DATA, owner.key.as_ref()],
        bump,
    )]
    pub data_account: AccountLoader<'info, VaultManager>,

    pub mint: Account<'info, Mint>, 

    #[account(
        mut,
        seeds = [VAULTS_PDA_ACCOUNT_OWNER, owner.key.as_ref(), &identifier],
        bump
    )]
    pub vault_account_owner: Account<'info, VaultOwner>, // Program account to own token account

    #[account(
        mut,
        seeds = [VAULTS_PDA_ACCOUNT, owner.key.as_ref(), &identifier],
        bump,
        token::mint = mint, 
        token::authority = vault_account_owner,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<UpdateSavingsAccountVault>,
    _identifier: [u8;22],
    new_name:Vec<u8>,
    spare_type: SpareType,
    earnings_enabled: u8,
) -> Result<()> {
    // Accounts can't be empty
    if name_is_empty(&new_name) {
        return Err(error!(VaultsAccountsError::AccountNameEmpty));
    }

    let vault_account = &ctx.accounts.vault_account;
    let data_account = &mut ctx.accounts.data_account.load_mut()?;

    if data_account.owner.key() == ctx.accounts.owner.key() {
        let accounts = &mut data_account.accounts;

        let mut account_mut = accounts.iter_mut();

        let account_option = account_mut.find(|x| x.pub_key.key() == vault_account.key());

        if let Some(account_found) = account_option {

            let vault_with_spare_activated = account_mut.find(|x| x.spare_type > 0);

            if vault_with_spare_activated.is_some() && vault_with_spare_activated.unwrap().pub_key != account_found.pub_key && spare_type as u8 > 0 {
                return Err(VaultsAccountsError::VaultWithSpareMaxReached.into());
            } else {

                account_found.name_length = new_name.len() as u8;
                account_found.name = get_name_array(new_name.clone());
                account_found.spare_type = spare_type as u8;
                account_found.earnings_enabled = earnings_enabled;
                Ok(())
            }
        } else {
            return Err(error!(VaultsAccountsError::AccountNotFound));
        }
        
    } else {
        return Err(error!(VaultsAccountsError::ActionNotAllowed));
    }
}
