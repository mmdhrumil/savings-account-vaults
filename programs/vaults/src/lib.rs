use anchor_lang::prelude::*;
use crate::instructions::*;

pub mod state;
pub mod instructions;
pub mod constants;
pub mod errors;

declare_id!("5j3KuMK2u7KFtoEwiLTexUeooHq5NPQX96rYp5dhuze9");

#[program]
pub mod vaults {

    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault(ctx)
    }

    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        instructions::deposit_funds(ctx, amount)
    }
    
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, amount: u64) -> Result<()> {
        instructions::withdraw_funds(ctx, amount)
    }
}