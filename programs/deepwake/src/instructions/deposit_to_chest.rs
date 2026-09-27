//! `deposit_to_chest` — credit an island's treasure counter and start the lock.

use anchor_lang::prelude::*;

use crate::{
    constants::{CHEST_LOCK_SECONDS, ISLAND_SEED},
    error::DeepwakeError,
    state::Island,
};

/// Accounts for [`crate::deposit_to_chest`].
///
/// `has_one = owner` rejects any signer other than the wallet stored on the island.
#[derive(Accounts)]
pub struct DepositToChest<'info> {
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

pub fn handle_deposit_to_chest(ctx: Context<DepositToChest>, amount: u64) -> Result<()> {
    require!(amount > 0, DeepwakeError::ZeroAmount);

    let unlock_at = ctx
        .accounts
        .clock
        .unix_timestamp
        .checked_add(CHEST_LOCK_SECONDS)
        .ok_or(DeepwakeError::ArithmeticOverflow)?;

    let island = &mut ctx.accounts.island;
    island.treasure_chest_balance = island
        .treasure_chest_balance
        .checked_add(amount)
        .ok_or(DeepwakeError::ArithmeticOverflow)?;
    island.unlock_at = unlock_at;

    msg!(
        "Deposited {} into island {} chest; locked until {}",
        amount,
        island.island_id,
        unlock_at
    );
    Ok(())
}
