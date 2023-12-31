use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer};

use crate::state::{Vault, DepositReceipt};

pub fn deposit_funds(
    ctx: Context<DepositFunds>,
    amount: u64
) -> Result<()> {

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    let vault = &mut ctx.accounts.vault;

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_user_ac.to_account_info(),
                to: ctx.accounts.token_vault_ac.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    *ctx.accounts.deposit_receipt = DepositReceipt {
        bump: *ctx.bumps.get("deposit_receipt").unwrap(),
        vault: vault.key(),
        owner: ctx.accounts.owner.key(),
        user_share: ctx.accounts.deposit_receipt.user_share + amount,
        deposit_timestamp: current_timestamp
    };

    vault.total_deposits += amount;

    Ok(())
}

#[derive(Accounts)]
pub struct DepositFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = token
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init_if_needed,
        seeds = [
            owner.key().as_ref(),
            vault.key().as_ref()
        ],
        bump,
        payer = owner,
        space = DepositReceipt::LEN
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
