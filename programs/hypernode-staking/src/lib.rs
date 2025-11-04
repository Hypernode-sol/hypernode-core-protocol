use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("HYPRstake1111111111111111111111111111111111");

/// Hypernode Staking Program
///
/// Manages HYPER token staking with xNOS calculation and time multipliers.
///
/// Core Features:
/// - Time-based multipliers (1x to 4x based on lock duration)
/// - Tier system (Starter → Diamond based on xNOS)
/// - Trustless escrow with time-locks
/// - Global stats tracking
///
/// Architecture Principles:
/// - Trustless: Time-locks enforced on-chain (Szabo)
/// - Modular: Independent from jobs/nodes programs (Wood)
/// - Safe: Extensive validations, no overflow (Amodei)
/// - Clear: Simple xNOS formula, well documented (Karpathy)
#[program]
pub mod hypernode_staking {
    use super::*;

    /// Initialize staking configuration
    ///
    /// Called once during deployment to set up:
    /// - Min/max stake amounts
    /// - Min/max durations
    /// - Tier thresholds
    /// - Authority
    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        instructions::initialize_config(ctx)
    }

    /// Stake HYPER tokens to earn xNOS
    ///
    /// Users lock HYPER for specified duration:
    /// - amount: HYPER tokens to stake
    /// - duration_seconds: Lock period (longer = higher multiplier)
    ///
    /// Returns xNOS based on formula:
    /// xNOS = amount * multiplier
    ///
    /// Multipliers:
    /// - 1 month: 1x
    /// - 3 months: 1.5x
    /// - 6 months: 2x
    /// - 1 year: 4x
    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
        duration_seconds: i64,
    ) -> Result<()> {
        instructions::stake(ctx, amount, duration_seconds)
    }

    /// Unstake HYPER tokens after lock period
    ///
    /// Withdraws staked HYPER once unlock_at is reached.
    /// Burns xNOS (voting power is lost).
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::unstake(ctx)
    }
}
