use anchor_lang::prelude::*;
use crate::state::JobAccount;
use crate::errors::JobReceiptError;

#[derive(Accounts)]
#[instruction(job_id: String)]
pub struct CreateJob<'info> {
    #[account(
        init,
        payer = client,
        space = JobAccount::LEN,
        seeds = [b"job", client.key().as_ref(), job_id.as_bytes()],
        bump
    )]
    pub job_account: Account<'info, JobAccount>,

    #[account(mut)]
    pub client: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateJob>,
    job_id: String,
    job_type: u8,
    price: u64,
    requirements_hash: String,
) -> Result<()> {
    require!(job_id.len() <= 32, JobReceiptError::JobIdTooLong);
    require!(job_type <= 5, JobReceiptError::InvalidJobType);
    require!(price > 0, JobReceiptError::InvalidPrice);
    require!(requirements_hash.len() <= 64, JobReceiptError::RequirementsHashTooLong);

    let job = &mut ctx.accounts.job_account;
    let clock = Clock::get()?;

    job.job_id = job_id.clone();
    job.client = ctx.accounts.client.key();
    job.assigned_node = Pubkey::default();
    job.job_type = job_type;
    job.price = price;
    job.requirements_hash = requirements_hash;
    job.created_at = clock.unix_timestamp;
    job.completed_at = 0;
    job.status = 0; // Pending
    job.result_hash = String::new();
    job.logs_url = String::new();
    job.payment_settled = false;
    job.bump = ctx.bumps.job_account;

    emit!(JobCreatedEvent {
        job_id,
        client: ctx.accounts.client.key(),
        job_type,
        price,
        timestamp: clock.unix_timestamp,
    });

    msg!("Job created: {}", job.job_id);
    Ok(())
}

#[event]
pub struct JobCreatedEvent {
    pub job_id: String,
    pub client: Pubkey,
    pub job_type: u8,
    pub price: u64,
    pub timestamp: i64,
}
