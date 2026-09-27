//! Accounts owned by the DEEPWAKE program.

use anchor_lang::prelude::*;

/// A claimable virtual island.
///
/// Address: PDA `[ISLAND_SEED, island_id.to_le_bytes()]`.
///
/// `treasure_chest_balance` is an on-account counter. Deposit and claim update
/// this number; they do not move SPL tokens in this first pass.
#[account]
#[derive(InitSpace)]
pub struct Island {
    /// Wallet allowed to deposit into and claim this chest.
    pub owner: Pubkey,
    /// Global id. Also the PDA seed, so it cannot change after creation.
    pub island_id: u64,
    /// Map coordinate. Not part of the PDA and not required to be unique.
    pub x: i32,
    /// Map coordinate. Not part of the PDA and not required to be unique.
    pub y: i32,
    /// Treasure recorded on the island. Increased by deposit, decreased by claim.
    pub treasure_chest_balance: u64,
    /// Unix timestamp at which [`crate::claim_chest`] starts succeeding.
    ///
    /// Zero until the first deposit. Each deposit sets this to
    /// `now + CHEST_LOCK_SECONDS`.
    pub unlock_at: i64,
    /// Canonical bump for the island PDA.
    pub bump: u8,
}
