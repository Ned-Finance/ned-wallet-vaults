use crate::errors::vaults::VaultsAccountsError;
use crate::state::vaults::VAULTS_PDA_ACCOUNT_OWNER;
use crate::utils::meteora::MercurialVault;
// use affiliate::accounts::DepositWithdrawLiquidity;
use affiliate::{
    cpi::accounts::{InitUser, InitUserPermissionless},
    program::Affiliate,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mercurial_vault::{
    cpi::{accounts::DepositWithdrawLiquidity, deposit},
    state::Vault,
};

// use crate::state::vaults::{VaultManager, VaultOwner, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER};
// use crate::errors::vaults::VaultsAccountsError;
// use anchor_lang::prelude::*;
// use mercurial_vault::cpi::accounts::DepositWithdrawLiquidity;
// use mercurial_vault::cpi::*;
// use mercurial_vault::instruction::Deposit;
// use mercurial_vault::state::Vault;
// use crate::utils::meteora::MercurialVault;

pub fn get_name_array(name: Vec<u8>) -> [u8; 30] {
    let mut fixed: [u8; 30] = [0; 30];
    let name_as_array: &[u8] = &name;
    for (index, byte) in fixed.iter_mut().enumerate() {
        if let Some(found) = name_as_array.get(index) {
            *byte = *found
        }
    }
    return fixed;
}

pub fn name_is_empty(name: &Vec<u8>) -> bool {
    let name_with_chars = name.iter().find(|x| **x != 0);
    return name_with_chars.is_none();
}

pub fn deposit_liquidity<'info>(
    owner: &Signer<'info>,
    data_account: &mut std::cell::Ref<'_, crate::state::vaults::VaultManager>,
    vault_account: &Account<'_, anchor_spl::token::TokenAccount>,
    vault_program: &Program<'info, MercurialVault>,
    vault: &Box<Account<'info, Vault>>,
    vault_lp_mint: &Box<Account<'info, Mint>>,
    user_token: &UncheckedAccount<'info>,
    user_lp: &Box<Account<'info, TokenAccount>>,
    user: &UncheckedAccount<'info>,
    token_vault: &UncheckedAccount<'info>,
    token_program: &Program<'info, Token>,
    program_id: &Pubkey,
    _identifier: [u8; 22],
    amount: u64,
) -> Result<()> {
    if data_account.owner.key() == owner.key() {
        let accounts = &data_account.accounts;

        let account_found = accounts
            .iter()
            .find(|x| x.pub_key.key() == vault_account.key());

        if let Some(vault_account) = account_found {
            if vault_account.earnings_enabled == true {
                msg!("Deposit started {}", amount);

                // return Ok(());

                let (_account, bump) = Pubkey::find_program_address(
                    &[
                        VAULTS_PDA_ACCOUNT_OWNER,
                        owner.key.as_ref(),
                        &vault_account.identifier,
                    ],
                    program_id,
                );

                let seeds = &[
                    VAULTS_PDA_ACCOUNT_OWNER,
                    owner.key.as_ref(),
                    &vault_account.identifier,
                    &[bump],
                ];

                let signer = &[&seeds[..]];

                let accounts = DepositWithdrawLiquidity {
                    vault: vault.to_account_info(),
                    lp_mint: vault_lp_mint.to_account_info(),
                    user_token: user_token.to_account_info(),
                    user_lp: user_lp.to_account_info(),
                    user: user.to_account_info(),
                    token_vault: token_vault.to_account_info(),
                    token_program: token_program.to_account_info(),
                };

                let cpi_ctx =
                    CpiContext::new_with_signer(vault_program.to_account_info(), accounts, signer);
                let result = deposit(cpi_ctx, amount, 0);

                msg!("Deposit ended {}", amount);

                return result;
            } else {
                return Err(error!(VaultsAccountsError::EarningsNotEnabled));
            }
        } else {
            return Err(error!(VaultsAccountsError::AccountNotFound));
        }
    }
    Ok(())
}
