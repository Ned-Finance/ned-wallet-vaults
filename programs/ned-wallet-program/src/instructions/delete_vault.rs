use crate::errors::vaults::VaultsAccountsError;
use crate::state::vaults::{VaultManager, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT};
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};
use anchor_lang::solana_program::pubkey;

#[derive(Accounts)]
#[instruction(identifier:[u8;22])]
pub struct DeleteSavingsAccountVault<'info> {
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

    #[account(
        mut,
        token::mint = mint, 
        token::authority = owner,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<DeleteSavingsAccountVault>,
    identifier:[u8;22],
) -> Result<()> {

    let vault_account = &ctx.accounts.vault_account;
    let mut data_account = ctx.accounts.data_account.load_mut()?;

    if data_account.owner.key() == ctx.accounts.owner.key() {
        let accounts = &mut data_account.accounts;
        
        let account = accounts.iter_mut().find(|x| x.pub_key.key() == vault_account.key());
        
        if let Some(account) = account {

            account.name = [0;30];
            account.spare_type = 0;
            account.pub_key = pubkey!("11111111111111111111111111111111");
            account.name_length = 0;
            account.identifier = [0;22];;
            
            drop(data_account);

            let close_instruction = anchor_spl::token::CloseAccount {
                account: vault_account.to_account_info().clone(),
                destination: ctx.accounts.user_token_account.to_account_info().clone(),
                authority: ctx.accounts.data_account.to_account_info(),
            };


            let (_account, _bump) =
                Pubkey::find_program_address(&[
                    VAULTS_PDA_DATA,  
                    ctx.accounts.owner.key.as_ref(), 
                ], ctx.program_id);

            let seeds = &[
                VAULTS_PDA_DATA, 
                ctx.accounts.owner.key.as_ref(),
                &[_bump]
            ];

            let signer = &[&seeds[..]];

            let cpi_program = ctx.accounts.token_program.to_account_info().clone();
            let cpi_ctx = CpiContext::new_with_signer(
                cpi_program,
                close_instruction,
                signer
            );
            anchor_spl::token::close_account(cpi_ctx)?;

            Ok(())
        } else {
            return Err(error!(VaultsAccountsError::AccountNotFound));
        }
    } else {
        return Err(error!(VaultsAccountsError::ActionNotAllowed));
    }
}
