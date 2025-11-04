use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct NodeAccount {
    /// Node owner wallet public key
    pub owner: Pubkey,

    /// Unique node identifier (max 32 chars)
    pub node_id: String,

    /// Hash of GPU specifications
    pub gpu_specs_hash: String,

    /// Geographic location hint
    pub location: String,

    /// Unix timestamp of registration
    pub registered_at: i64,

    /// Unix timestamp of last heartbeat
    pub last_heartbeat: i64,

    /// Node status (0=offline, 1=online, 2=suspended)
    pub status: u8,

    /// Amount of HYPER staked (in lamports)
    pub stake_amount: u64,

    /// Reputation score (0-1000)
    pub reputation_score: u16,

    /// Total jobs completed successfully
    pub jobs_completed: u64,

    /// Total jobs failed
    pub jobs_failed: u64,

    /// Total HYPER earned (in lamports)
    pub total_earned: u64,

    /// Bump seed for PDA
    pub bump: u8,
}

impl NodeAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        (4 + 32) + // node_id
        (4 + 64) + // gpu_specs_hash
        (4 + 32) + // location
        8 + // registered_at
        8 + // last_heartbeat
        1 + // status
        8 + // stake_amount
        2 + // reputation_score
        8 + // jobs_completed
        8 + // jobs_failed
        8 + // total_earned
        1; // bump

    pub fn is_online(&self) -> bool {
        self.status == 1
    }

    pub fn is_suspended(&self) -> bool {
        self.status == 2
    }

    pub fn increment_completed_jobs(&mut self) {
        self.jobs_completed = self.jobs_completed.saturating_add(1);
        self.update_reputation();
    }

    pub fn increment_failed_jobs(&mut self) {
        self.jobs_failed = self.jobs_failed.saturating_add(1);
        self.update_reputation();
    }

    pub fn add_earnings(&mut self, amount: u64) {
        self.total_earned = self.total_earned.saturating_add(amount);
    }

    pub fn update_reputation(&mut self) {
        let total_jobs = self.jobs_completed.saturating_add(self.jobs_failed);
        if total_jobs == 0 {
            self.reputation_score = 500; // neutral starting score
            return;
        }

        // Simple reputation: (completed / total) * 1000
        let success_rate = (self.jobs_completed as f64 / total_jobs as f64) * 1000.0;

        // Bonus for stake (up to 10% boost)
        let stake_bonus = (self.stake_amount as f64 / 1_000_000_000.0).min(100.0);

        let final_score = (success_rate + stake_bonus).min(1000.0);
        self.reputation_score = final_score as u16;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Offline = 0,
    Online = 1,
    Suspended = 2,
}
