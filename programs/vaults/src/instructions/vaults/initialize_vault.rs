use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token};

use crate::state::*;

pub fn initialize_vault(
    ctx: Context<InitializeVault>,
    interest_per_month_in_pct: u64
) -> Result<()> {

    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;

    *ctx.accounts.vault = Vault {
        bump: *ctx.bumps.get("vault").unwrap(),
        owner: ctx.accounts.owner.key(),
        token: ctx.accounts.token.key(),
        token_vault_ac: ctx.accounts.token_vault_ac.key(),
        vault_key: ctx.accounts.vault_key.key(),
        total_deposits: 0u64,
        interest_reserves: 0u64,
        interest_per_month_in_pct,
        last_interest_timestamp: current_timestamp
    };

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: No constraint needed
    pub vault_key: UncheckedAccount<'info>,

    pub token: Account<'info, Mint>,

    #[account(
        init,
        seeds = [
            owner.key().as_ref(),
            token.key().as_ref(),
            vault_key.key().as_ref()
        ],
        bump,
        space = Vault::LEN,
        payer = owner
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init_if_needed,
        payer = owner,
        token::mint = token,
        token::authority = vault
    )]
    pub token_vault_ac: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>
}