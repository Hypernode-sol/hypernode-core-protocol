use anchor_lang::prelude::*;
use crate::state::NodeAccount;
use crate::errors::NodeRegistryError;

#[derive(Accounts)]
pub struct UpdateHeartbeat<'info> {
    #[account(
        mut,
        has_one = owner @ NodeRegistryError::Unauthorized
    )]
    pub node_account: Account<'info, NodeAccount>,

    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateHeartbeat>) -> Result<()> {
    let node = &mut ctx.accounts.node_account;
    let clock = Clock::get()?;

    node.last_heartbeat = clock.unix_timestamp;

    // Auto set to online if heartbeat is called
    if node.status == 0 {
        node.status = 1;
    }

    msg!("Heartbeat updated for node: {}", node.node_id);
    Ok(())
}
