use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;

/// Claim rewards based on xNOS (O(1) distribution)
pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let pool = &mut ctx.accounts.reward_pool;
    let stake_account = &ctx.accounts.stake_account;
    let staking_config = &ctx.accounts.staking_config;

    require!(pool.enabled, RewardError::RewardsDisabled);
    require!(stake_account.xnos > 0, RewardError::NoStake);
    require!(pool.total_rewards > 0, RewardError::NoRewards);

    // Calculate claimable rewards (O(1))
    let claimable = pool.calculate_claimable(
        stake_account.xnos,
        staking_config.total_xnos,
    )?;

    require!(claimable > 0, RewardError::NoRewards);

    // Transfer rewards
    let seeds = &[b"reward_pool", &[pool.bump]];
    let signer = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.reward_vault.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: ctx.accounts.reward_pool.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::transfer(cpi_ctx, claimable)?;

    // Update stats
    pool.total_rewards = pool.total_rewards.saturating_sub(claimable);
    pool.total_claimed = pool.total_claimed.saturating_add(claimable);
    pool.total_stakers_rewarded = pool.total_stakers_rewarded.saturating_add(1);

    msg!("Claimed {} rewards for {} xNOS", claimable, stake_account.xnos);

    emit!(ClaimEvent {
        user: ctx.accounts.authority.key(),
        amount: claimable,
        xnos: stake_account.xnos,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [b"reward_pool"],
        bump = reward_pool.bump
    )]
    pub reward_pool: Account<'info, RewardPool>,

    /// Reference to staking config (for total_xnos)
    #[account(
        seeds = [b"config"],
        bump = staking_config.bump,
        seeds::program = staking_program.key()
    )]
    pub staking_config: Account<'info, StakingConfig>,

    /// User's stake account (for xNOS)
    #[account(
        seeds = [b"stake", authority.key().as_ref()],
        bump = stake_account.bump,
        seeds::program = staking_program.key()
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub reward_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_account.owner == authority.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub authority: Signer<'info>,

    /// CHECK: Staking program address
    pub staking_program: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

/// Placeholder types (would import from staking program)
#[account]
pub struct StakingConfig {
    pub authority: Pubkey,
    pub total_xnos: u64,
    pub bump: u8,
}

#[account]
pub struct StakeAccount {
    pub authority: Pubkey,
    pub xnos: u64,
    pub bump: u8,
}

#[event]
pub struct ClaimEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub xnos: u64,
    pub timestamp: i64,
}
