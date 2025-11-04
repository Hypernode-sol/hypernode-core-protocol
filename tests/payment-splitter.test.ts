import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PaymentSplitter } from "../target/types/payment_splitter";
import { assert } from "chai";

describe("payment-splitter", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PaymentSplitter as Program<PaymentSplitter>;

  const treasury = anchor.web3.Keypair.generate();
  const incentivePool = anchor.web3.Keypair.generate();

  it("Initializes splitter config", async () => {
    const [splitterConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("splitter_config")],
      program.programId
    );

    await program.methods
      .initialize(
        80, // operator
        10, // treasury
        5,  // incentive
        5   // orchestrator
      )
      .accounts({
        splitterConfig,
        authority: provider.wallet.publicKey,
        treasury: treasury.publicKey,
        incentivePool: incentivePool.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const config = await program.account.splitterConfig.fetch(splitterConfig);

    assert.equal(config.operatorShare, 80);
    assert.equal(config.treasuryShare, 10);
    assert.equal(config.incentiveShare, 5);
    assert.equal(config.orchestratorShare, 5);
    assert.equal(config.totalVolume.toNumber(), 0);
    assert.equal(config.totalPayments.toNumber(), 0);
  });

  it("Validates share percentages must sum to 100", async () => {
    const [splitterConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("splitter_config_invalid")],
      program.programId
    );

    try {
      await program.methods
        .initialize(
          70, // operator
          20, // treasury
          5,  // incentive
          10  // orchestrator (total = 105, invalid!)
        )
        .accounts({
          splitterConfig,
          authority: provider.wallet.publicKey,
          treasury: treasury.publicKey,
          incentivePool: incentivePool.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("Should have thrown error for invalid percentages");
    } catch (err) {
      assert.include(err.toString(), "InvalidSharePercentages");
    }
  });

  it("Updates config", async () => {
    const [splitterConfig] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("splitter_config")],
      program.programId
    );

    // Config already initialized in first test
    await program.methods
      .updateConfig(
        75, // operator
        15, // treasury
        5,  // incentive
        5   // orchestrator
      )
      .accounts({
        splitterConfig,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const config = await program.account.splitterConfig.fetch(splitterConfig);

    assert.equal(config.operatorShare, 75);
    assert.equal(config.treasuryShare, 15);
  });
});
