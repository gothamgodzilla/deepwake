//! Errors returned by the DEEPWAKE instructions.

use anchor_lang::prelude::*;

#[error_code]
pub enum DeepwakeError {
    /// Signer is not the wallet stored on the island.
    #[msg("Only the island owner can use this chest")]
    NotIslandOwner,

    /// Deposit or claim amount was zero.
    #[msg("Amount must be greater than zero")]
    ZeroAmount,

    /// `Clock::unix_timestamp` is still earlier than the island's `unlock_at`.
    #[msg("Treasure chest is still time-locked")]
    ChestLocked,

    /// Claim amount is larger than `treasure_chest_balance`.
    #[msg("Treasure chest balance is too small for this claim")]
    InsufficientChestBalance,

    /// A balance or timestamp calculation overflowed.
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
