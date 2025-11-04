import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { JobReceipt } from "../target/types/job_receipt";
import { assert } from "chai";

describe("job-receipt", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.JobReceipt as Program<JobReceipt>;

  it("Creates a new job", async () => {
    const jobId = "job-" + Date.now();
    const jobType = 0; // LlmInference
    const price = new anchor.BN(1000000); // 1 HYPER
    const requirementsHash = "hash_qwen_7b_inference";

    const [jobAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("job"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(jobId),
      ],
      program.programId
    );

    await program.methods
      .createJob(jobId, jobType, price, requirementsHash)
      .accounts({
        jobAccount,
        client: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const job = await program.account.jobAccount.fetch(jobAccount);

    assert.equal(job.jobId, jobId);
    assert.equal(job.jobType, jobType);
    assert.equal(job.price.toNumber(), price.toNumber());
    assert.equal(job.status, 0); // Pending
    assert.equal(job.paymentSettled, false);
  });

  it("Assigns job to a node", async () => {
    const jobId = "job-assign-" + Date.now();
    const nodeOperator = anchor.web3.Keypair.generate();
    const nodeId = "node-123";

    const [jobAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("job"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(jobId),
      ],
      program.programId
    );

    // Create job first
    await program.methods
      .createJob(jobId, 0, new anchor.BN(1000000), "hash")
      .accounts({
        jobAccount,
        client: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    // Assign job
    await program.methods
      .assignJob(nodeId)
      .accounts({
        jobAccount,
        authority: provider.wallet.publicKey,
        nodeOperator: nodeOperator.publicKey,
      })
      .rpc();

    const job = await program.account.jobAccount.fetch(jobAccount);
    assert.equal(job.status, 1); // Assigned
    assert.equal(job.assignedNode.toString(), nodeOperator.publicKey.toString());
  });

  it("Submits job result", async () => {
    const jobId = "job-result-" + Date.now();
    const nodeOperator = provider.wallet; // Using provider wallet as operator for test
    const resultHash = "result_hash_abc123";
    const logsUrl = "https://ipfs.io/ipfs/QmExample";

    const [jobAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("job"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(jobId),
      ],
      program.programId
    );

    // Create and assign job
    await program.methods
      .createJob(jobId, 0, new anchor.BN(1000000), "hash")
      .accounts({
        jobAccount,
        client: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    await program.methods
      .assignJob("node-1")
      .accounts({
        jobAccount,
        authority: provider.wallet.publicKey,
        nodeOperator: nodeOperator.publicKey,
      })
      .rpc();

    // Submit result
    await program.methods
      .submitResult(resultHash, logsUrl)
      .accounts({
        jobAccount,
        operator: nodeOperator.publicKey,
        assignedNode: nodeOperator.publicKey,
      })
      .rpc();

    const job = await program.account.jobAccount.fetch(jobAccount);
    assert.equal(job.status, 3); // Completed
    assert.equal(job.resultHash, resultHash);
    assert.equal(job.logsUrl, logsUrl);
    assert.isTrue(job.completedAt.toNumber() > 0);
  });

  it("Cancels a job", async () => {
    const jobId = "job-cancel-" + Date.now();

    const [jobAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("job"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(jobId),
      ],
      program.programId
    );

    await program.methods
      .createJob(jobId, 0, new anchor.BN(1000000), "hash")
      .accounts({
        jobAccount,
        client: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    await program.methods
      .cancelJob()
      .accounts({
        jobAccount,
        client: provider.wallet.publicKey,
      })
      .rpc();

    const job = await program.account.jobAccount.fetch(jobAccount);
    assert.equal(job.status, 5); // Cancelled
  });
});
