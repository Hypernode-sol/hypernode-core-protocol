use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod node_registry {
    use super::*;

    /// Register a new GPU/CPU node on-chain
    pub fn register_node(
        ctx: Context<RegisterNode>,
        node_id: String,
        gpu_specs_hash: String,
        location: String,
    ) -> Result<()> {
        instructions::register_node(ctx, node_id, gpu_specs_hash, location)
    }

    /// Update node status (online/offline/suspended)
    pub fn update_node_status(
        ctx: Context<UpdateNodeStatus>,
        status: NodeStatus,
    ) -> Result<()> {
        instructions::update_node_status(ctx, status)
    }

    /// Stake HYPER tokens to increase reputation
    pub fn stake_for_node(
        ctx: Context<StakeForNode>,
        amount: u64,
    ) -> Result<()> {
        instructions::stake_for_node(ctx, amount)
    }

    /// Send heartbeat to show node is still active
    pub fn heartbeat(ctx: Context<Heartbeat>) -> Result<()> {
        instructions::heartbeat(ctx)
    }

    /// Deregister node from the network
    pub fn deregister_node(ctx: Context<DeregisterNode>) -> Result<()> {
        instructions::deregister_node(ctx)
    }

    /// Increment jobs completed counter
    pub fn increment_jobs_completed(
        ctx: Context<IncrementJobsCompleted>,
    ) -> Result<()> {
        instructions::increment_jobs_completed(ctx)
    }

    /// Add earnings to node account
    pub fn add_earnings(
        ctx: Context<AddEarnings>,
        amount: u64,
    ) -> Result<()> {
        instructions::add_earnings(ctx, amount)
    }
}
