/**
 * Integration tests for the DEEPWAKE program.
 *
 * Uses the Anchor TypeScript client published as `@anchor-lang/core`
 * (the client previously shipped as `@coral-xyz/anchor`).
 *
 * Run with `anchor test`. `initialize_mint` CPIs into the Metaplex
 * token-metadata program, which `Anchor.toml` clones onto the local validator.
 */
import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { assert } from "chai";
import { Deepwake } from "../target/types/deepwake";

const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
const TOKEN_PROGRAM_ID = new anchor.web3.PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);
const RENT_SYSVAR_ID = new anchor.web3.PublicKey(
  "SysvarRent111111111111111111111111111111111"
);

/** Must match `THRESHOLD` in `programs/deepwake/src/constants.rs`. */
const THRESHOLD = new anchor.BN("1000000000000");

describe("deepwake", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.deepwake as Program<Deepwake>;
  const owner = provider.wallet;

  const islandId = new anchor.BN(7);
  const [islandPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("island"), islandId.toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("krkn-mint")],
    program.programId
  );
  const [mintAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint-authority")],
    program.programId
  );
  const [metadata] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("metadata"), TOKEN_METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    TOKEN_METADATA_PROGRAM_ID
  );

  it("initializes the KRKN mint and metadata", async () => {
    const tx = await program.methods
      .initializeMint()
      .accountsPartial({
        payer: owner.publicKey,
        mintAuthority,
        mint,
        metadata,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: RENT_SYSVAR_ID,
      })
      .rpc();
    console.log("initialize_mint", tx);

    const mintInfo = await provider.connection.getAccountInfo(mint);
    const metadataInfo = await provider.connection.getAccountInfo(metadata);
    assert.isNotNull(mintInfo);
    assert.isNotNull(metadataInfo);
    assert.ok(mintInfo!.owner.equals(TOKEN_PROGRAM_ID));
  });

  it("creates an island PDA for the owner", async () => {
    const tx = await program.methods
      .createIsland(islandId, 12, -4)
      .accountsPartial({
        owner: owner.publicKey,
        island: islandPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("create_island", tx);

    const island = await program.account.island.fetch(islandPda);
    assert.ok(island.owner.equals(owner.publicKey));
    assert.ok(island.islandId.eq(islandId));
    assert.equal(island.x, 12);
    assert.equal(island.y, -4);
    assert.ok(island.treasureChestBalance.eq(new anchor.BN(0)));
    assert.ok(island.unlockAt.eq(new anchor.BN(0)));
  });

  it("rejects a chest deposit from a wallet that does not own the island", async () => {
    const intruder = anchor.web3.Keypair.generate();
    try {
      await program.methods
        .depositToChest(new anchor.BN(1))
        .accountsPartial({
          island: islandPda,
          owner: intruder.publicKey,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        })
        .signers([intruder])
        .rpc();
      assert.fail("intruder deposit should fail");
    } catch (err) {
      assert.include((err as Error).message, "Only the island owner can use this chest");
    }
  });

  it("deposits into the chest and locks a claim until unlock_at", async () => {
    const amount = new anchor.BN(25);
    await program.methods
      .depositToChest(amount)
      .accountsPartial({
        island: islandPda,
        owner: owner.publicKey,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .rpc();

    const island = await program.account.island.fetch(islandPda);
    assert.ok(island.treasureChestBalance.eq(amount));
    assert.ok(island.unlockAt.gt(new anchor.BN(0)));

    try {
      await program.methods
        .claimChest(new anchor.BN(1))
        .accountsPartial({
          island: islandPda,
          owner: owner.publicKey,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        })
        .rpc();
      assert.fail("claim should be time-locked");
    } catch (err) {
      assert.include((err as Error).message, "Treasure chest is still time-locked");
    }
  });

  it("logs a small trade without waking the Kraken", async () => {
    const tx = await program.methods
      .recordTradeEvent(THRESHOLD)
      .accountsPartial({ trader: owner.publicKey })
      .rpc();
    console.log("record_trade_event at threshold", tx);
  });

  it("emits KrakenAwoken when a trade exceeds THRESHOLD", async () => {
    const amount = THRESHOLD.add(new anchor.BN(1));
    let awoken: { amount: anchor.BN; trader: anchor.web3.PublicKey } | null = null;
    const listener = program.addEventListener("krakenAwoken", (event) => {
      awoken = event;
    });

    try {
      const tx = await program.methods
        .recordTradeEvent(amount)
        .accountsPartial({ trader: owner.publicKey })
        .rpc();
      console.log("record_trade_event above threshold", tx);

      const started = Date.now();
      while (awoken === null && Date.now() - started < 5000) {
        await new Promise((resolve) => setTimeout(resolve, 200));
      }

      assert.isNotNull(awoken);
      assert.ok(awoken!.amount.eq(amount));
      assert.ok(awoken!.trader.equals(owner.publicKey));
    } finally {
      await program.removeEventListener(listener);
    }
  });
});
