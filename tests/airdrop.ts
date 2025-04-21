import * as anchor from "@coral-xyz/anchor";

export async function airdrop(
  provider: anchor.Provider,
  address: anchor.web3.PublicKey,
  amount = 2
) {
  const signature = await provider.connection.requestAirdrop(
    address,
    amount * anchor.web3.LAMPORTS_PER_SOL
  );

  await provider.connection.confirmTransaction(signature);
}
