# deepwake
DEEPWAKE ($KRKN) - Solana pirate exploration built on Anchor

## On-chain program

The Anchor workspace contains one program, `deepwake` (program id `BdpbXkD8MrKDvtgifnAFx2cPHXma3TZFzEeHV2wSEHkD`). This first pass covers the $KRKN mint, claimable virtual islands, a time-locked treasure counter, and the Kraken guardian trade alert.

### Layout

The tree matches `anchor init` (Anchor 1.2, modular program, TypeScript tests):

| Path | Role |
| --- | --- |
| `Anchor.toml` | Localnet provider, program id, test script, and a clone of the Metaplex token-metadata program so `initialize_mint` can run under `anchor test` |
| `Cargo.toml` | Rust workspace (`programs/*`) |
| `programs/deepwake/src/lib.rs` | `#[program]` entrypoint. Each instruction delegates to a handler |
| `programs/deepwake/src/instructions/` | Handlers and `#[derive(Accounts)]` validators |
| `programs/deepwake/src/state.rs` | `Island` account |
| `programs/deepwake/src/events.rs` | `KrakenAwoken` event |
| `programs/deepwake/src/constants.rs` | Mint name/symbol, chest lock, and `THRESHOLD` |
| `programs/deepwake/src/error.rs` | Program errors |
| `tests/deepwake.ts` | Mocha tests using the Anchor TypeScript client (`@anchor-lang/core`) |
| `migrations/deploy.ts` | Anchor deploy-script stub |

### Instructions

1. **`initialize_mint`** — Creates the canonical $KRKN SPL mint (PDA seed `krkn-mint`, 9 decimals) and Metaplex metadata via `anchor-spl` (`name: DEEPWAKE`, `symbol: KRKN`). Mint and freeze authority are the `mint-authority` PDA, which also signs the metadata CPI. No supply is minted.

2. **`create_island`** — Creates an island PDA (`["island", island_id]`) owned by the signer. Stores `owner`, `island_id`, map coordinates (`x`, `y`), `treasure_chest_balance` (starts at 0), and `unlock_at`.

3. **`deposit_to_chest` / `claim_chest`** — The island owner adds to or withdraws from `treasure_chest_balance`. A deposit sets `unlock_at` to `Clock.unix_timestamp + CHEST_LOCK_SECONDS` (60 seconds in this pass) and restarts the lock. A claim requires `unix_timestamp >= unlock_at`. The balance is an on-account counter; these instructions do not move SPL tokens yet.

4. **`record_trade_event`** — Logs the trade size with `msg!`. If `amount` is strictly greater than `THRESHOLD` (1,000 KRKN in base units), emits `KrakenAwoken { amount, trader }` for an off-chain indexer. That event is the Kraken guardian anti-rug alert.

### Tests

```bash
npm install
anchor test
```

`anchor test` builds the program, starts a local validator, and runs `tests/deepwake.ts`. The default validator is Surfpool in online mode so the token-metadata program can be fetched from mainnet. `anchor test --validator legacy` uses the `[[test.validator.clone]]` entry in `Anchor.toml` instead.
