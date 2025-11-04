use anchor_lang::prelude::*;

/// Global staking configuration
///
/// Manages staking parameters that can be updated by governance
#[account]
pub struct StakingConfig {
    /// Authority that can update config (multisig/governance)
    pub authority: Pubkey,

    /// Minimum stake amount (in lamports)
    pub min_stake_amount: u64,

    /// Maximum stake amount per account (anti-whale measure)
    pub max_stake_amount: u64,

    /// Minimum stake duration (in seconds, default 1 day)
    pub min_duration: i64,

    /// Maximum stake duration (in seconds, default 2 years)
    pub max_duration: i64,

    /// Tier thresholds (xNOS amounts)
    pub tier_thresholds: TierThresholds,

    /// Whether staking is currently enabled
    pub staking_enabled: bool,

    /// Total HYPER staked across all accounts
    pub total_staked: u64,

    /// Total xNOS issued
    pub total_xnos: u64,

    /// Number of active stake accounts
    pub total_stakers: u64,

    /// PDA bump
    pub bump: u8,
}

impl StakingConfig {
    pub const SPACE: usize = 8 + // discriminator
        32 + // authority
        8 + // min_stake_amount
        8 + // max_stake_amount
        8 + // min_duration
        8 + // max_duration
        40 + // tier_thresholds (5 * 8)
        1 + // staking_enabled
        8 + // total_staked
        8 + // total_xnos
        8 + // total_stakers
        1; // bump

    /// Default configuration (used during initialization)
    pub fn default_config() -> (u64, u64, i64, i64, TierThresholds) {
        const DAY: i64 = 24 * 60 * 60;
        const YEAR: i64 = 365 * DAY;

        (
            1_000_000_000, // min_stake_amount: 1 HYPER (9 decimals)
            1_000_000_000_000_000, // max_stake_amount: 1M HYPER
            DAY, // min_duration: 1 day
            2 * YEAR, // max_duration: 2 years
            TierThresholds {
                bronze: 1_000,
                silver: 10_000,
                gold: 50_000,
                diamond: 100_000,
            },
        )
    }
}

/// Tier threshold configuration
///
/// Defines the xNOS amounts required for each tier
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub struct TierThresholds {
    /// Bronze tier threshold
    pub bronze: u64,

    /// Silver tier threshold
    pub silver: u64,

    /// Gold tier threshold
    pub gold: u64,

    /// Diamond tier threshold
    pub diamond: u64,
}
