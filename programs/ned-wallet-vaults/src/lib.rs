pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

use crate::instructions::*;
use crate::state::vaults::SpareType;

declare_id!("NEDrjAVUqMdZjpkJUPXeJe7SmfREW8pzLAFyhDUQhaJ");

#[program]
pub mod ned_wallet_vaults {

    use super::*;

    pub fn create_vault(
        ctx: Context<CreateVault>,
        name: Vec<u8>,
        identifier: [u8; 22],
        account_type: SpareType,
    ) -> Result<()> {
        create_vault::handler(ctx, name, identifier, account_type)
    }

    pub fn update_vault(
        ctx: Context<UpdateSavingsAccountVault>,
        identifier: [u8; 22],
        new_name: Vec<u8>,
        account_type: SpareType,
    ) -> Result<()> {
        update_vault::handler(ctx, identifier, new_name, account_type)
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
        // withdraw_from_vault::handler(ctx, identifier)
    }
}
