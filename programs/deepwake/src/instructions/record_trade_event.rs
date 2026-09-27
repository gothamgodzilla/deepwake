//! `record_trade_event` — log a trade and optionally emit `KrakenAwoken`.

use anchor_lang::prelude::*;

use crate::{constants::THRESHOLD, events::KrakenAwoken};

/// Accounts for [`crate::record_trade_event`].
///
/// No program state is written. `trader` signs so the event cannot name an
/// arbitrary wallet.
#[derive(Accounts)]
pub struct RecordTradeEvent<'info> {
    /// Wallet recorded on [`KrakenAwoken`] when the trade exceeds [`THRESHOLD`].
    pub trader: Signer<'info>,
}

pub fn handle_record_trade_event(ctx: Context<RecordTradeEvent>, amount: u64) -> Result<()> {
    msg!("DEEPWAKE trade size: {}", amount);

    if amount > THRESHOLD {
        emit!(KrakenAwoken {
            amount,
            trader: ctx.accounts.trader.key(),
        });
        msg!(
            "Kraken awoken by {} for {}",
            ctx.accounts.trader.key(),
            amount
        );
    }

    Ok(())
}
