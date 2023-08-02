use anchor_lang::prelude::*;

#[error_code]
pub enum VaultsAccountsError {
    #[msg("This account was already initialized")]
    AlreadyInitialized,
    #[msg("Max accounts reached")]
    MaxAccountsReached,
    #[msg("Account name can't be empty")]
    AccountNameEmpty,
    #[msg("Account not found")]
    AccountNotFound,
    #[msg("Action not allowed")]
    ActionNotAllowed,
}
