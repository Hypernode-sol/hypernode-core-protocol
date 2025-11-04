use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Gx7UjBjVKHNqiQGPzWGVxQb8j4ZKN5cVx5TmPvRfZ8U");

#[program]
pub mod job_receipt {
    use super::*;

    /// Create a new job request
    pub fn create_job(
        ctx: Context<CreateJob>,
        job_id: String,
        job_type: u8,
        price: u64,
        requirements_hash: String,
    ) -> Result<()> {
        instructions::create_job::handler(ctx, job_id, job_type, price, requirements_hash)
    }

    /// Assign job to a node
    pub fn assign_job(ctx: Context<AssignJob>, node_id: String) -> Result<()> {
        instructions::assign_job::handler(ctx, node_id)
    }

    /// Submit job result from node
    pub fn submit_result(
        ctx: Context<SubmitResult>,
        result_hash: String,
        logs_url: String,
    ) -> Result<()> {
        instructions::submit_result::handler(ctx, result_hash, logs_url)
    }

    /// Settle job payment
    pub fn settle_job(ctx: Context<SettleJob>) -> Result<()> {
        instructions::settle_job::handler(ctx)
    }

    /// Cancel job
    pub fn cancel_job(ctx: Context<CancelJob>) -> Result<()> {
        instructions::cancel_job::handler(ctx)
    }
}
