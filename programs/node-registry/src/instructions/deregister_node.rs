use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::NodeRegistryError;

#[derive(Accounts)]
pub struct DeregisterNode<'info> {
    #[account(
        mut,
        has_one = owner @ NodeRegistryError::UnauthorizedNotOwner,
        close = owner
    )]
    pub node_account: Account<'info, NodeAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn deregister_node(ctx: Context<DeregisterNode>) -> Result<()> {
    let node_account = &ctx.accounts.node_account;
    let clock = Clock::get()?;

    emit!(NodeDeregistered {
        node_id: node_account.node_id.clone(),
        owner: node_account.owner,
        timestamp: clock.unix_timestamp,
    });

    // Account will be closed automatically due to close constraint
    Ok(())
}

#[event]
pub struct NodeDeregistered {
    pub node_id: String,
    pub owner: Pubkey,
    pub timestamp: i64,
}
