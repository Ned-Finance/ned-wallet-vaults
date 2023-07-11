pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

use crate::instructions::*;
use crate::state::savings::AccountType;

declare_id!("5J8oWg8KbXvNqe64tpzCsbUdtVpBiHn6roFRrQ73FNMQ");

#[program]
pub mod ned_wallet_program {

    use super::*;

    pub fn create_savings_vault(
        ctx: Context<CreateSavingsVault>,
        name: Vec<u8>,
        identifier: [u8; 22],
        account_type: AccountType,
    ) -> Result<()> {
        create_savings_vault::handler(ctx, name, identifier, account_type)
    }

    pub fn update_savings_vault(
        ctx: Context<UpdateSavingsAccountVault>,
        identifier: [u8; 22],
        new_name: Vec<u8>,
        account_type: AccountType,
    ) -> Result<()> {
        update_savings_vault::handler(ctx, identifier, new_name, account_type)
    }

    pub fn delete_savings_vault(
        ctx: Context<DeleteSavingsAccountVault>,
        identifier: [u8; 22],
    ) -> Result<()> {
        delete_savings_vault::handler(ctx, identifier)
    }
}
