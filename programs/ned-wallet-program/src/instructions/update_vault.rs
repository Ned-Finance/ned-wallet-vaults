use crate::errors::vaults::VaultsAccountsError;
use crate::state::vaults::{SpareType, VaultManager, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT};
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
        seeds = [VAULTS_PDA_ACCOUNT, owner.key.as_ref(), &identifier],
        bump,
        token::mint = mint, 
        token::authority = data_account,
    )]
    pub vault_account: Account<'info, TokenAccount>,
}

pub fn handler(
    ctx: Context<UpdateSavingsAccountVault>,
    _identifier: [u8;22],
    new_name:Vec<u8>,
    spare_type: SpareType,
) -> Result<()> {
    // Accounts can't be empty
    if name_is_empty(&new_name) {
        return Err(error!(VaultsAccountsError::AccountNameEmpty));
    }

    let vault_account = &ctx.accounts.vault_account;
    let data_account = &mut ctx.accounts.data_account.load_mut()?;
    if data_account.owner.key() == ctx.accounts.owner.key() {
        let account = data_account
            .accounts
            .iter_mut()
            .find(|x| x.pub_key.key() == vault_account.key());
        if let Some(account_found) = account {
            account_found.name = get_name_array(&new_name);
            account_found.name_length = new_name.len() as u8;
            account_found.spare_type = spare_type as u8;
            Ok(())
        } else {
            return Err(error!(VaultsAccountsError::AccountNotFound));
        }
    } else {
        return Err(error!(VaultsAccountsError::ActionNotAllowed));
    }
}
