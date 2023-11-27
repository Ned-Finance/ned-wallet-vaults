use crate::state::vaults::{VaultManager, VaultOwner, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER};
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Mint};
use mercurial_vault::state::Vault;
use crate::utils::meteora::MercurialVault;
use crate::utils::vaults::deposit_liquidity;

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

    let data_account = &mut ctx.accounts.data_account.load()?;

    return deposit_liquidity(
        &ctx.accounts.owner,
        data_account,
        &ctx.accounts.vault_account,
        &ctx.accounts.vault_program,
        &ctx.accounts.vault,
        &ctx.accounts.vault_lp_mint,
        &ctx.accounts.user_token,
        &ctx.accounts.user_lp,
        &ctx.accounts.user,
        &ctx.accounts.token_vault,
        &ctx.accounts.token_program,
        &ctx.program_id,
        _identifier,
        amount
    );
    // Ok(())
}
