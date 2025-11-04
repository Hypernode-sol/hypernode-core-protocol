use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::SplitterConfig;
use crate::errors::PaymentSplitterError;

#[derive(Accounts)]
pub struct WithdrawTreasury<'info> {
    #[account(
        seeds = [b"splitter_config"],
        bump = splitter_config.bump,
        has_one = authority @ PaymentSplitterError::Unauthorized
    )]
    pub splitter_config: Account<'info, SplitterConfig>,

    /// Treasury token account (source)
    #[account(mut)]
    pub treasury_token_account: Account<'info, TokenAccount>,

    /// Destination token account
    #[account(mut)]
    pub destination_token_account: Account<'info, TokenAccount>,

    /// Authority (governance)
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
    require!(amount > 0, PaymentSplitterError::InvalidAmount);
    require!(
        ctx.accounts.treasury_token_account.amount >= amount,
        PaymentSplitterError::InsufficientBalance
    );

    // Transfer from treasury to destination
    let transfer = Transfer {
        from: ctx.accounts.treasury_token_account.to_account_info(),
        to: ctx.accounts.destination_token_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer),
        amount,
    )?;

    emit!(TreasuryWithdrawalEvent {
        amount,
        destination: ctx.accounts.destination_token_account.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Withdrawn {} from treasury", amount);
    Ok(())
}

#[event]
pub struct TreasuryWithdrawalEvent {
    pub amount: u64,
    pub destination: Pubkey,
    pub timestamp: i64,
}
