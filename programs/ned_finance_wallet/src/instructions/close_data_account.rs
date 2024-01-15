use crate::state::vaults::VaultManager;
use anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR;
use anchor_lang::prelude::*;
use std::io::{Cursor, Write};
use std::ops::DerefMut;

#[derive(Accounts)]
pub struct CloseDataAccount<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, close = destination)]
    pub data_account: AccountLoader<'info, VaultManager>, // Program account to store data

    /// CHECK:
    #[account(mut)]
    destination: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CloseDataAccount>) -> Result<()> {
    let data_account = &mut ctx.accounts.data_account.to_account_info();

    if data_account.owner.key() == ctx.accounts.owner.key() {
        let dest_starting_lamports = ctx.accounts.destination.lamports();

        **ctx.accounts.destination.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(data_account.lamports())
            .unwrap();
        **data_account.lamports.borrow_mut() = 0;

        let mut data = data_account.try_borrow_mut_data()?;
        for byte in data.deref_mut().iter_mut() {
            *byte = 0;
        }

        let dst: &mut [u8] = &mut data;
        let mut cursor = Cursor::new(dst);
        cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwrap();
    }
    Ok(())
}
