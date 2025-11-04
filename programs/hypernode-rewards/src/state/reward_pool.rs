use anchor_lang::prelude::*;

/// Reward pool with O(1) token reflection distribution
///
/// Implements Nosana-style reward distribution where:
/// - Rewards are accumulated in a pool
/// - Distribution happens instantly for all stakers (O(1))
/// - Each staker claims their share based on their xNOS
///
/// Formula: user_reward = (xNOS / total_xNOS) * accumulated_rewards
#[account]
pub struct RewardPool {
    /// Authority that can update pool parameters
    pub authority: Pubkey,

    /// Total rewards accumulated (not yet claimed)
    pub total_rewards: u64,

    /// Total rewards claimed by users
    pub total_claimed: u64,

    /// Reward rate per job (basis points of job price)
    /// Example: 500 = 5% of job price goes to stakers
    pub reward_rate_bps: u16,

    /// Whether rewards are currently enabled
    pub enabled: bool,

    /// Stats
    pub total_distributions: u64,
    pub total_stakers_rewarded: u64,

    /// PDA bump
    pub bump: u8,
}

impl RewardPool {
    pub const SPACE: usize = 8 + // discriminator
        32 + // authority
        8 + // total_rewards
        8 + // total_claimed
        2 + // reward_rate_bps
        1 + // enabled
        8 + // total_distributions
        8 + // total_stakers_rewarded
        1; // bump

    /// Calculate user's claimable rewards based on xNOS
    ///
    /// Formula: (user_xnos * total_rewards) / total_xnos
    ///
    /// This is O(1) - no iteration needed!
    pub fn calculate_claimable(
        &self,
        user_xnos: u64,
        total_xnos: u64,
    ) -> Result<u64> {
        if total_xnos == 0 {
            return Ok(0);
        }

        let claimable = (user_xnos as u128)
            .checked_mul(self.total_rewards as u128)
            .and_then(|v| v.checked_div(total_xnos as u128))
            .and_then(|v| u64::try_from(v).ok())
            .ok_or(RewardError::CalculationOverflow)?;

        Ok(claimable)
    }
}

#[error_code]
pub enum RewardError {
    #[msg("Calculation resulted in overflow")]
    CalculationOverflow,

    #[msg("No rewards available to claim")]
    NoRewards,

    #[msg("Rewards are currently disabled")]
    RewardsDisabled,

    #[msg("User has no staked xNOS")]
    NoStake,
}
