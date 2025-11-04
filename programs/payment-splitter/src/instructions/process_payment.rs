use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::SplitterConfig;
use crate::errors::PaymentSplitterError;

#[derive(Accounts)]
pub struct ProcessPayment<'info> {
    #[account(mut, seeds = [b"splitter_config"], bump = splitter_config.bump)]
    pub splitter_config: Account<'info, SplitterConfig>,

    /// Source token account (client paying)
    #[account(mut)]
    pub source_token_account: Account<'info, TokenAccount>,

    /// Node operator token account
    #[account(mut)]
    pub operator_token_account: Account<'info, TokenAccount>,

    /// Treasury token account
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// Incentive pool token account
    #[account(mut)]
    pub incentive_pool_account: Account<'info, TokenAccount>,

    /// Orchestrator token account (optional)
    #[account(mut)]
    pub orchestrator_token_account: Account<'info, TokenAccount>,

    /// Authority to transfer from source
    pub source_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
    require!(amount > 0, PaymentSplitterError::InvalidAmount);

    let config = &mut ctx.accounts.splitter_config;

    // Calculate shares
    let operator_amount = config.calculate_operator_amount(amount);
    let treasury_amount = config.calculate_treasury_amount(amount);
    let incentive_amount = config.calculate_incentive_amount(amount);
    let orchestrator_amount = config.calculate_orchestrator_amount(amount);

    // Transfer to operator
    let transfer_to_operator = Transfer {
        from: ctx.accounts.source_token_account.to_account_info(),
        to: ctx.accounts.operator_token_account.to_account_info(),
        authority: ctx.accounts.source_authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_operator),
        operator_amount,
    )?;

    // Transfer to treasury
    let transfer_to_treasury = Transfer {
        from: ctx.accounts.source_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.source_authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_treasury),
        treasury_amount,
    )?;

    // Transfer to incentive pool
    let transfer_to_incentive = Transfer {
        from: ctx.accounts.source_token_account.to_account_info(),
        to: ctx.accounts.incentive_pool_account.to_account_info(),
        authority: ctx.accounts.source_authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_incentive),
        incentive_amount,
    )?;

    // Transfer to orchestrator
    let transfer_to_orchestrator = Transfer {
        from: ctx.accounts.source_token_account.to_account_info(),
        to: ctx.accounts.orchestrator_token_account.to_account_info(),
        authority: ctx.accounts.source_authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_orchestrator),
        orchestrator_amount,
    )?;

    // Record payment
    config.record_payment(amount);

    emit!(PaymentProcessedEvent {
        total_amount: amount,
        operator_amount,
        treasury_amount,
        incentive_amount,
        orchestrator_amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Payment of {} processed and distributed", amount);
    Ok(())
}

#[event]
pub struct PaymentProcessedEvent {
    pub total_amount: u64,
    pub operator_amount: u64,
    pub treasury_amount: u64,
    pub incentive_amount: u64,
    pub orchestrator_amount: u64,
    pub timestamp: i64,
}
