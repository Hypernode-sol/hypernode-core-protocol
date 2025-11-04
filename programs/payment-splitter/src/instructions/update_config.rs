use anchor_lang::prelude::*;
use crate::state::SplitterConfig;
use crate::errors::PaymentSplitterError;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"splitter_config"],
        bump = splitter_config.bump,
        has_one = authority @ PaymentSplitterError::Unauthorized
    )]
    pub splitter_config: Account<'info, SplitterConfig>,

    pub authority: Signer<'info>,
}

pub fn handler(
    ctx: Context<UpdateConfig>,
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

    config.operator_share = operator_share;
    config.treasury_share = treasury_share;
    config.incentive_share = incentive_share;
    config.orchestrator_share = orchestrator_share;

    emit!(ConfigUpdatedEvent {
        operator_share,
        treasury_share,
        incentive_share,
        orchestrator_share,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Splitter config updated");
    Ok(())
}

#[event]
pub struct ConfigUpdatedEvent {
    pub operator_share: u8,
    pub treasury_share: u8,
    pub incentive_share: u8,
    pub orchestrator_share: u8,
    pub timestamp: i64,
}
