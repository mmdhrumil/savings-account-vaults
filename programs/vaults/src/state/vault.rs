use anchor_lang::prelude::*;


// Vault is a PDA of [owner, token, vault_key]
// This lets us allow users to create as many as vaults
// of every token as needed.
#[account]
#[derive(Debug, Default)]
pub struct Vault {
    pub bump: u8,

    pub owner: Pubkey,
    pub token: Pubkey,
    pub token_vault_ac: Pubkey,
    pub vault_key: Pubkey,

    pub total_deposits: u64,    
    pub interest_reserves: u64,

    pub interest_per_month_in_pct: u64,

    pub last_interest_timestamp: i64
}

impl Vault {
    pub const LEN: usize = 8 + (1 * 1) + (32 * 4) + (8 * 4);
}

#[account]
#[derive(Debug, Default)]
pub struct DepositReceipt {
    pub bump: u8,
    
    pub owner: Pubkey,
    pub vault: Pubkey,

    pub user_share: u64,
    pub deposit_timestamp: i64
}

impl DepositReceipt {
    pub const LEN: usize = 8 + (1 * 1) + (32 * 2) + (8 * 2);
}