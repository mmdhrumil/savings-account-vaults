use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

use crate::{state::Vault, utils::calculate_interest, constants::INTEREST_PER_MONTH};

pub fn pay_interest(ctx: Context<PayInterest>) -> Result<()> {

    let vault = &mut ctx.accounts.vault;

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let amount = calculate_interest(
        vault.last_interest_timestamp as u64,
        current_timestamp as u64,
        vault.balance,
        INTEREST_PER_MONTH as u64
    );

    msg!("vault balance: {}", vault.balance);
    msg!("Earned: {} as interest over {} seconds", amount, (current_timestamp - vault.last_interest_timestamp));

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_payer_ac.to_account_info(),
                to: ctx.accounts.token_vault_ac.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        ),
        amount,
    )?;

    vault.balance += amount;
    vault.last_interest_timestamp = current_timestamp;

    Ok(())
}

#[derive(Accounts)]
pub struct PayInterest<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token: Account<'info, Mint>,

    #[account(
        mut,
        has_one = token
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        constraint = (token_payer_ac.mint == vault.token)
    )]
    pub token_payer_ac: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = (token_vault_ac.mint == vault.token),
        address = vault.token_vault_ac
    )]
    token_vault_ac: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>
}