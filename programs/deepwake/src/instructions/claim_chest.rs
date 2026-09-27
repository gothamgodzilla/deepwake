//! `claim_chest` — debit an island's treasure counter after the time lock.

use anchor_lang::prelude::*;

use crate::{constants::ISLAND_SEED, error::DeepwakeError, state::Island};

/// Accounts for [`crate::claim_chest`].
///
/// The handler compares `Clock::unix_timestamp` with `island.unlock_at`.
#[derive(Accounts)]
pub struct ClaimChest<'info> {
    #[account(
        mut,
        seeds = [ISLAND_SEED, &island.island_id.to_le_bytes()],
        bump = island.bump,
        has_one = owner @ DeepwakeError::NotIslandOwner
    )]
    pub island: Account<'info, Island>,

    /// Must match `island.owner`.
    pub owner: Signer<'info>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn handle_claim_chest(ctx: Context<ClaimChest>, amount: u64) -> Result<()> {
    require!(amount > 0, DeepwakeError::ZeroAmount);
    require!(
        ctx.accounts.clock.unix_timestamp >= ctx.accounts.island.unlock_at,
        DeepwakeError::ChestLocked
    );

    let island = &mut ctx.accounts.island;
    require!(
        island.treasure_chest_balance >= amount,
        DeepwakeError::InsufficientChestBalance
    );
    island.treasure_chest_balance = island
        .treasure_chest_balance
        .checked_sub(amount)
        .ok_or(DeepwakeError::ArithmeticOverflow)?;

    msg!(
        "Claimed {} from island {} chest; remaining {}",
        amount,
        island.island_id,
        island.treasure_chest_balance
    );
    Ok(())
}
