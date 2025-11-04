use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct SplitterConfig {
    /// Program authority
    pub authority: Pubkey,

    /// Treasury wallet
    pub treasury: Pubkey,

    /// Incentive pool wallet
    pub incentive_pool: Pubkey,

    /// Operator share percentage (e.g., 80 = 80%)
    pub operator_share: u8,

    /// Treasury share percentage (e.g., 10 = 10%)
    pub treasury_share: u8,

    /// Incentive pool share percentage (e.g., 5 = 5%)
    pub incentive_share: u8,

    /// Orchestrator/Agent share percentage (e.g., 5 = 5%)
    pub orchestrator_share: u8,

    /// Total processed volume (in lamports)
    pub total_volume: u64,

    /// Total payments processed
    pub total_payments: u64,

    /// Bump seed for PDA
    pub bump: u8,
}

impl SplitterConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // treasury
        32 + // incentive_pool
        1 + // operator_share
        1 + // treasury_share
        1 + // incentive_share
        1 + // orchestrator_share
        8 + // total_volume
        8 + // total_payments
        1; // bump

    pub fn validate_shares(&self) -> bool {
        let total = self.operator_share as u16
            + self.treasury_share as u16
            + self.incentive_share as u16
            + self.orchestrator_share as u16;
        total == 100
    }

    pub fn calculate_operator_amount(&self, total: u64) -> u64 {
        (total * self.operator_share as u64) / 100
    }

    pub fn calculate_treasury_amount(&self, total: u64) -> u64 {
        (total * self.treasury_share as u64) / 100
    }

    pub fn calculate_incentive_amount(&self, total: u64) -> u64 {
        (total * self.incentive_share as u64) / 100
    }

    pub fn calculate_orchestrator_amount(&self, total: u64) -> u64 {
        (total * self.orchestrator_share as u64) / 100
    }

    pub fn record_payment(&mut self, amount: u64) {
        self.total_volume = self.total_volume.saturating_add(amount);
        self.total_payments = self.total_payments.saturating_add(1);
    }
}
