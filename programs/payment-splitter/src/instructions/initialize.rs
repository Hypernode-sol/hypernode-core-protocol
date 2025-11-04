use anchor_lang::prelude::*;
use crate::state::SplitterConfig;
use crate::errors::PaymentSplitterError;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = SplitterConfig::LEN,
        seeds = [b"splitter_config"],
        bump
    )]
    pub splitter_config: Account<'info, SplitterConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Treasury wallet
    pub treasury: AccountInfo<'info>,

    /// CHECK: Incentive pool wallet
    pub incentive_pool: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    operator_share: u8,
    treasury_share: u8,
    incentive_share: u8,
    orchestrator_share: u8,
) -> Result<()> {
    let total_shares = operator_share as u16
        + treasury_share as u16
        + incentive_share as u16
        + orchestrator_share as u16;

    require!(
        total_shares == 100,
        PaymentSplitterError::InvalidSharePercentages
    );

    let config = &mut ctx.accounts.splitter_config;

    config.authority = ctx.accounts.authority.key();
    config.treasury = ctx.accounts.treasury.key();
    config.incentive_pool = ctx.accounts.incentive_pool.key();
    config.operator_share = operator_share;
    config.treasury_share = treasury_share;
    config.incentive_share = incentive_share;
    config.orchestrator_share = orchestrator_share;
    config.total_volume = 0;
    config.total_payments = 0;
    config.bump = ctx.bumps.splitter_config;

    emit!(SplitterInitializedEvent {
        authority: ctx.accounts.authority.key(),
        operator_share,
        treasury_share,
        incentive_share,
        orchestrator_share,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Payment splitter initialized");
    Ok(())
}

#[event]
pub struct SplitterInitializedEvent {
    pub authority: Pubkey,
    pub operator_share: u8,
    pub treasury_share: u8,
    pub incentive_share: u8,
    pub orchestrator_share: u8,
    pub timestamp: i64,
}
