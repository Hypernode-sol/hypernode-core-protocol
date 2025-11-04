use anchor_lang::prelude::*;
use crate::state::JobAccount;
use crate::errors::JobReceiptError;

#[derive(Accounts)]
pub struct CancelJob<'info> {
    #[account(
        mut,
        has_one = client @ JobReceiptError::Unauthorized
    )]
    pub job_account: Account<'info, JobAccount>,

    pub client: Signer<'info>,
}

pub fn handler(ctx: Context<CancelJob>) -> Result<()> {
    let job = &mut ctx.accounts.job_account;

    // Can only cancel if pending or assigned (not running or completed)
    require!(
        job.status == 0 || job.status == 1,
        JobReceiptError::InvalidJobStatus
    );

    job.status = 5; // Cancelled

    emit!(JobCancelledEvent {
        job_id: job.job_id.clone(),
        client: ctx.accounts.client.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Job cancelled: {}", job.job_id);
    Ok(())
}

#[event]
pub struct JobCancelledEvent {
    pub job_id: String,
    pub client: Pubkey,
    pub timestamp: i64,
}
