use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("3sJjr8BspgJc3K1KKxFDPU1QSH5PG3PoHs5xLRzR52fT");

#[program]
pub mod solana_twitter {
    use super::*;

    // pub author: Pubkey,  // 32 bytes
    // pub timestamp: i64,  // 8 bytes
    // pub topic: String,   // limiting to a size of 50 characters
    // pub content: String,

    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> Result<()> {
        let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into());
        }

        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into());
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;

        Ok(())
    }
}

#[derive(Accounts)]
<<<<<<< Updated upstream
pub struct Initialize {}
=======
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,

    #[account(mut)]
    pub author: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

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

#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}
>>>>>>> Stashed changes
