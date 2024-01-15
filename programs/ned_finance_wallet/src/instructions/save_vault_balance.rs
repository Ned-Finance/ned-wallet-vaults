use crate::state::ledger::{LedgerStore, LEDGER_PDA_DATA};
use crate::state::vaults::{VAULTS_PDA_ACCOUNT, VAULTS_PDA_ACCOUNT_OWNER, VaultOwner};

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
#[instruction(identifier: [u8;22])]
pub struct SaveVaultBalanceOnLedger<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

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
        mut,
        seeds = [VAULTS_PDA_ACCOUNT, owner.key.as_ref(), &identifier],
        bump,
        token::mint = mint, 
        token::authority = vault_account_owner,
    )]
    pub vault_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        seeds = [LEDGER_PDA_DATA],
        bump,
        payer = owner,
        space = LedgerStore::SIZE // extra data to store, not defined yet
    )]
    pub ledger_data: Account<'info, LedgerStore>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<SaveVaultBalanceOnLedger>, _identifier: [u8;22]) -> Result<()> {
    let ledger_data = &mut ctx.accounts.ledger_data;
    let user_token_account = ctx.accounts.vault_account.amount;
    ledger_data.amount = user_token_account;
    return Ok(());
}
