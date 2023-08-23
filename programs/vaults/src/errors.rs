use anchor_lang::error_code;

#[error_code]
pub enum VaultsError {

    #[msg("Withdraw amount cannot exceed balance")]
    WithdrawExceedsBalance,
    
}