use anchor_lang::prelude::*;
use crate::instructions::*;

pub mod state;
pub mod instructions;

declare_id!("5j3KuMK2u7KFtoEwiLTexUeooHq5NPQX96rYp5dhuze9");

#[program]
pub mod vaults {

    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        instructions::initialize_vault(ctx)
    }    
}