use anchor_lang::prelude::*;

declare_id!("3sJjr8BspgJc3K1KKxFDPU1QSH5PG3PoHs5xLRzR52fT");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
