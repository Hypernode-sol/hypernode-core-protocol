use anchor_lang::prelude::*;
use crate::state::*;

/// Initialize reward pool
pub fn initialize_pool(
    ctx: Context<InitializePool>,
    reward_rate_bps: u16,
) -> Result<()> {
    let pool = &mut ctx.accounts.reward_pool;

    require!(reward_rate_bps <= 10000, RewardError::InvalidRate);

    pool.authority = ctx.accounts.authority.key();
    pool.total_rewards = 0;
    pool.total_claimed = 0;
    pool.reward_rate_bps = reward_rate_bps;
    pool.enabled = true;
    pool.total_distributions = 0;
    pool.total_stakers_rewarded = 0;
    pool.bump = ctx.bumps.reward_pool;

    msg!("Reward pool initialized with {}% rate", reward_rate_bps as f64 / 100.0);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = RewardPool::SPACE,
        seeds = [b"reward_pool"],
        bump
    )]
    pub reward_pool: Account<'info, RewardPool>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum RewardError {
    #[msg("Reward rate must be <= 10000 bps (100%)")]
    InvalidRate,
}
