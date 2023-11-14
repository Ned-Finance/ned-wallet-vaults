use crate::errors::vaults::VaultsAccountsError;
use crate::state::ledger::{LedgerStore, LEDGER_PDA_DATA};
use crate::state::vaults::{VaultManager, VaultOwner, VAULTS_PDA_ACCOUNT_OWNER, VAULTS_PDA_ACCOUNT, VAULTS_PDA_DATA};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint, Token, Transfer};
use anchor_lang::solana_program::sysvar;

#[derive(Accounts)]
#[instruction(identifier: [u8;22])]
pub struct DepositToVaultWithDiffBalance<'info> {

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

    #[account(
        mut,
        token::mint = mint, 
        token::authority = owner,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    /// CHECK: check instructions account
    #[account(address = sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [LEDGER_PDA_DATA, owner.key.as_ref()],
        bump,
    )]
    pub ledger_data: Account<'info, LedgerStore>, 
}

impl <'info> DepositToVaultWithDiffBalance<'info> {
    fn deposit(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info().clone(), 
            Transfer {
                from: self.user_token_account.to_account_info().clone(),
                to: self.vault_account.to_account_info().clone(),
                authority: self.owner.to_account_info().clone(),
            }
        )
    }
}

pub fn handler(
    ctx: Context<DepositToVaultWithDiffBalance>,
    _identifier: [u8;22],
) -> Result<()> {
    let data_account = &mut ctx.accounts.data_account.load()?;
    if data_account.owner.key() == ctx.accounts.owner.key() {

        let account_balance = ctx.accounts.ledger_data.amount;

        let account_current_balance = ctx.accounts.user_token_account.amount;
        // let diff_balance_in_account: u64 =  account_current_balance - account_balance;
        let diff_balance_in_account: i64 =  account_current_balance as i64 - account_balance as i64;
        
        msg!("Account intial balance {:?}", account_balance);
        msg!("Account current balance {:?}", account_current_balance);
        msg!("Amount to deposit: {:?}", diff_balance_in_account);
        msg!("Amount to deposit abs: {:?}", diff_balance_in_account.abs());

        let accounts = &data_account.accounts;

        let account_found = accounts.iter().find(
            |x| x.pub_key.key() == ctx.accounts.vault_account.key());

        if let Some(account) = account_found {
            let (_vault_account_owner, bump) =
                Pubkey::find_program_address(
                    &[
                        VAULTS_PDA_ACCOUNT_OWNER, 
                        ctx.accounts.owner.key.as_ref(),
                        &account.identifier
                    ], 
                    ctx.program_id,
                );

            let seeds = &[
                &VAULTS_PDA_ACCOUNT_OWNER, 
                ctx.accounts.owner.key.as_ref(), 
                &account.identifier,
                &[bump]
            ];

            token::transfer(
                ctx.accounts
                    .deposit()
                    .with_signer(&[&seeds[..]]),
                    diff_balance_in_account.abs() as u64,
            )?;

            ctx.accounts.ledger_data.amount = diff_balance_in_account.abs() as u64;

            return Ok(())

        } else {
            return Err(error!(VaultsAccountsError::AccountNotFound));
        }

    } else {
        return Err(error!(VaultsAccountsError::ActionNotAllowed));
    }
}
