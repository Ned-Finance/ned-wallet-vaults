use crate::state::vaults::{VaultManager, VaultOwner, VAULTS_PDA_DATA, VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER};
use crate::state::ledger::{LedgerStore, LEDGER_PDA_DATA};
use affiliate::Partner;
use affiliate::program::Affiliate;
use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Mint};
use mercurial_vault::state::Vault;
use crate::utils::meteora::MercurialVault;
use crate::utils::vaults::deposit_liquidity;

#[derive(Accounts)]
#[instruction(identifier: [u8;22])]
pub struct DepositLiquidityWithDiffBalance<'info> {
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
        init_if_needed,
        seeds = [VAULTS_PDA_ACCOUNT_OWNER, owner.key.as_ref(), &identifier],
        bump,
        payer = owner,
        space = VaultOwner::LEN + 8
    )]
    pub vault_account_owner: Account<'info, VaultOwner>, // Program account to own token account

    #[account(
        init_if_needed,
        seeds = [VAULTS_PDA_ACCOUNT, owner.key.as_ref(), &identifier],
        bump,
        payer = owner,
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

    pub vault_program: Program<'info, MercurialVault>,
    
    pub affiliate_program: Program<'info, Affiliate>,

    #[account(mut)]
    pub vault: Box<Account<'info, Vault>>,
    /// CHECK:
    #[account(mut)]
    pub token_vault: UncheckedAccount<'info>,

    #[account(mut)]
    pub vault_lp_mint: Box<Account<'info, Mint>>,
    /// CHECK:
    #[account(mut)]
    pub user: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub partner: Box<Account<'info, Partner>>,
    /// CHECK:
    #[account(mut)]
    pub user_token: UncheckedAccount<'info>,

    #[account(mut, constraint = user_lp.owner == user.key())] //mint to account of user PDA
    pub user_lp: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [LEDGER_PDA_DATA, owner.key.as_ref()],
        bump,
    )]
    pub ledger_data: Account<'info, LedgerStore>, 
}

pub fn handler(
    ctx: Context<DepositLiquidityWithDiffBalance>,
    _identifier: [u8;22]
) -> Result<()> {

    let balance_transfered_to_vault = ctx.accounts.ledger_data.amount;

    msg!("Balance transfered to vault to be transfered to Meteora: {:?}", balance_transfered_to_vault);

    let data_account = &mut ctx.accounts.data_account.load()?;

    return deposit_liquidity(
        &ctx.accounts.owner,
        &ctx.accounts.partner,
        &ctx.accounts.vault_account,
        data_account,
        &ctx.accounts.vault_account_owner,
        &ctx.accounts.vault,
        &ctx.accounts.vault_lp_mint,
        &ctx.accounts.user_token,
        &ctx.accounts.user_lp,
        &ctx.accounts.user,
        &ctx.accounts.token_vault,
        &ctx.accounts.token_program,
        &ctx.accounts.system_program,
        &ctx.program_id,
        &ctx.accounts.vault_program,
        &ctx.accounts.affiliate_program,
        _identifier,
        balance_transfered_to_vault
    );
    // Ok(())
}
