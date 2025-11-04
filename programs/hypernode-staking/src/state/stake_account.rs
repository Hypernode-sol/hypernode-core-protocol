use anchor_lang::prelude::*;

/// User's staking account with xNOS calculation
///
/// Implements time-based multipliers for xNOS calculation:
/// - 1 month = 1x multiplier
/// - 3 months = 1.5x multiplier
/// - 6 months = 2x multiplier
/// - 1 year = 4x multiplier
#[account]
pub struct StakeAccount {
    /// Owner of this stake account
    pub authority: Pubkey,

    /// Amount of HYPER staked
    pub staked_amount: u64,

    /// Duration of stake in seconds
    pub stake_duration: i64,

    /// When stake started (timestamp)
    pub staked_at: i64,

    /// When stake unlocks (timestamp)
    pub unlock_at: i64,

    /// Calculated xNOS based on amount and duration
    pub xnos: u64,

    /// Multiplier applied (stored as basis points: 100 = 1x, 400 = 4x)
    pub multiplier_bps: u16,

    /// Tier level (Starter, Bronze, Silver, Gold, Diamond)
    pub tier: StakeTier,

    /// Whether stake has been withdrawn
    pub withdrawn: bool,

    /// PDA bump
    pub bump: u8,
}

impl StakeAccount {
    pub const SPACE: usize = 8 + // discriminator
        32 + // authority
        8 + // staked_amount
        8 + // stake_duration
        8 + // staked_at
        8 + // unlock_at
        8 + // xnos
        2 + // multiplier_bps
        1 + // tier
        1 + // withdrawn
        1; // bump

    /// Calculate xNOS based on staked amount and duration
    ///
    /// Formula: xNOS = staked_amount * multiplier
    ///
    /// Duration multipliers (Nosana-inspired):
    /// - < 1 month: 1x (100 bps)
    /// - 1-3 months: 1.5x (150 bps)
    /// - 3-6 months: 2x (200 bps)
    /// - 6-12 months: 3x (300 bps)
    /// - >= 1 year: 4x (400 bps)
    pub fn calculate_xnos(amount: u64, duration_seconds: i64) -> Result<(u64, u16)> {
        const MONTH: i64 = 30 * 24 * 60 * 60; // 30 days in seconds
        const YEAR: i64 = 365 * 24 * 60 * 60; // 365 days in seconds

        let multiplier_bps = if duration_seconds >= YEAR {
            400 // 4x
        } else if duration_seconds >= 6 * MONTH {
            300 // 3x
        } else if duration_seconds >= 3 * MONTH {
            200 // 2x
        } else if duration_seconds >= MONTH {
            150 // 1.5x
        } else {
            100 // 1x
        };

        // Calculate xNOS: amount * multiplier / 100
        let xnos = (amount as u128)
            .checked_mul(multiplier_bps as u128)
            .and_then(|v| v.checked_div(100))
            .and_then(|v| u64::try_from(v).ok())
            .ok_or(StakingError::CalculationOverflow)?;

        Ok((xnos, multiplier_bps))
    }

    /// Determine tier based on xNOS amount
    ///
    /// Tiers (adjustable in config):
    /// - Starter: 0-999 xNOS
    /// - Bronze: 1,000-9,999 xNOS
    /// - Silver: 10,000-49,999 xNOS
    /// - Gold: 50,000-99,999 xNOS
    /// - Diamond: 100,000+ xNOS
    pub fn calculate_tier(xnos: u64) -> StakeTier {
        if xnos >= 100_000 {
            StakeTier::Diamond
        } else if xnos >= 50_000 {
            StakeTier::Gold
        } else if xnos >= 10_000 {
            StakeTier::Silver
        } else if xnos >= 1_000 {
            StakeTier::Bronze
        } else {
            StakeTier::Starter
        }
    }
}

/// Staking tiers based on xNOS amount
///
/// Each tier can unlock different benefits:
/// - Priority in job queue
/// - Reduced fees
/// - Access to premium markets
/// - Governance voting power
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum StakeTier {
    Starter,  // 0-999 xNOS
    Bronze,   // 1,000-9,999 xNOS
    Silver,   // 10,000-49,999 xNOS
    Gold,     // 50,000-99,999 xNOS
    Diamond,  // 100,000+ xNOS
}

#[error_code]
pub enum StakingError {
    #[msg("Calculation resulted in overflow")]
    CalculationOverflow,

    #[msg("Stake is still locked (unlock_at not reached)")]
    StakeLocked,

    #[msg("Stake has already been withdrawn")]
    AlreadyWithdrawn,

    #[msg("Duration must be at least 1 day")]
    DurationTooShort,

    #[msg("Duration exceeds maximum (2 years)")]
    DurationTooLong,

    #[msg("Amount must be greater than zero")]
    InvalidAmount,
}
