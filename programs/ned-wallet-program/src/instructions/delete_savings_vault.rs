use crate::errors::savings::SavingsAccountsError;
use crate::state::savings::{UserSavingsManager, SAVINGS_PDA};
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
        seeds = [SAVINGS_PDA, owner.key.as_ref()],
        bump,
    )]
    pub data_account: AccountLoader<'info, UserSavingsManager>,

    pub mint: Account<'info, Mint>, 

    #[account(
        mut,
        seeds = [SAVINGS_PDA, owner.key.as_ref(), &identifier],
        bump,
        token::mint = mint, 
        token::authority = owner,
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
    let data_account = &mut ctx.accounts.data_account.load_mut()?;
    if data_account.owner.key() == ctx.accounts.owner.key() {
        let account = data_account
            .accounts
            .iter_mut()
            .find(|x| x.pub_key.key() == vault_account.key());
        if let Some(account_found) = account {
            account_found.name = [0;30];
            account_found.account_type = 0;
            account_found.pub_key = pubkey!("11111111111111111111111111111111");
            account_found.name_length = 0;

            let close_instruction = anchor_spl::token::CloseAccount {
                account: ctx.accounts.vault_account.to_account_info().clone(),
                destination: ctx.accounts.user_token_account.to_account_info().clone(),
                authority: ctx.accounts.owner.to_account_info().clone(),
            };


            let (_account, _bump) =
                Pubkey::find_program_address(&[
                    SAVINGS_PDA,  
                    ctx.accounts.owner.key.as_ref(), 
                    &identifier
                ], ctx.program_id);

            let seeds = &[
                SAVINGS_PDA, 
                ctx.accounts.owner.key.as_ref(),
                &identifier,
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
            return Err(error!(SavingsAccountsError::AccountNotFound));
        }
    } else {
        return Err(error!(SavingsAccountsError::ActionNotAllowed));
    }
}
