use crate::errors::vaults::VaultsAccountsError;
use crate::state::vaults::{VaultManager, VaultOwner, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Mint, Token, Transfer};
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

impl <'info> DeleteSavingsAccountVault<'info> {
    fn transfer_token(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.user_token_account.to_account_info().clone(),
            authority: self.vault_account_owner.to_account_info().clone(),
        };
        CpiContext::new(self.token_program.to_account_info().clone(), cpi_accounts)
    }

}

pub fn handler(
    ctx: Context<DeleteSavingsAccountVault>,
    _identifier:[u8;22],
) -> Result<()> {

    let vault_account = &ctx.accounts.vault_account;
    let mut data_account = ctx.accounts.data_account.load_mut()?;

    if data_account.owner.key() == ctx.accounts.owner.key() {
        let accounts = &mut data_account.accounts;
        
        let account = accounts.iter_mut().find(|x| x.pub_key.key() == vault_account.key());
        
        if let Some(account) = account {

            let identifier = account.identifier.clone();

            account.name = [0;30];
            account.spare_type = 0;
            account.pub_key = pubkey!("11111111111111111111111111111111");
            account.name_length = 0;
            account.identifier = [0;22];
            
            drop(data_account);

            let (_account, _bump) =
                Pubkey::find_program_address(&[
                    VAULTS_PDA_ACCOUNT_OWNER,  
                    ctx.accounts.owner.key.as_ref(), 
                    &identifier,
                ], ctx.program_id);

            let seeds = &[
                VAULTS_PDA_ACCOUNT_OWNER, 
                ctx.accounts.owner.key.as_ref(),
                &identifier,
                &[_bump]
            ];

            let signer = &[&seeds[..]];

            let cpi_program = ctx.accounts.token_program.to_account_info().clone();

            token::transfer(
                ctx.accounts
                    .transfer_token()
                    .with_signer(signer),
                    vault_account.amount,
            )?;
    
            //Close NED vault
            let close_instruction = anchor_spl::token::CloseAccount {
                account: vault_account.to_account_info().clone(),
                destination: ctx.accounts.user_token_account.to_account_info().clone(),
                authority: ctx.accounts.vault_account_owner.to_account_info(),
            };

            let cpi_ctx = CpiContext::new_with_signer(
                cpi_program,
                close_instruction,
                signer
            );
            anchor_spl::token::close_account(cpi_ctx)?;
    
            //Close NED vault owner
            ctx.accounts.vault_account_owner.close(ctx.accounts.owner.to_account_info())?;

            Ok(())
        } else {
            return Err(error!(VaultsAccountsError::AccountNotFound));
        }
    } else {
        return Err(error!(VaultsAccountsError::ActionNotAllowed));
    }
}
