use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct JobAccount {
    /// Unique job identifier
    pub job_id: String,

    /// Job requester wallet
    pub client: Pubkey,

    /// Assigned node operator (if assigned)
    pub assigned_node: Pubkey,

    /// Job type (0-5)
    pub job_type: u8,

    /// Job price in HYPER (lamports)
    pub price: u64,

    /// Hash of job requirements
    pub requirements_hash: String,

    /// Unix timestamp of creation
    pub created_at: i64,

    /// Unix timestamp of completion
    pub completed_at: i64,

    /// Job status (0=pending, 1=assigned, 2=running, 3=completed, 4=failed, 5=cancelled)
    pub status: u8,

    /// Hash of job result
    pub result_hash: String,

    /// URL to logs (IPFS/S3)
    pub logs_url: String,

    /// Whether payment has been settled
    pub payment_settled: bool,

    /// Bump seed for PDA
    pub bump: u8,
}

impl JobAccount {
    pub const LEN: usize = 8 + // discriminator
        (4 + 32) + // job_id
        32 + // client
        32 + // assigned_node
        1 + // job_type
        8 + // price
        (4 + 64) + // requirements_hash
        8 + // created_at
        8 + // completed_at
        1 + // status
        (4 + 64) + // result_hash
        (4 + 128) + // logs_url
        1 + // payment_settled
        1; // bump

    pub fn is_pending(&self) -> bool {
        self.status == 0
    }

    pub fn is_assigned(&self) -> bool {
        self.status == 1
    }

    pub fn is_running(&self) -> bool {
        self.status == 2
    }

    pub fn is_completed(&self) -> bool {
        self.status == 3
    }

    pub fn is_failed(&self) -> bool {
        self.status == 4
    }

    pub fn is_cancelled(&self) -> bool {
        self.status == 5
    }

    pub fn can_be_assigned(&self) -> bool {
        self.status == 0 // Only pending jobs can be assigned
    }

    pub fn can_submit_result(&self) -> bool {
        self.status == 1 || self.status == 2 // Assigned or running
    }

    pub fn can_be_settled(&self) -> bool {
        self.status == 3 && !self.payment_settled // Completed and not yet settled
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum JobType {
    LlmInference = 0,
    LlmFineTuning = 1,
    RagIndexing = 2,
    VisionPipeline = 3,
    Render = 4,
    GenericCompute = 5,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum JobStatus {
    Pending = 0,
    Assigned = 1,
    Running = 2,
    Completed = 3,
    Failed = 4,
    Cancelled = 5,
}
