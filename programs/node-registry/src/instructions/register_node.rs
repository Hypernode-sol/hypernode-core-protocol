use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::NodeRegistryError;

#[derive(Accounts)]
#[instruction(node_id: String)]
pub struct RegisterNode<'info> {
    #[account(
        init,
        payer = owner,
        space = NodeAccount::LEN,
        seeds = [b"node", owner.key().as_ref(), node_id.as_bytes()],
        bump
    )]
    pub node_account: Account<'info, NodeAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn register_node(
    ctx: Context<RegisterNode>,
    node_id: String,
    gpu_specs_hash: String,
    location: String,
) -> Result<()> {
    require!(node_id.len() <= 64, NodeRegistryError::NodeIdTooLong);
    require!(gpu_specs_hash.len() <= 64, NodeRegistryError::InvalidGpuSpecsHash);
    require!(location.len() <= 64, NodeRegistryError::LocationTooLong);

    let node_account = &mut ctx.accounts.node_account;
    let clock = Clock::get()?;

    node_account.owner = ctx.accounts.owner.key();
    node_account.node_id = node_id.clone();
    node_account.gpu_specs_hash = gpu_specs_hash;
    node_account.location = location;
    node_account.registered_at = clock.unix_timestamp;
    node_account.last_heartbeat = clock.unix_timestamp;
    node_account.status = NodeStatus::Online;
    node_account.stake_amount = 0;
    node_account.reputation_score = 100; // Starting reputation
    node_account.jobs_completed = 0;
    node_account.total_earned = 0;
    node_account.bump = ctx.bumps.node_account;

    emit!(NodeRegistered {
        node_id,
        owner: ctx.accounts.owner.key(),
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[event]
pub struct NodeRegistered {
    pub node_id: String,
    pub owner: Pubkey,
    pub timestamp: i64,
}
