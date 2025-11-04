use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct IncrementJobsCompleted<'info> {
    #[account(mut)]
    pub node_account: Account<'info, NodeAccount>,

    // In production, this would be called by Job Receipt program via CPI
    // For now, we'll allow direct calls for testing
}

pub fn increment_jobs_completed(ctx: Context<IncrementJobsCompleted>) -> Result<()> {
    let node_account = &mut ctx.accounts.node_account;

    node_account.jobs_completed = node_account.jobs_completed.checked_add(1).unwrap();

    // Slightly increase reputation for each completed job (max 1000)
    if node_account.reputation_score < 1000 {
        node_account.reputation_score = node_account.reputation_score.saturating_add(1);
    }

    Ok(())
}
