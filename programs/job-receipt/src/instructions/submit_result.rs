use anchor_lang::prelude::*;
use crate::state::JobAccount;
use crate::errors::JobReceiptError;

#[derive(Accounts)]
pub struct SubmitResult<'info> {
    #[account(
        mut,
        has_one = assigned_node @ JobReceiptError::Unauthorized
    )]
    pub job_account: Account<'info, JobAccount>,

    /// Node operator submitting the result
    #[account(mut)]
    pub operator: Signer<'info>,

    /// CHECK: This should be the same as job_account.assigned_node
    #[account(address = job_account.assigned_node)]
    pub assigned_node: AccountInfo<'info>,
}

pub fn handler(
    ctx: Context<SubmitResult>,
    result_hash: String,
    logs_url: String,
) -> Result<()> {
    require!(result_hash.len() <= 64, JobReceiptError::ResultHashInvalid);
    require!(logs_url.len() <= 128, JobReceiptError::LogsUrlTooLong);

    let job = &mut ctx.accounts.job_account;

    require!(job.can_submit_result(), JobReceiptError::JobNotAssigned);

    let clock = Clock::get()?;

    job.result_hash = result_hash.clone();
    job.logs_url = logs_url.clone();
    job.completed_at = clock.unix_timestamp;
    job.status = 3; // Completed

    emit!(JobCompletedEvent {
        job_id: job.job_id.clone(),
        operator: ctx.accounts.operator.key(),
        result_hash,
        logs_url,
        timestamp: clock.unix_timestamp,
    });

    msg!("Job {} completed by {}", job.job_id, ctx.accounts.operator.key());
    Ok(())
}

#[event]
pub struct JobCompletedEvent {
    pub job_id: String,
    pub operator: Pubkey,
    pub result_hash: String,
    pub logs_url: String,
    pub timestamp: i64,
}
