use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("HYPRreward111111111111111111111111111111111");

/// Hypernode Rewards Program
///
/// Implements token reflection rewards with O(1) distribution.
///
/// Core Features:
/// - O(1) reward calculation (no iteration needed)
/// - Proportional distribution based on xNOS
/// - Automatic accumulation from job fees
/// - Instant claims for all stakers
///
/// Formula: user_reward = (xNOS / total_xNOS) * accumulated_rewards
///
/// Architecture Principles:
/// - Trustless: Math-based distribution (Szabo)
/// - Modular: Independent rewards program (Wood)
/// - Safe: Overflow protection (Amodei)
/// - Clear: Simple O(1) formula (Karpathy)
#[program]
pub mod hypernode_rewards {
    use super::*;

    /// Initialize reward pool
    ///
    /// Creates global reward pool with:
    /// - reward_rate_bps: Percentage of job price (e.g., 500 = 5%)
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        reward_rate_bps: u16,
    ) -> Result<()> {
        instructions::initialize_pool(ctx, reward_rate_bps)
    }

    /// Claim rewards based on xNOS
    ///
    /// Users claim their proportional share:
    /// - No iteration needed (O(1))
    /// - Based on current xNOS amount
    /// - Instant calculation and transfer
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards(ctx)
    }
}
