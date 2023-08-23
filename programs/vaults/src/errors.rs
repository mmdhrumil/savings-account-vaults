use anchor_lang::error_code;

#[error_code]
pub enum VaultsError {

    #[msg("Not enough interest reserves")]
    NotEnoughInterestReserves,
    
}