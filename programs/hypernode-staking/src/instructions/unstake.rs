use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;

/// Unstake HYPER tokens after lock period ends
///
/// Users can withdraw their staked HYPER once unlock_at timestamp is reached.
/// xNOS is burned when unstaking (voting power is lost).
pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let stake_account = &mut ctx.accounts.stake_account;
    let clock = Clock::get()?;

    // Validation: must be owner
    require!(
        stake_account.authority == ctx.accounts.authority.key(),
        StakingError::Unauthorized
    );

    // Validation: lock period must have ended
    require!(
        clock.unix_timestamp >= stake_account.unlock_at,
        StakingError::StakeLocked
    );

    // Validation: not already withdrawn
    require!(!stake_account.withdrawn, StakingError::AlreadyWithdrawn);

    // Transfer tokens from vault back to user
    let config_seeds = &[b"config", &[config.bump]];
    let signer = &[&config_seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.config.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
    token::transfer(cpi_ctx, stake_account.staked_amount)?;

    // Update global stats
    config.total_staked = config
        .total_staked
        .checked_sub(stake_account.staked_amount)
        .ok_or(StakingError::CalculationOverflow)?;
    config.total_xnos = config
        .total_xnos
        .checked_sub(stake_account.xnos)
        .ok_or(StakingError::CalculationOverflow)?;
    config.total_stakers = config.total_stakers.saturating_sub(1);

    // Mark as withdrawn (keep account for history)
    stake_account.withdrawn = true;

    msg!(
        "Unstaked {} HYPER. Burned {} xNOS.",
        stake_account.staked_amount,
        stake_account.xnos
    );

    // Emit event
    emit!(UnstakeEvent {
        user: ctx.accounts.authority.key(),
        amount: stake_account.staked_amount,
        xnos_burned: stake_account.xnos,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    /// Staking configuration (needed for PDA signing)
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, StakingConfig>,

    /// User's stake account
    #[account(
        mut,
        seeds = [b"stake", authority.key().as_ref()],
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    /// User withdrawing stake
    pub authority: Signer<'info>,

    /// User's token account (HYPER destination)
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
}

/// Event emitted when tokens are unstaked
#[event]
pub struct UnstakeEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub xnos_burned: u64,
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

    #[msg("Only stake owner can perform this action")]
    Unauthorized,
}
