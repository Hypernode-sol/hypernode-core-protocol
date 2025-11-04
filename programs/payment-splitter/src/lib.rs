use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("HyPErNoDePaymEntSpLiTtErProGramId1234567");

#[program]
pub mod payment_splitter {
    use super::*;

    /// Initialize payment splitter configuration
    pub fn initialize(
        ctx: Context<Initialize>,
        operator_share: u8,
        treasury_share: u8,
        incentive_share: u8,
        orchestrator_share: u8,
    ) -> Result<()> {
        instructions::initialize::handler(
            ctx,
            operator_share,
            treasury_share,
            incentive_share,
            orchestrator_share,
        )
    }

    /// Process payment distribution for a job
    pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64) -> Result<()> {
        instructions::process_payment::handler(ctx, amount)
    }

    /// Update splitter configuration (governance)
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        operator_share: u8,
        treasury_share: u8,
        incentive_share: u8,
        orchestrator_share: u8,
    ) -> Result<()> {
        instructions::update_config::handler(
            ctx,
            operator_share,
            treasury_share,
            incentive_share,
            orchestrator_share,
        )
    }

    /// Withdraw from treasury (governance only)
    pub fn withdraw_treasury(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
        instructions::withdraw_treasury::handler(ctx, amount)
    }
}
