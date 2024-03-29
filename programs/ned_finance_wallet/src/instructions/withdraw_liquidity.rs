// use crate::errors::vaults::VaultsAccountsError;
use crate::state::vaults::{VaultManager, VaultOwner, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER};
use affiliate::{
    cpi::{
        accounts::DepositWithdrawLiquidity,
        *,
    },
    program::Affiliate,
    Partner,
};
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Mint};
use anchor_spl::associated_token::AssociatedToken;
use mercurial_vault::state::Vault;
use crate::utils::meteora::MercurialVault;


#[derive(Accounts)]
#[instruction(identifier: [u8;22])]
pub struct WithdrawLiquidity<'info> {
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

    pub vault_program: Program<'info, MercurialVault>,
    
    pub affiliate_program: Program<'info, Affiliate>,

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

    #[account(mut)]
    pub partner: Box<Account<'info, Partner>>,

    #[account(
        init_if_needed, 
        payer = owner,
        associated_token::mint = vault_lp_mint,
        associated_token::authority = user
    )] 
    pub user_lp: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(
    ctx: Context<WithdrawLiquidity>,
    _identifier: [u8;22],
    amount:u64,
) -> Result<()> {
    let data_account = &mut ctx.accounts.data_account.load()?;
    if data_account.owner.key() == ctx.accounts.owner.key() {

        let accounts = &data_account.accounts;

        let account_found = accounts.iter().find(
            |x| x.pub_key.key() == ctx.accounts.vault_account.key());

        if let Some(vault_account) = account_found {
            msg!("Withdraw started {}", amount);
            

            let accounts = DepositWithdrawLiquidity {
                
                vault: ctx.accounts.vault.to_account_info(),
                owner: ctx.accounts.vault_account_owner.to_account_info(),
                user_token: ctx.accounts.vault_account.to_account_info(),
                user_lp: ctx.accounts.user_lp.to_account_info(),
                user: ctx.accounts.user.to_account_info(),
                vault_lp_mint: ctx.accounts.vault_lp_mint.to_account_info(),
                token_vault: ctx.accounts.token_vault.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                vault_program: ctx.accounts.vault_program.to_account_info(),
                partner: ctx.accounts.partner.to_account_info(),
            };

            let (_account, bump) = Pubkey::find_program_address(
                &[
                    VAULTS_PDA_ACCOUNT_OWNER,
                    ctx.accounts.owner.key.as_ref(),
                    &vault_account.identifier,
                ],
                ctx.program_id,
            );

            let seeds = &[
                VAULTS_PDA_ACCOUNT_OWNER,
                ctx.accounts.owner.key.as_ref(),
                &vault_account.identifier,
                &[bump],
            ];

            // &[&[&[u8]]]

            let signer = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.vault_program.to_account_info(), accounts, signer);
            let result = withdraw(cpi_ctx, amount, 0);

            msg!("Withdraw ended {}", amount);

            return result
        }

    }
    Ok(())
}
