use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct AddEarnings<'info> {
    #[account(mut)]
    pub node_account: Account<'info, NodeAccount>,

    // In production, called by Payment Splitter via CPI
}

pub fn add_earnings(ctx: Context<AddEarnings>, amount: u64) -> Result<()> {
    let node_account = &mut ctx.accounts.node_account;

    node_account.total_earned = node_account.total_earned.checked_add(amount).unwrap();

    Ok(())
}
