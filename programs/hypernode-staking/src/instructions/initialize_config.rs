use anchor_lang::prelude::*;
use crate::state::*;

/// Initialize staking configuration
///
/// Creates the global StakingConfig account with default parameters.
/// Only needs to be called once during program deployment.
pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;

    let (min_stake, max_stake, min_duration, max_duration, tier_thresholds) =
        StakingConfig::default_config();

    config.authority = ctx.accounts.authority.key();
    config.min_stake_amount = min_stake;
    config.max_stake_amount = max_stake;
    config.min_duration = min_duration;
    config.max_duration = max_duration;
    config.tier_thresholds = tier_thresholds;
    config.staking_enabled = true;
    config.total_staked = 0;
    config.total_xnos = 0;
    config.total_stakers = 0;
    config.bump = ctx.bumps.config;

    msg!("Staking config initialized");
    msg!("Min stake: {} lamports", min_stake);
    msg!("Max stake: {} lamports", max_stake);
    msg!("Min duration: {} seconds", min_duration);
    msg!("Max duration: {} seconds", max_duration);

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    /// Staking configuration account (PDA)
    #[account(
        init,
        payer = authority,
        space = StakingConfig::SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, StakingConfig>,

    /// Authority initializing config (becomes config authority)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// System program
    pub system_program: Program<'info, System>,
}
