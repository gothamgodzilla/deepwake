//! `create_island` — claim a virtual island PDA.

use anchor_lang::prelude::*;

use crate::{constants::ISLAND_SEED, state::Island};

/// Accounts for [`crate::create_island`].
///
/// The island PDA is seeded by `island_id`, so the id is the claim key.
#[derive(Accounts)]
#[instruction(island_id: u64)]
pub struct CreateIsland<'info> {
    /// Becomes `Island::owner` and pays account rent.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// New island account. Closed only by a later instruction (none yet).
    #[account(
        init,
        payer = owner,
        space = 8 + Island::INIT_SPACE,
        seeds = [ISLAND_SEED, &island_id.to_le_bytes()],
        bump
    )]
    pub island: Account<'info, Island>,

    pub system_program: Program<'info, System>,
}

pub fn handle_create_island(
    ctx: Context<CreateIsland>,
    island_id: u64,
    x: i32,
    y: i32,
) -> Result<()> {
    let island = &mut ctx.accounts.island;
    island.owner = ctx.accounts.owner.key();
    island.island_id = island_id;
    island.x = x;
    island.y = y;
    island.treasure_chest_balance = 0;
    island.unlock_at = 0;
    island.bump = ctx.bumps.island;

    msg!(
        "Island {} claimed by {} at ({}, {})",
        island_id,
        island.owner,
        x,
        y
    );
    Ok(())
}
