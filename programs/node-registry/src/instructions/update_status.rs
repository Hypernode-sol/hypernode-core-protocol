use anchor_lang::prelude::*;
use crate::state::NodeAccount;
use crate::errors::NodeRegistryError;

#[derive(Accounts)]
pub struct UpdateNodeStatus<'info> {
    #[account(
        mut,
        has_one = owner @ NodeRegistryError::Unauthorized
    )]
    pub node_account: Account<'info, NodeAccount>,

    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateNodeStatus>, status: u8) -> Result<()> {
    require!(status <= 2, NodeRegistryError::InvalidNodeStatus);

    let node = &mut ctx.accounts.node_account;
    node.status = status;

    emit!(NodeStatusUpdatedEvent {
        node_id: node.node_id.clone(),
        status,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Node status updated: {} -> {}", node.node_id, status);
    Ok(())
}

#[event]
pub struct NodeStatusUpdatedEvent {
    pub node_id: String,
    pub status: u8,
    pub timestamp: i64,
}
