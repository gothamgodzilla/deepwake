//! Events an off-chain indexer can subscribe to.

use anchor_lang::prelude::*;

/// Emitted by [`crate::record_trade_event`] when a trade's size exceeds
/// [`crate::THRESHOLD`].
///
/// This is the Kraken guardian anti-rug alert. Indexers should treat it as a
/// signal that `trader` just moved more than the threshold, not as a
/// settlement. Trades at or below the threshold are only written to program
/// logs via `msg!`.
#[event]
pub struct KrakenAwoken {
    /// Trade size in base units. Always greater than [`crate::THRESHOLD`].
    pub amount: u64,
    /// Signer of `record_trade_event`.
    pub trader: Pubkey,
}
