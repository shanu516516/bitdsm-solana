import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BitdsmSolana } from "../target/types/bitdsm_solana";
import { expect } from "chai";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

describe("bitdsm-solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BitdsmSolana as Program<BitdsmSolana>;

  // Test accounts
  let registryKeypair: Keypair;
  let podKeypair: Keypair;
  let operatorKeypair: Keypair;
  let userKeypair: Keypair;

  before(async () => {
    registryKeypair = Keypair.generate();
    podKeypair = Keypair.generate();
    operatorKeypair = Keypair.generate();
    userKeypair = Keypair.generate();
  });

  it("Can initialize registry", async () => {
    const minStakeWeight = new anchor.BN(1000);

    await program.methods
      .initializeRegistry(minStakeWeight)
      .accounts({
        registry: registryKeypair.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([registryKeypair])
      .rpc();

    const registryAccount = await program.account.registry.fetch(
      registryKeypair.publicKey
    );
    expect(registryAccount.authority.toString()).to.equal(
      provider.wallet.publicKey.toString()
    );
    expect(registryAccount.minStakeWeight.toNumber()).to.equal(1000);
  });

  it("Can register operator", async () => {
    const btcPubKey =
      "02a1633cafcc01ebfb6d78e39f687a1f0995c62fc95f51ead10a02ee0be551b5dc";

    await program.methods
      .registerOperator(btcPubKey)
      .accounts({
        operator: operatorKeypair.publicKey,
        registry: registryKeypair.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([operatorKeypair])
      .rpc();

    const operatorAccount = await program.account.operator.fetch(
      operatorKeypair.publicKey
    );
    expect(operatorAccount.btcPubKey).to.equal(btcPubKey);
    expect(operatorAccount.isActive).to.be.true;
  });

  it("Can create pod", async () => {
    const btcAddress = "76a914ca29dfa9e97fa4f0623742e4f7b90f81bfe5671b88ac";

    await program.methods
      .createPod(btcAddress)
      .accounts({
        pod: podKeypair.publicKey,
        operator: operatorKeypair.publicKey,
        user: userKeypair.publicKey,
        registry: registryKeypair.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([podKeypair, userKeypair])
      .rpc();

    const podAccount = await program.account.pod.fetch(podKeypair.publicKey);
    expect(podAccount.btcAddress).to.equal(btcAddress);
    expect(podAccount.operator.toString()).to.equal(
      operatorKeypair.publicKey.toString()
    );
    expect(podAccount.owner.toString()).to.equal(
      userKeypair.publicKey.toString()
    );
  });

  it("Can confirm deposit", async () => {
    const txId =
      "965d5c75ae6c7a68761e6f9cf2657363bd97f11fc6727410adacd7f81368541b";
    const amount = new anchor.BN(100000000); // 1 BTC in satoshis

    await program.methods
      .confirmDeposit(amount)
      .accounts({
        pod: podKeypair.publicKey,
        operator: operatorKeypair.publicKey,
        registry: registryKeypair.publicKey,
        authority: provider.wallet.publicKey,
      })
      .signers([])
      .rpc();

    const podAccount = await program.account.pod.fetch(podKeypair.publicKey);
    expect(podAccount.balance.toNumber()).to.equal(amount.toNumber());
    expect(podAccount.lastTxId).to.equal(txId);
  });
});
