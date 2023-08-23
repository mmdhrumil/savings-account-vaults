use anchor_lang::prelude::*;
use crate::instructions::*;

pub mod state;
pub mod instructions;
pub mod constants;
pub mod errors;
pub mod utils;

declare_id!("5j3KuMK2u7KFtoEwiLTexUeooHq5NPQX96rYp5dhuze9");

#[program]
pub mod vaults {

    use super::*;

    // Vault creators can create the vaults for specific tokens
    pub fn initialize_vault(ctx: Context<InitializeVault>, interest_per_month_in_pct: u64) -> Result<()> {
        instructions::initialize_vault(ctx, interest_per_month_in_pct)
    }

    // Users can deposit the amounts of their choice to the vault
    pub fn deposit_funds(ctx: Context<DepositFunds>, amount: u64) -> Result<()> {
        instructions::deposit_funds(ctx, amount)
    }

    // Users can withdraw their deposited amount + the interest accrued
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>) -> Result<()> {
        instructions::withdraw_funds(ctx)
    }

    // Can only be called by the vault creator
    pub fn topup_interest(ctx: Context<TopupInterest>, amount: u64) -> Result<()> {
        instructions::topup_interest(ctx, amount)
    }
}