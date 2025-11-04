import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NodeRegistry } from "../target/types/node_registry";
import { assert } from "chai";

describe("node-registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NodeRegistry as Program<NodeRegistry>;

  it("Registers a new node", async () => {
    const nodeId = "test-node-" + Date.now();
    const gpuSpecsHash = "hash_nvidia_rtx_4090_24gb";
    const location = "us-east-1";

    const [nodeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("node"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(nodeId),
      ],
      program.programId
    );

    await program.methods
      .registerNode(nodeId, gpuSpecsHash, location)
      .accounts({
        nodeAccount,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const node = await program.account.nodeAccount.fetch(nodeAccount);

    assert.equal(node.nodeId, nodeId);
    assert.equal(node.gpuSpecsHash, gpuSpecsHash);
    assert.equal(node.location, location);
    assert.equal(node.status, 1); // Online
    assert.equal(node.reputationScore, 500); // Neutral
  });

  it("Updates node status", async () => {
    const nodeId = "test-node-status-" + Date.now();

    const [nodeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("node"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(nodeId),
      ],
      program.programId
    );

    // Register node first
    await program.methods
      .registerNode(nodeId, "hash", "location")
      .accounts({
        nodeAccount,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Update status to offline (0)
    await program.methods
      .updateNodeStatus(0)
      .accounts({
        nodeAccount,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    const node = await program.account.nodeAccount.fetch(nodeAccount);
    assert.equal(node.status, 0); // Offline
  });

  it("Updates heartbeat", async () => {
    const nodeId = "test-node-heartbeat-" + Date.now();

    const [nodeAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("node"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(nodeId),
      ],
      program.programId
    );

    await program.methods
      .registerNode(nodeId, "hash", "location")
      .accounts({
        nodeAccount,
        owner: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const nodeBefore = await program.account.nodeAccount.fetch(nodeAccount);
    const heartbeatBefore = nodeBefore.lastHeartbeat.toNumber();

    // Wait a bit
    await new Promise(resolve => setTimeout(resolve, 2000));

    await program.methods
      .updateHeartbeat()
      .accounts({
        nodeAccount,
        owner: provider.wallet.publicKey,
      })
      .rpc();

    const nodeAfter = await program.account.nodeAccount.fetch(nodeAccount);
    const heartbeatAfter = nodeAfter.lastHeartbeat.toNumber();

    assert.isTrue(heartbeatAfter > heartbeatBefore);
  });
});
