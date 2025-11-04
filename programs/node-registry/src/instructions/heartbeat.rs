use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::NodeRegistryError;

#[derive(Accounts)]
pub struct Heartbeat<'info> {
    #[account(
        mut,
        has_one = owner @ NodeRegistryError::UnauthorizedNotOwner
    )]
    pub node_account: Account<'info, NodeAccount>,

    pub owner: Signer<'info>,
}

pub fn heartbeat(ctx: Context<Heartbeat>) -> Result<()> {
    let node_account = &mut ctx.accounts.node_account;
    let clock = Clock::get()?;

    // Prevent heartbeat spam (minimum 30 seconds between heartbeats)
    let time_since_last = clock.unix_timestamp - node_account.last_heartbeat;
    require!(time_since_last >= 30, NodeRegistryError::HeartbeatTooFrequent);

    node_account.last_heartbeat = clock.unix_timestamp;

    // If node was offline, bring it back online
    if node_account.status == NodeStatus::Offline {
        node_account.status = NodeStatus::Online;
    }

    Ok(())
}
