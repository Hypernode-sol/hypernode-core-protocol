use anchor_lang::prelude::*;

/// Node account structure stored on-chain
#[account]
pub struct NodeAccount {
    /// Node operator's wallet address
    pub owner: Pubkey,
    /// Unique node identifier (UUID)
    pub node_id: String,
    /// Hash of GPU specifications (for verification)
    pub gpu_specs_hash: String,
    /// Approximate location (country/city)
    pub location: String,
    /// Unix timestamp of registration
    pub registered_at: i64,
    /// Unix timestamp of last heartbeat
    pub last_heartbeat: i64,
    /// Current node status
    pub status: NodeStatus,
    /// Staked HYPER amount (in lamports)
    pub stake_amount: u64,
    /// Reputation score (0-1000)
    pub reputation_score: u16,
    /// Total number of jobs completed
    pub jobs_completed: u64,
    /// Total HYPER earned (in lamports)
    pub total_earned: u64,
    /// Bump seed for PDA
    pub bump: u8,
}

impl NodeAccount {
    /// Calculate space needed for account
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        4 + 64 + // node_id (max 64 chars)
        4 + 64 + // gpu_specs_hash
        4 + 64 + // location
        8 + // registered_at
        8 + // last_heartbeat
        1 + // status
        8 + // stake_amount
        2 + // reputation_score
        8 + // jobs_completed
        8 + // total_earned
        1; // bump
}

/// Node status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum NodeStatus {
    Online,
    Offline,
    Suspended,
}

impl Default for NodeStatus {
    fn default() -> Self {
        NodeStatus::Offline
    }
}
