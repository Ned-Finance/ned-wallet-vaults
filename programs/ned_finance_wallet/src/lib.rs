pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

use crate::instructions::*;
use crate::state::vaults::SpareType;

declare_id!("NEDXqFFWdkRYUE9oRRAteiS22tXDvBiSZgNcGn9G5QA");

#[program]
pub mod ned_finance_wallet {

    use super::*;

    pub fn create_vault(
        ctx: Context<CreateVault>,
        name: Vec<u8>,
        identifier: [u8; 22],
        account_type: SpareType,
        earnings_enabled: u8,
    ) -> Result<()> {
        create_vault::handler(ctx, name, identifier, account_type, earnings_enabled)
    }

    pub fn update_vault(
        ctx: Context<UpdateSavingsAccountVault>,
        identifier: [u8; 22],
        new_name: Vec<u8>,
        account_type: SpareType,
        earnings_enabled: u8,
    ) -> Result<()> {
        update_vault::handler(ctx, identifier, new_name, account_type, earnings_enabled)
    }

    pub fn delete_vault(
        ctx: Context<DeleteSavingsAccountVault>,
        identifier: [u8; 22],
    ) -> Result<()> {
        delete_vault::handler(ctx, identifier)
    }

    pub fn withdraw_from_vault(
        ctx: Context<WithdrawFromVault>,
        identifier: [u8; 22],
        amount: u64,
    ) -> Result<()> {
        withdraw_from_vault::handler(ctx, identifier, amount)
    }

    pub fn deposit_liquidity(
        ctx: Context<DepositLiquidity>,
        identifier: [u8; 22],
        amount: u64,
    ) -> Result<()> {
        deposit_liquidity::handler(ctx, identifier, amount)
    }

    pub fn withdraw_liquidity(
        ctx: Context<WithdrawLiquidity>,
        identifier: [u8; 22],
        amount: u64,
    ) -> Result<()> {
        withdraw_liquidity::handler(ctx, identifier, amount)
    }

    pub fn withdraw_from_diff_balance(
        ctx: Context<WithdrawFromDiffBalance>,
        identifier: [u8; 22],
    ) -> Result<()> {
        withdraw_from_diff_balance::handler(ctx, identifier)
    }

    pub fn save_account_balance(ctx: Context<SaveAccountBalanceOnLedger>) -> Result<()> {
        save_account_balance::handler(ctx)
    }

    pub fn save_vault_balance(
        ctx: Context<SaveVaultBalanceOnLedger>,
        identifier: [u8; 22],
    ) -> Result<()> {
        save_vault_balance::handler(ctx, identifier)
    }

    pub fn deposit_to_vault_with_diff_balance(
        ctx: Context<DepositToVaultWithDiffBalance>,
        identifier: [u8; 22],
    ) -> Result<()> {
        deposit_to_vault_with_diff_balance::handler(ctx, identifier)
    }

    pub fn deposit_liquidity_with_diff_balance(
        ctx: Context<DepositLiquidityWithDiffBalance>,
        identifier: [u8; 22],
    ) -> Result<()> {
        deposit_liquidity_with_diff_balance::handler(ctx, identifier)
    }

    pub fn close_data_account(ctx: Context<CloseDataAccount>) -> Result<()> {
        close_data_account::handler(ctx)
    }
}
