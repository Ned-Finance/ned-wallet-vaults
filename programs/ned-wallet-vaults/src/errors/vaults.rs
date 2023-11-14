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
    #[msg("Investments not allowed until enabling earnings")]
    EarningsNotEnabled,
    #[msg("Error when depositing on protocol")]
    DepositSavingsError,
    #[msg("Error when withdrawal from protocol")]
    WithdrawalSavingsError,
    #[msg("Vault not found")]
    VaultNotFound,
    #[msg("Amount in account is over slippage")]
    AccountBalanceOverSlippage,
}
