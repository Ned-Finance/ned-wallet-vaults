use crate::errors::vaults::VaultsAccountsError;
use crate::state::vaults::VAULTS_PDA_ACCOUNT_OWNER;
use crate::utils::meteora::MercurialVault;
// use affiliate::accounts::DepositWithdrawLiquidity;
use affiliate::{
    cpi::{
        accounts::{DepositWithdrawLiquidity, InitUser, InitUserPermissionless},
        *,
    },
    program::Affiliate,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mercurial_vault::state::Vault;

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

// #[account(mut)]
//     pub vault: Box<Account<'info, Vault>>,
//     /// CHECK:
//     #[account(mut)]
//     pub token_vault: UncheckedAccount<'info>,
//     /// CHECK:
//     #[account(mut)]
//     pub vault_lp_mint: Box<Account<'info, Mint>>,
//     /// CHECK:
//     #[account(mut)]
//     pub user: UncheckedAccount<'info>,
//     /// CHECK:
//     #[account(mut)]
//     pub user_token: UncheckedAccount<'info>,
//     /// CHECK:
//     #[account(mut, constraint = user_lp.owner == vault_account_owner.key())] //mint to account of user PDA
//     pub user_lp: Box<Account<'info, TokenAccount>>,
//     /// CHECK:
//     pub token_program: Program<'info, Token>,

pub fn deposit_liquidity<'info>(
    owner: &Signer<'info>,
    partner: &UncheckedAccount<'info>,
    vault_account: &Account<'_, anchor_spl::token::TokenAccount>,
    data_account: &mut std::cell::Ref<'_, crate::state::vaults::VaultManager>,
    vault: &Box<Account<'info, Vault>>,
    vault_lp_mint: &Box<Account<'info, Mint>>,
    user_token: &UncheckedAccount<'info>,
    user_lp: &Box<Account<'info, TokenAccount>>,
    user: &UncheckedAccount<'info>,
    token_vault: &UncheckedAccount<'info>,
    token_program: &Program<'info, Token>,
    system_program: &Program<'info, System>,
    rent: &Sysvar<'info, Rent>,
    program_id: &Pubkey,
    vault_program: &Program<'info, MercurialVault>,
    affiliate_program: &Program<'info, Affiliate>,
    _identifier: [u8; 22],
    amount: u64,
) -> Result<()> {
    // let vault_account = &ctx.accounts.vault_account;
    // let data_account = &mut ctx.accounts.data_account.load()?;
    if data_account.owner.key() == owner.key() {
        let accounts = &data_account.accounts;

        let account_found = accounts
            .iter()
            .find(|x| x.pub_key.key() == vault_account.key());

        if let Some(vault_account) = account_found {
            if vault_account.earnings_enabled == 1 {
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

                if user.lamports() == 0 {
                    msg!("User account not initialized");
                    let accounts_init_user = InitUserPermissionless {
                        user: user.to_account_info(),
                        partner: partner.to_account_info(),
                        owner: owner.to_account_info(),
                        payer: owner.to_account_info(),
                        system_program: system_program.to_account_info(),
                    };

                    let cpi_ctx =
                        CpiContext::new(affiliate_program.to_account_info(), accounts_init_user);
                    let result = init_user_permissionless(cpi_ctx);

                    match result {
                        Ok(()) => msg!("User was created sucessfully"),
                        Err(err) => msg!("There was an error creating user {:?}", err),
                    }
                } else {
                    msg!("User account is not initialized");
                }

                // let accounts = DepositWithdrawLiquidity {
                //     vault: vault.to_account_info(),
                //     owner: owner.to_account_info(),
                //     user_token: user_token.to_account_info(),
                //     user_lp: user_lp.to_account_info(),
                //     user: user.to_account_info(),
                //     vault_lp_mint: vault_lp_mint.to_account_info(),
                //     token_vault: token_vault.to_account_info(),
                //     token_program: token_program.to_account_info(),
                //     vault_program: vault_program.to_account_info(),
                //     partner: partner.to_account_info(),
                // };

                // let cpi_ctx = CpiContext::new_with_signer(
                //     affiliate_program.to_account_info(),
                //     accounts,
                //     signer,
                // );
                // let result = deposit(cpi_ctx, amount, 0);

                // msg!("Deposit ended {}", amount);

                // return result;
            } else {
                return Err(error!(VaultsAccountsError::EarningsNotEnabled));
            }
        } else {
            return Err(error!(VaultsAccountsError::AccountNotFound));
        }
    }
    Ok(())
}
