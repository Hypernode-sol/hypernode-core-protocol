use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::NodeAccount;
use crate::errors::NodeRegistryError;

#[derive(Accounts)]
pub struct StakeForNode<'info> {
    #[account(
        mut,
        has_one = owner @ NodeRegistryError::Unauthorized
    )]
    pub node_account: Account<'info, NodeAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// Owner's HYPER token account
    #[account(mut)]
    pub owner_token_account: Account<'info, TokenAccount>,

    /// Protocol's HYPER token vault for staking
    #[account(mut)]
    pub stake_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<StakeForNode>, amount: u64) -> Result<()> {
    require!(amount > 0, NodeRegistryError::InsufficientStake);

    // Transfer HYPER tokens to stake vault
    let cpi_accounts = Transfer {
        from: ctx.accounts.owner_token_account.to_account_info(),
        to: ctx.accounts.stake_vault.to_account_info(),
        authority: ctx.accounts.owner.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Update node stake amount
    let node = &mut ctx.accounts.node_account;
    node.stake_amount = node.stake_amount.saturating_add(amount);
    node.update_reputation();

    emit!(NodeStakedEvent {
        node_id: node.node_id.clone(),
        amount,
        total_stake: node.stake_amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Staked {} HYPER for node: {}", amount, node.node_id);
    Ok(())
}

#[event]
pub struct NodeStakedEvent {
    pub node_id: String,
    pub amount: u64,
    pub total_stake: u64,
    pub timestamp: i64,
}
