use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::NodeRegistryError;

#[derive(Accounts)]
pub struct UpdateNodeStatus<'info> {
    #[account(
        mut,
        has_one = owner @ NodeRegistryError::UnauthorizedNotOwner
    )]
    pub node_account: Account<'info, NodeAccount>,

    pub owner: Signer<'info>,
}

pub fn update_node_status(
    ctx: Context<UpdateNodeStatus>,
    status: NodeStatus,
) -> Result<()> {
    let node_account = &mut ctx.accounts.node_account;
    let clock = Clock::get()?;

    node_account.status = status.clone();
    node_account.last_heartbeat = clock.unix_timestamp;

    emit!(NodeStatusUpdated {
        node_id: node_account.node_id.clone(),
        status,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct NodeStatusUpdated {
    pub node_id: String,
    pub status: NodeStatus,
    pub timestamp: i64,
}
