//! DEEPWAKE ($KRKN) — first on-chain program.
//!
//! The program does four things:
//!
//! * [`deepwake::initialize_mint`] creates the $KRKN SPL mint and its Metaplex metadata.
//! * [`deepwake::create_island`] claims a virtual island PDA for an owner.
//! * [`deepwake::deposit_to_chest`] / [`deepwake::claim_chest`] move a time-locked
//!   treasure counter stored on that island.
//! * [`deepwake::record_trade_event`] logs a trade and, above [`THRESHOLD`], emits
//!   [`KrakenAwoken`] for off-chain indexers (the Kraken guardian alert).

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("BdpbXkD8MrKDvtgifnAFx2cPHXma3TZFzEeHV2wSEHkD");

#[program]
pub mod deepwake {
    use super::*;

    /// Create the canonical $KRKN mint and attach Metaplex metadata.
    ///
    /// The mint address is the PDA derived from [`MINT_SEED`], so the
    /// instruction can succeed only once. Decimals, name (`DEEPWAKE`), and
    /// symbol (`KRKN`) come from [`constants`]. Mint and freeze authority are
    /// the [`MINT_AUTHORITY_SEED`] PDA, which also signs the metadata CPI.
    /// This instruction does not mint supply.
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        instructions::initialize_mint::handle_initialize_mint(ctx)
    }

    /// Claim a virtual island.
    ///
    /// `island_id` is the global id and the PDA seed, so each id can be
    /// created once. `x` and `y` are the island's map coordinates and are
    /// stored for clients; they are not required to be unique. The chest
    /// starts empty and unlocked (`unlock_at = 0`) until the first deposit.
    pub fn create_island(ctx: Context<CreateIsland>, island_id: u64, x: i32, y: i32) -> Result<()> {
        instructions::create_island::handle_create_island(ctx, island_id, x, y)
    }

    /// Add `amount` to the island's treasure chest and restart the time lock.
    ///
    /// Only the stored owner may call this. The balance is an on-account
    /// counter, not an SPL token transfer. `unlock_at` becomes
    /// `Clock::unix_timestamp + CHEST_LOCK_SECONDS`, so a new deposit delays
    /// the next successful claim.
    pub fn deposit_to_chest(ctx: Context<DepositToChest>, amount: u64) -> Result<()> {
        instructions::deposit_to_chest::handle_deposit_to_chest(ctx, amount)
    }

    /// Withdraw `amount` from the island's treasure chest.
    ///
    /// Only the stored owner may call this, and only once
    /// `Clock::unix_timestamp >= island.unlock_at`. The counter is decreased;
    /// no tokens are transferred in this first pass.
    pub fn claim_chest(ctx: Context<ClaimChest>, amount: u64) -> Result<()> {
        instructions::claim_chest::handle_claim_chest(ctx, amount)
    }

    /// Record a trade for the Kraken guardian.
    ///
    /// Always logs `amount` with [`msg!`]. When `amount` is strictly greater
    /// than [`THRESHOLD`], emits [`KrakenAwoken`] so an off-chain indexer can
    /// treat the trade as an anti-rug alert. The signer is stored as `trader`.
    pub fn record_trade_event(ctx: Context<RecordTradeEvent>, amount: u64) -> Result<()> {
        instructions::record_trade_event::handle_record_trade_event(ctx, amount)
    }
}
