use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::JobAccount;
use crate::errors::JobReceiptError;

#[derive(Accounts)]
pub struct SettleJob<'info> {
    #[account(mut)]
    pub job_account: Account<'info, JobAccount>,

    /// Client's HYPER token account
    #[account(mut)]
    pub client_token_account: Account<'info, TokenAccount>,

    /// Node operator's HYPER token account
    #[account(mut)]
    pub operator_token_account: Account<'info, TokenAccount>,

    /// Protocol treasury HYPER account
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// Incentive pool HYPER account
    #[account(mut)]
    pub incentive_pool_account: Account<'info, TokenAccount>,

    /// Authority that can settle payments
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<SettleJob>) -> Result<()> {
    let job = &mut ctx.accounts.job_account;

    require!(job.can_be_settled(), JobReceiptError::JobNotCompleted);
    require!(!job.payment_settled, JobReceiptError::JobAlreadySettled);

    let total_price = job.price;

    // Distribution: 80% operator, 10% treasury, 5% incentive, 5% orchestrator
    let operator_share = (total_price * 80) / 100;
    let treasury_share = (total_price * 10) / 100;
    let incentive_share = (total_price * 5) / 100;
    // orchestrator_share would be the remaining 5%

    // Transfer to operator (80%)
    let transfer_to_operator = Transfer {
        from: ctx.accounts.client_token_account.to_account_info(),
        to: ctx.accounts.operator_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_operator),
        operator_share,
    )?;

    // Transfer to treasury (10%)
    let transfer_to_treasury = Transfer {
        from: ctx.accounts.client_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_treasury),
        treasury_share,
    )?;

    // Transfer to incentive pool (5%)
    let transfer_to_incentive = Transfer {
        from: ctx.accounts.client_token_account.to_account_info(),
        to: ctx.accounts.incentive_pool_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_to_incentive),
        incentive_share,
    )?;

    job.payment_settled = true;

    emit!(PaymentDistributedEvent {
        job_id: job.job_id.clone(),
        operator_amount: operator_share,
        treasury_amount: treasury_share,
        incentive_amount: incentive_share,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Payment settled for job: {}", job.job_id);
    Ok(())
}

#[event]
pub struct PaymentDistributedEvent {
    pub job_id: String,
    pub operator_amount: u64,
    pub treasury_amount: u64,
    pub incentive_amount: u64,
    pub timestamp: i64,
}
