use anchor_lang::prelude::*;

#[error_code]
pub enum PaymentSplitterError {
    #[msg("Invalid share percentages (must sum to 100)")]
    InvalidSharePercentages,

    #[msg("Unauthorized operation")]
    Unauthorized,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Splitter already initialized")]
    AlreadyInitialized,

    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Math overflow")]
    MathOverflow,
}
