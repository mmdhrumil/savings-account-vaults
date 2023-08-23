use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Transfer};

use crate::state::Vault;

pub fn topup_interest(
    ctx: Context<TopupInterest>,
    amount: u64
) -> Result<()> {

    let vault = &mut ctx.accounts.vault;

    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_payer_ac.to_account_info(),
                to: ctx.accounts.token_vault_ac.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        amount,
    )?;

    vault.interest_reserves += amount;

    Ok(())
}

#[derive(Accounts)]
pub struct TopupInterest<'info> {
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
        constraint = (token_payer_ac.mint == vault.token)
    )]
    token_payer_ac: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = (token_vault_ac.mint == vault.token),
        address = vault.token_vault_ac
    )]
    token_vault_ac: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
