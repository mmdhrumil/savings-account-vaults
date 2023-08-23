use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer};

use crate::state::Vault;
use crate::errors::VaultsError;

pub fn withdraw_funds(
    ctx: Context<WithdrawFunds>,
    amount: u64
) -> Result<()> {

    let vault_bump = ctx.accounts.vault.bump;

    let owner = ctx.accounts.owner.key();
    let token = ctx.accounts.token.key();
    let vault_key = ctx.accounts.vault.vault_key;

    let vault_seeds = &[
        owner.as_ref(),
        token.as_ref(),
        vault_key.as_ref(),
        &[vault_bump]
    ];
    let vault_signer_seeds = &[&vault_seeds[..]];

    let vault = &mut ctx.accounts.vault;

    require!(amount <= vault.balance, VaultsError::WithdrawExceedsBalance);

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
        amount,
    )?;

    vault.balance -= amount;

    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        has_one = owner,
        has_one = token
    )]
    pub vault: Account<'info, Vault>,

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
