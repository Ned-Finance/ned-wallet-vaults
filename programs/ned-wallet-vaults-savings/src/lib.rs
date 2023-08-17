pub mod instructions;
pub mod utils;

use anchor_lang::prelude::*;

use crate::instructions::*;

declare_id!("NEDNE4zjiduJKv6tAK7Lm7L1tgwYhJPArTk96hpAJKA");

#[program]
pub mod ned_wallet_vaults_savings {

    use super::*;

    pub fn provide_liquidity(ctx: Context<ProvideLiquidity>) -> Result<()> {
        Ok(())
    }
}
