use anchor_lang::prelude::*;
use crate::state::JobAccount;
use crate::errors::JobReceiptError;

#[derive(Accounts)]
pub struct AssignJob<'info> {
    #[account(mut)]
    pub job_account: Account<'info, JobAccount>,

    /// Authority that can assign jobs (backend service)
    pub authority: Signer<'info>,

    /// CHECK: Node operator public key
    pub node_operator: AccountInfo<'info>,
}

pub fn handler(ctx: Context<AssignJob>, node_id: String) -> Result<()> {
    let job = &mut ctx.accounts.job_account;

    require!(job.can_be_assigned(), JobReceiptError::JobNotPending);

    job.assigned_node = ctx.accounts.node_operator.key();
    job.status = 1; // Assigned

    emit!(JobAssignedEvent {
        job_id: job.job_id.clone(),
        node_id,
        node_operator: ctx.accounts.node_operator.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Job {} assigned to node {}", job.job_id, node_id);
    Ok(())
}

#[event]
pub struct JobAssignedEvent {
    pub job_id: String,
    pub node_id: String,
    pub node_operator: Pubkey,
    pub timestamp: i64,
}
