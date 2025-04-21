import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert, expect } from "chai";
import { Divi } from "../target/types/divi";
import { airdrop } from "./airdrop";
import { VAULT, VAULT_AUTHORITY } from "./constants";

describe("divi", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Divi as Program<Divi>;
  const paymentId = Math.floor(Math.random() * 100000);
  const amount = new anchor.BN(10 * anchor.web3.LAMPORTS_PER_SOL);
  const issuer = anchor.web3.Keypair.generate();
  const customerA = anchor.web3.Keypair.generate();
  const customerB = anchor.web3.Keypair.generate();
  const customerC = anchor.web3.Keypair.generate();
  const paymentIdBuffer = new anchor.BN(paymentId).toArrayLike(Buffer, "le", 4);

  let actualAmount = 0;

  const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(VAULT),
      issuer.publicKey.toBuffer(),
      paymentIdBuffer,
    ],
    program.programId
  );

  const [vaultAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(VAULT_AUTHORITY),
      issuer.publicKey.toBuffer(),
      paymentIdBuffer,
    ],
    program.programId
  );

  before(async () => {
    await airdrop(provider, issuer.publicKey, 100);
    await airdrop(provider, customerA.publicKey, 100);
    await airdrop(provider, customerB.publicKey, 100);
    await airdrop(provider, customerC.publicKey, 100);
  });

  it("Create the vault", async () => {
    const tx = await program.methods
      .initializeVault(paymentId, amount)
      .accountsStrict({
        issuer: issuer.publicKey,
        vault,
        vaultAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([issuer])
      .rpc();

    console.log(tx);

    const vaultAccount = await program.account.paymentVault.fetch(vault);

    expect(vaultAccount.issuer.toBase58()).equal(issuer.publicKey.toBase58());
    expect(vaultAccount.totalAmount.toNumber()).equal(amount.toNumber());
    expect(vaultAccount.isFinalized).to.be.false;
    expect(vaultAccount.paymentId).equal(paymentId);
  });

  it("Customer A try to deposit an amount which is greater than the vault total amount", async () => {
    try {
      const tx = await program.methods
        .pay(paymentId, new anchor.BN(15))
        .accountsStrict({
          payer: customerA.publicKey,
          issuer: issuer.publicKey,
          vault,
          vaultAuthority,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([customerA])
        .rpc();

      console.log(tx);
    } catch (err) {
      if (err instanceof anchor.AnchorError) {
        assert.strictEqual(
          err.error.errorCode.code,
          "AmountIsGreaterThanVaultTotalAmount"
        );
      } else {
        console.error(err);
      }
    }
  });

  it("Issuer share the vault, customer A pay", async () => {
    actualAmount += 4;

    const tx = await program.methods
      .pay(paymentId, new anchor.BN(4))
      .accountsStrict({
        payer: customerA.publicKey,
        issuer: issuer.publicKey,
        vault,
        vaultAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerA])
      .rpc();
    console.log(tx);

    const balance = await program.provider.connection.getBalance(
      vaultAuthority
    );

    expect(balance).equal(actualAmount * anchor.web3.LAMPORTS_PER_SOL);
  });

  it("Issuer share the vault, customer B pay", async () => {
    actualAmount += 4;

    const tx = await program.methods
      .pay(paymentId, new anchor.BN(4))
      .accountsStrict({
        payer: customerB.publicKey,
        issuer: issuer.publicKey,
        vault,
        vaultAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerB])
      .rpc();
    console.log(tx);

    const balance = await program.provider.connection.getBalance(
      vaultAuthority
    );

    expect(balance).equal(actualAmount * anchor.web3.LAMPORTS_PER_SOL);
  });

  it("Issuer try to close the vault, but not finalized for the moment", async () => {
    try {
      const tx = await program.methods
        .closeVault(paymentId)
        .accountsStrict({
          issuer: issuer.publicKey,
          vault,
          vaultAuthority,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([issuer])
        .rpc();
      console.log(tx);
    } catch (err) {
      if (err instanceof anchor.AnchorError) {
        assert.strictEqual(err.error.errorCode.code, "VaultIsNotFinalized");
      } else {
        console.error(err);
      }
    }
  });

  it("Issuer share the vault, customer C pay", async () => {
    actualAmount += 2;

    const tx = await program.methods
      .pay(paymentId, new anchor.BN(2))
      .accountsStrict({
        payer: customerC.publicKey,
        issuer: issuer.publicKey,
        vault,
        vaultAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerC])
      .rpc();
    console.log(tx);

    const balance = await program.provider.connection.getBalance(
      vaultAuthority
    );

    expect(balance).equal(actualAmount * anchor.web3.LAMPORTS_PER_SOL);
  });

  it("Issuer share the vault, customer C pay again, but the program doesn't works", async () => {
    try {
      const tx = await program.methods
        .pay(paymentId, new anchor.BN(2))
        .accountsStrict({
          payer: customerC.publicKey,
          issuer: issuer.publicKey,
          vault,
          vaultAuthority,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([customerC])
        .rpc();
      console.log(tx);
    } catch (err) {
      if (err instanceof anchor.AnchorError) {
        assert.strictEqual(err.error.errorCode.code, "VaultIsAlreadyFinalized");
      } else {
        console.error(err);
      }
    }
  });

  it("Vault is finalized", async () => {
    const vaultAccount = await program.account.paymentVault.fetch(vault);

    expect(vaultAccount.isFinalized).to.be.true;
  });

  it("Issuer close the vault", async () => {
    const tx = await program.methods
      .closeVault(paymentId)
      .accountsStrict({
        issuer: issuer.publicKey,
        vault,
        vaultAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([issuer])
      .rpc();
    console.log(tx);

    const issuerBalance = await program.provider.connection.getBalance(
      issuer.publicKey
    );

    expect(issuerBalance > 1000);
  });
});
