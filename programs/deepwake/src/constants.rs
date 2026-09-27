//! Program-wide constants shared by instructions and the IDL.

use anchor_lang::prelude::*;

/// PDA seed for the single $KRKN mint.
#[constant]
pub const MINT_SEED: &[u8] = b"krkn-mint";

/// PDA seed for the mint authority that signs metadata creation.
///
/// The account is not initialized; it only exists as a signing PDA.
#[constant]
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";

/// PDA seed for an [`crate::Island`]. Combined with the island id.
#[constant]
pub const ISLAND_SEED: &[u8] = b"island";

/// SPL decimals for the $KRKN mint.
#[constant]
pub const KRKN_DECIMALS: u8 = 9;

/// Metaplex metadata name. Max length enforced by token-metadata is 32 bytes.
#[constant]
pub const TOKEN_NAME: &str = "DEEPWAKE";

/// Metaplex metadata symbol. Max length enforced by token-metadata is 10 bytes.
#[constant]
pub const TOKEN_SYMBOL: &str = "KRKN";

/// Metaplex metadata URI. Empty until a JSON document is hosted.
#[constant]
pub const TOKEN_URI: &str = "";

/// Seconds added to `Clock::unix_timestamp` on every chest deposit.
///
/// A claim is rejected until that timestamp. One minute is a first-pass value
/// so a local validator can exercise the lock; raise it before mainnet.
#[constant]
pub const CHEST_LOCK_SECONDS: i64 = 60;

/// Trade size, in base units, above which [`crate::KrakenAwoken`] is emitted.
///
/// `1_000_000_000_000` is 1,000 KRKN at [`KRKN_DECIMALS`] decimals. The check
/// is strict: a trade equal to this value does not wake the Kraken.
#[constant]
pub const THRESHOLD: u64 = 1_000_000_000_000;
