use crate::state::vaults::{VaultManager, VaultOwner, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER};
use crate::errors::vaults::VaultsAccountsError;
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Mint};
use mercurial_vault::cpi::accounts::DepositWithdrawLiquidity;
use mercurial_vault::cpi::*;
use mercurial_vault::instruction::Deposit;
use mercurial_vault::state::Vault;
use crate::utils::meteora::MercurialVault;

#[derive(Accounts)]
#[instruction(identifier: [u8;22])]
pub struct DepositLiquidity<'info> {
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


    /// CHECK:
    pub vault_program: Program<'info, MercurialVault>,
    /// CHECK:
    #[account(mut)]
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK:
    #[account(mut)]
    pub token_vault: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub vault_lp_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    #[account(mut)]
    pub user: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub user_token: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut, constraint = user_lp.owner == vault_account_owner.key())] //mint to account of user PDA
    pub user_lp: Box<Account<'info, TokenAccount>>,
    /// CHECK:
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<DepositLiquidity>,
    _identifier: [u8;22],
    amount:u64,
) -> Result<()> {

    let vault_account = &ctx.accounts.vault_account;
    let data_account = &mut ctx.accounts.data_account.load()?;
    if data_account.owner.key() == ctx.accounts.owner.key() {

        let accounts = &data_account.accounts;

        let account_found = accounts.iter()
            .find(|x| x.pub_key.key() == vault_account.key());

        if let Some(vault_account) = account_found {

            if vault_account.earnings_enabled == true {
                
                msg!("Deposit started {}", amount);

                let accounts = DepositWithdrawLiquidity {
                    vault: ctx.accounts.vault.to_account_info(),
                    lp_mint: ctx.accounts.vault_lp_mint.to_account_info(),
                    user_token: ctx.accounts.user_token.to_account_info(),
                    user_lp: ctx.accounts.user_lp.to_account_info(),
                    user: ctx.accounts.user.to_account_info(),
                    token_vault: ctx.accounts.token_vault.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                };
        
                let (_account, bump) =
                        Pubkey::find_program_address(&[
                            VAULTS_PDA_ACCOUNT_OWNER,  
                            ctx.accounts.owner.key.as_ref(), 
                            &vault_account.identifier
                        ], ctx.program_id);
        
                let seeds = &[
                        VAULTS_PDA_ACCOUNT_OWNER, 
                        ctx.accounts.owner.key.as_ref(),
                        &vault_account.identifier,
                        &[bump]
                    ];
        
                let signer = &[&seeds[..]];
        
                let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.vault_program.to_account_info(), accounts, signer);
                let result = deposit(cpi_ctx, amount, 0);
        
                msg!("Deposit ended {}", amount);
        
                return result
            } else {
                return Err(error!(VaultsAccountsError::EarningsNotEnabled));
            }
        } else {
            return Err(error!(VaultsAccountsError::AccountNotFound));
        }

    }
    Ok(())
}
