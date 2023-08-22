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

#[account]
pub struct Tweet {
    pub author: Pubkey,  // 32 bytes
    pub timestamp: i64,  // 8 bytes
    pub topic: String,   // limiting to a size of 50 characters
    pub content: String, // limiting to a size of 280 characters
}

// Calculations
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX_TOPIC: usize = 4; // used to store the total length of the topic string
const MAX_TOPIC_LENGTH: usize = 50 * 4; // above title
const STRING_LENGTH_PREFIX_CONTENT: usize = 4; // used to store the total length of the topic string
const MAX_CONTENT_LENGTH: usize = 280 * 4; // above content

impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX_TOPIC + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX_CONTENT + MAX_CONTENT_LENGTH; // Content.
}
