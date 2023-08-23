use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer};

use crate::state::{Vault, DepositReceipt};
use crate::errors::VaultsError;
use crate::utils::calculate_interest;

pub fn withdraw_funds(
    ctx: Context<WithdrawFunds>,
) -> Result<()> {

    let vault_bump = ctx.accounts.vault.bump;

    let withdrawer = ctx.accounts.owner.key();
    let token = ctx.accounts.token.key();
    let vault_key = ctx.accounts.vault.vault_key;
    let vault = &mut ctx.accounts.vault;
    let base_amount = ctx.accounts.deposit_receipt.user_share;

    let vault_seeds = &[
        withdrawer.as_ref(),
        token.as_ref(),
        vault_key.as_ref(),
        &[vault_bump]
    ];
    let vault_signer_seeds = &[&vault_seeds[..]];

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let interest_amount = calculate_interest(
        ctx.accounts.deposit_receipt.deposit_timestamp as u64,
        current_timestamp as u64,
        vault.total_deposits,
        vault.interest_per_month_in_pct as u64
    );

    require!(interest_amount <= vault.interest_reserves, VaultsError::NotEnoughInterestReserves);

    let total_amount: u64 = interest_amount + base_amount;

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_vault_ac.to_account_info(),
                to: ctx.accounts.token_user_ac.to_account_info(),
                authority: vault.to_account_info(),
            },
            vault_signer_seeds
        ),
        total_amount,
    )?;

    ctx.accounts.deposit_receipt.user_share = 0;
    vault.total_deposits -= base_amount;
    vault.interest_reserves -= interest_amount;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = token
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [
            owner.key().as_ref(),
            vault.key().as_ref()
        ],
        bump = deposit_receipt.bump
    )]
    pub deposit_receipt: Account<'info, DepositReceipt>,

    pub token: Account<'info, Mint>,

    #[account(
        mut,
        constraint = (token_user_ac.mint == vault.token)
    )]
    token_user_ac: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = (token_vault_ac.mint == vault.token),
        address = vault.token_vault_ac
    )]
    token_vault_ac: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
