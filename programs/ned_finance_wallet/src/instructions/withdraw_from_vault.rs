use crate::errors::vaults::VaultsAccountsError;
use crate::state::vaults::{VaultManager, VaultOwner, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint, Token, Transfer};

#[derive(Accounts)]
#[instruction(identifier: [u8;22])]
pub struct WithdrawFromVault<'info> {

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
}

impl <'info> WithdrawFromVault<'info> {
    fn withdraw(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info().clone(), 
            Transfer {
                from: self.vault_account.to_account_info().clone(),
                to: self.user_token_account.to_account_info().clone(),
                authority: self.vault_account_owner.to_account_info().clone(),
            }
        )
    }
}

pub fn handler(
    ctx: Context<WithdrawFromVault>,
    _identifier: [u8;22],
    amount:u64
) -> Result<()> {

    let data_account = &mut ctx.accounts.data_account.load()?;
    if data_account.owner.key() == ctx.accounts.owner.key() {
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
                    .withdraw()
                    .with_signer(&[&seeds[..]]),
                    amount,
            )?;
        }
        Ok(())
    } else {
        return Err(error!(VaultsAccountsError::ActionNotAllowed));
    }
}
