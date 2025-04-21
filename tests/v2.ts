import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert, expect } from "chai";
import { Divi } from "../target/types/divi";
import { airdrop } from "./airdrop";
import {
  PARTICIPANT_VAULT,
  PARTICIPANT_VAULT_AUTHORITY,
  VAULT,
  VAULT_AUTHORITY,
} from "./constants";

describe("Divi v2", () => {
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

  it("Create payment with 10 SOL", async () => {
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

    expect(vaultAccount.issuer.equals(issuer.publicKey)).to.be.true;
    expect(vaultAccount.totalAmount.eq(amount)).to.be.true;
    expect(vaultAccount.isFinalized).to.be.false;
    expect(vaultAccount.paymentId).equal(paymentId);
  });

  it("Customer A participate with 4 SOL", async () => {
    const pdas = getParticipantPdas(
      customerA.publicKey,
      paymentId,
      program.programId
    );

    const participationAmount = new anchor.BN(4 * anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .participate(paymentId, participationAmount)
      .accountsStrict({
        participant: customerA.publicKey,
        vault,
        participantVault: pdas.vault,
        participantVaultAuthority: pdas.vaultAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerA])
      .rpc();

    console.log(tx);

    const participant = await program.account.participantVault.fetch(
      pdas.vault
    );

    expect(participant.amount.eq(participationAmount)).to.be.true;
    expect(participant.participant.equals(customerA.publicKey)).to.be.true;
    expect(participant.paymentId).equal(paymentId);
    expect(participant.issuer.equals(issuer.publicKey)).to.be.true;
  });

  it("Customer B participate with 4 SOL", async () => {
    const pdas = getParticipantPdas(
      customerB.publicKey,
      paymentId,
      program.programId
    );

    const participationAmount = new anchor.BN(4 * anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .participate(paymentId, participationAmount)
      .accountsStrict({
        participant: customerB.publicKey,
        vault,
        participantVault: pdas.vault,
        participantVaultAuthority: pdas.vaultAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerB])
      .rpc();

    console.log(tx);

    const participant = await program.account.participantVault.fetch(
      pdas.vault
    );

    expect(participant.amount.eq(participationAmount)).to.be.true;
    expect(participant.participant.equals(customerB.publicKey)).to.be.true;
    expect(participant.paymentId).equal(paymentId);
    expect(participant.issuer.equals(issuer.publicKey)).to.be.true;
  });

  it("Customer C participate with 2 SOL", async () => {
    const pdas = getParticipantPdas(
      customerC.publicKey,
      paymentId,
      program.programId
    );

    const participationAmount = new anchor.BN(2 * anchor.web3.LAMPORTS_PER_SOL);

    const tx = await program.methods
      .participate(paymentId, participationAmount)
      .accountsStrict({
        participant: customerC.publicKey,
        vault,
        participantVault: pdas.vault,
        participantVaultAuthority: pdas.vaultAuthority,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([customerC])
      .rpc();

    console.log(tx);

    const participant = await program.account.participantVault.fetch(
      pdas.vault
    );

    expect(participant.amount.eq(participationAmount)).to.be.true;
    expect(participant.participant.equals(customerC.publicKey)).to.be.true;
    expect(participant.paymentId).equal(paymentId);
    expect(participant.issuer.equals(issuer.publicKey)).to.be.true;
  });

  it("Issuer cancel the payment, but failed because participant are not refunded", async () => {
    try {
      const tx = await program.methods
        .cancelPayment(paymentId)
        .accountsStrict({
          vault,
          vaultAuthority,
          issuer: issuer.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .remainingAccounts(
          (
            await program.account.participantVault.all()
          ).map((p) => ({
            pubkey: p.publicKey,
            isWritable: true,
            isSigner: false,
          }))
        )
        .signers([issuer])
        .rpc();

      console.log(tx);
    } catch (err) {
      if (err instanceof anchor.AnchorError) {
        assert.strictEqual(
          err.error.errorCode.code,
          "NotAllParticipantsRefunded"
        );
      } else {
        console.error(err);
      }
    }
  });

  it("Refund all participant vault", async () => {
    const participants = await program.account.participantVault.all([
      {
        memcmp: {
          offset: 8,
          bytes: issuer.publicKey.toBase58(),
        },
      },
    ]);

    expect(participants.length).to.equal(3);

    for await (const participant of participants) {
      const pdas = getParticipantPdas(
        participant.account.participant,
        paymentId,
        program.programId
      );

      const tx = await program.methods
        .refundParticipant(paymentId)
        .accountsStrict({
          vault,
          participantVaultAuthority: pdas.vaultAuthority,
          participantVault: pdas.vault,
          issuer: issuer.publicKey,
          participant: participant.account.participant,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([issuer])
        .rpc();

      console.log(tx);
    }

    const pts = await program.account.participantVault.all();

    expect(pts.length).eq(0);
  });

  it("Issuer close the payment", async () => {
    const tx = await program.methods
      .closePaymentVault(paymentId)
      .accountsStrict({
        vault,
        issuer: issuer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .remainingAccounts(
        (
          await program.account.participantVault.all()
        ).map((p) => ({
          pubkey: p.publicKey,
          isWritable: true,
          isSigner: false,
        }))
      )
      .signers([issuer])
      .rpc();

    console.log(tx);
  });
});

function getParticipantPdas(
  participant: anchor.web3.PublicKey,
  paymentId: number,
  programId: anchor.web3.PublicKey
) {
  const paymentIdBuffer = new anchor.BN(paymentId).toArrayLike(Buffer, "le", 4);

  const [vault] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(PARTICIPANT_VAULT),
      participant.toBuffer(),
      paymentIdBuffer,
    ],
    programId
  );

  const [vaultAuthority] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(PARTICIPANT_VAULT_AUTHORITY),
      participant.toBuffer(),
      paymentIdBuffer,
    ],
    programId
  );

  return {
    vault,
    vaultAuthority,
  };
}
