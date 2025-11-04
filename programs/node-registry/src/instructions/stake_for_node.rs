use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::NodeRegistryError;

#[derive(Accounts)]
pub struct StakeForNode<'info> {
    #[account(
        mut,
        has_one = owner @ NodeRegistryError::UnauthorizedNotOwner
    )]
    pub node_account: Account<'info, NodeAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    // In a real implementation, would transfer SPL tokens here
    // pub token_account: Account<'info, TokenAccount>,
    // pub hyper_mint: Account<'info, Mint>,
    // pub token_program: Program<'info, Token>,
}

pub fn stake_for_node(
    ctx: Context<StakeForNode>,
    amount: u64,
) -> Result<()> {
    require!(amount > 0, NodeRegistryError::InsufficientStake);

    let node_account = &mut ctx.accounts.node_account;

    // In real implementation: transfer tokens to PDA
    // For MVP, just increment the stake_amount
    node_account.stake_amount = node_account.stake_amount.checked_add(amount).unwrap();

    // Increase reputation based on stake (simple linear model)
    // Each 1000 tokens staked = +10 reputation (max 1000)
    let reputation_increase = ((amount / 1_000_000_000) * 10) as u16; // Assuming 9 decimals
    node_account.reputation_score = std::cmp::min(
        1000,
        node_account.reputation_score.saturating_add(reputation_increase)
    );

    Ok(())
}
