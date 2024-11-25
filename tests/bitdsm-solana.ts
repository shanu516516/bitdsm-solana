import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BitdsmSolana } from "../target/types/bitdsm_solana";
import { assert } from "chai";
import { SystemProgram } from "@solana/web3.js";

describe("bitdsm-solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BitdsmSolana as Program<BitdsmSolana>;

  // Test accounts
  let registryKeypair: anchor.web3.Keypair;
  let appKeypair: anchor.web3.Keypair;
  let podKeypair: anchor.web3.Keypair;

  before(async () => {
    registryKeypair = anchor.web3.Keypair.generate();
    appKeypair = anchor.web3.Keypair.generate();
    podKeypair = anchor.web3.Keypair.generate();
  });

  describe("Registry", () => {
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
      assert.equal(registryAccount.authority.toString(), provider.wallet.publicKey.toString());
      assert.equal(registryAccount.minStakeWeight.toNumber(), 1000);
      assert.equal(registryAccount.operatorCount.toNumber(), 0);
      assert.equal(registryAccount.totalStake.toNumber(), 0);
    });

    it("Cannot initialize registry with zero stake weight", async () => {
      const minStakeWeight = new anchor.BN(0);
      const newRegistryKeypair = anchor.web3.Keypair.generate();

      try {
        await program.methods
          .initializeRegistry(minStakeWeight)
          .accounts({
            registry: newRegistryKeypair.publicKey,
            authority: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([newRegistryKeypair])
          .rpc();
        assert.fail("Should have failed with invalid stake weight");
      } catch (error) {
        assert.equal(error.error.errorMessage, "Invalid stake weight");
      }
    });
  });

  describe("Applications", () => {
    it("Can register application", async () => {
      const appName = "Test App";
      const appMetadata = JSON.stringify({ version: "1.0.0" });

      await program.methods
        .registerApp(appName, appMetadata)
        .accounts({
          app: appKeypair.publicKey,
          authority: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([appKeypair])
        .rpc();

      const appAccount = await program.account.app.fetch(appKeypair.publicKey);
      assert.equal(appAccount.authority.toString(), provider.wallet.publicKey.toString());
      assert.equal(appAccount.name, appName);
      assert.equal(appAccount.metadata, appMetadata);
      assert.equal(appAccount.isActive, true);
      assert.equal(appAccount.createdAt.toNumber() > 0, true);
    });

    it("Cannot register app with empty name", async () => {
      const newAppKeypair = anchor.web3.Keypair.generate();
      try {
        await program.methods
          .registerApp("", "metadata")
          .accounts({
            app: newAppKeypair.publicKey,
            authority: provider.wallet.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([newAppKeypair])
          .rpc();
        assert.fail("Should have failed with invalid app name");
      } catch (error) {
        assert.equal(error.error.errorMessage, "Invalid application name");
      }
    });
  });

  describe("Pods", () => {
    const validBtcPubKey = "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798";
    const invalidBtcPubKey = "invalid";
    
    it("Can create pod", async () => {
      const [podPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("pod"),
          provider.wallet.publicKey.toBuffer(),
          Buffer.from(validBtcPubKey).subarray(0, 8)
        ],
        program.programId
      );
      
      await program.methods
        .createPod(validBtcPubKey)
        .accounts({
          pod: podPda,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
        
      const pod = await program.account.pod.fetch(podPda);
      assert.equal(pod.authority.toString(), provider.wallet.publicKey.toString());
      assert.equal(pod.btcPublicKey, validBtcPubKey);
      assert.equal(pod.isActive, true);
      assert.equal(pod.balance.toNumber(), 0);
    });
    
    it("Cannot create pod with invalid BTC public key", async () => {
      const [podPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("pod"),
          provider.wallet.publicKey.toBuffer(),
          Buffer.from(invalidBtcPubKey).subarray(0, 8)
        ],
        program.programId
      );
      
      try {
        await program.methods
          .createPod(invalidBtcPubKey)
          .accounts({
            pod: podPda,
            authority: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        assert.fail("Should have failed");
      } catch (err: any) {
        assert.equal(err.error.errorMessage, "Invalid Bitcoin public key");
      }
    });
    
    it("Can confirm deposit", async () => {
      const [podPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("pod"),
          provider.wallet.publicKey.toBuffer(),
          Buffer.from(validBtcPubKey).subarray(0, 8)
        ],
        program.programId
      );
      
      // First create a pod
      await program.methods
        .createPod(validBtcPubKey)
        .accounts({
          pod: podPda,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
        
      // Then confirm deposit
      const amount = new anchor.BN(100);
      await program.methods
        .confirmDeposit(amount)
        .accounts({
          pod: podPda,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
        
      const pod = await program.account.pod.fetch(podPda);
      assert.equal(pod.balance.toString(), amount.toString());
    });
    
    it("Cannot confirm deposit with zero amount", async () => {
      const [podPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("pod"),
          provider.wallet.publicKey.toBuffer(),
          Buffer.from(validBtcPubKey).subarray(0, 8)
        ],
        program.programId
      );
      
      // First create a pod
      await program.methods
        .createPod(validBtcPubKey)
        .accounts({
          pod: podPda,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
        
      try {
        await program.methods
          .confirmDeposit(new anchor.BN(0))
          .accounts({
            pod: podPda,
            authority: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        assert.fail("Should have failed");
      } catch (err: any) {
        assert.equal(err.error.errorMessage, "Invalid amount");
      }
    });
    
    it("Cannot confirm deposit with wrong authority", async () => {
      const [podPda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("pod"),
          provider.wallet.publicKey.toBuffer(),
          Buffer.from(validBtcPubKey).subarray(0, 8)
        ],
        program.programId
      );
      
      // First create a pod
      await program.methods
        .createPod(validBtcPubKey)
        .accounts({
          pod: podPda,
          authority: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
        
      // Create a new keypair to act as wrong authority
      const wrongAuthority = anchor.web3.Keypair.generate();
      
      try {
        await program.methods
          .confirmDeposit(new anchor.BN(100))
          .accounts({
            pod: podPda,
            authority: wrongAuthority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([wrongAuthority])
          .rpc();
        assert.fail("Should have failed");
      } catch (err: any) {
        assert.equal(err.error.errorMessage, "Unauthorized");
      }
    });
  });
});
