use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;

/// Stake HYPER tokens to earn xNOS
///
/// Users lock HYPER for a specified duration and receive xNOS based on:
/// - Amount staked
/// - Duration of lock (longer = higher multiplier)
///
/// xNOS represents voting power and priority in the network
pub fn stake(
    ctx: Context<Stake>,
    amount: u64,
    duration_seconds: i64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let stake_account = &mut ctx.accounts.stake_account;
    let clock = Clock::get()?;

    // Validation: staking must be enabled
    require!(config.staking_enabled, StakingError::StakingDisabled);

    // Validation: amount
    require!(amount > 0, StakingError::InvalidAmount);
    require!(
        amount >= config.min_stake_amount,
        StakingError::BelowMinimum
    );
    require!(
        amount <= config.max_stake_amount,
        StakingError::AboveMaximum
    );

    // Validation: duration
    require!(
        duration_seconds >= config.min_duration,
        StakingError::DurationTooShort
    );
    require!(
        duration_seconds <= config.max_duration,
        StakingError::DurationTooLong
    );

    // Calculate xNOS and multiplier
    let (xnos, multiplier_bps) = StakeAccount::calculate_xnos(amount, duration_seconds)?;

    // Determine tier
    let tier = StakeAccount::calculate_tier(xnos);

    // Transfer tokens to vault (escrow)
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Initialize stake account
    stake_account.authority = ctx.accounts.authority.key();
    stake_account.staked_amount = amount;
    stake_account.stake_duration = duration_seconds;
    stake_account.staked_at = clock.unix_timestamp;
    stake_account.unlock_at = clock
        .unix_timestamp
        .checked_add(duration_seconds)
        .ok_or(StakingError::CalculationOverflow)?;
    stake_account.xnos = xnos;
    stake_account.multiplier_bps = multiplier_bps;
    stake_account.tier = tier;
    stake_account.withdrawn = false;
    stake_account.bump = ctx.bumps.stake_account;

    // Update global stats
    config.total_staked = config
        .total_staked
        .checked_add(amount)
        .ok_or(StakingError::CalculationOverflow)?;
    config.total_xnos = config
        .total_xnos
        .checked_add(xnos)
        .ok_or(StakingError::CalculationOverflow)?;
    config.total_stakers = config.total_stakers.checked_add(1).unwrap();

    msg!(
        "Staked {} HYPER for {} seconds. Earned {} xNOS ({}x multiplier). Tier: {:?}",
        amount,
        duration_seconds,
        xnos,
        multiplier_bps as f64 / 100.0,
        tier
    );

    // Emit event
    emit!(StakeEvent {
        user: ctx.accounts.authority.key(),
        amount,
        duration_seconds,
        xnos,
        multiplier_bps,
        tier,
        unlock_at: stake_account.unlock_at,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    /// Staking configuration
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakingConfig>,

    /// User's stake account (PDA)
    #[account(
        init,
        payer = authority,
        space = StakeAccount::SPACE,
        seeds = [b"stake", authority.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    /// User initiating stake
    #[account(mut)]
    pub authority: Signer<'info>,

    /// User's token account (HYPER)
    #[account(
        mut,
        constraint = user_token_account.owner == authority.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    /// Vault where staked tokens are held
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    /// SPL Token program
    pub token_program: Program<'info, Token>,

    /// System program
    pub system_program: Program<'info, System>,
}

/// Event emitted when tokens are staked
#[event]
pub struct StakeEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub duration_seconds: i64,
    pub xnos: u64,
    pub multiplier_bps: u16,
    pub tier: StakeTier,
    pub unlock_at: i64,
    pub timestamp: i64,
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

    #[msg("Amount is below minimum stake")]
    BelowMinimum,

    #[msg("Amount exceeds maximum stake per account")]
    AboveMaximum,

    #[msg("Staking is currently disabled")]
    StakingDisabled,
}
