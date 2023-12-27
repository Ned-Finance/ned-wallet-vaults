use crate::state::ledger::{LedgerStore, LEDGER_PDA_DATA};

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

#[derive(Accounts)]
pub struct SaveAccountBalanceOnLedger<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub mint: Account<'info, Mint>, 

    #[account(
        mut,
        token::mint = mint, 
        token::authority = owner,
    )]
    pub user_token_account: Account<'info, TokenAccount>,

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

pub fn handler(ctx: Context<SaveAccountBalanceOnLedger>) -> Result<()> {
    let ledger_data = &mut ctx.accounts.ledger_data;
    let user_token_account = ctx.accounts.user_token_account.amount;
    ledger_data.amount = user_token_account;
    return Ok(());
}
