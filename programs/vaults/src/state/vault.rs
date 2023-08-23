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
    pub vault_key: Pubkey,
}

impl Vault {
    pub const LEN: usize = 8 + (1 * 1) + (32 * 3);
}