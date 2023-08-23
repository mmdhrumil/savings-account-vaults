import * as anchor from "@project-serum/anchor";
import {
  createMint,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress,
  mintToChecked,
} from "@solana/spl-token-latest";
import { createKeypair } from "./utils";

export const initializeMints = async (
  provider: anchor.Provider,
  numMints: number,
  decimals: number[],
  recipients?: anchor.web3.PublicKey[],
  amount?: anchor.BN[]
) => {
  let mints: anchor.web3.PublicKey[] = [];
  let decimalsRes = [];

  let keypair = await createKeypair(provider);

  let i = 0;
  for (let _ of Array(numMints)) {
    const tokenMint = await createMint(
      provider.connection,
      keypair,
      keypair.publicKey,
      keypair.publicKey,
      decimals[i]
    );

    if (recipients && recipients.length > 0) {
      for (let recipient of recipients) {
        if (!amount[i]) {
          amount[i] = new anchor.BN(1000).mul(new anchor.BN(10 ** decimals[i]));
        }

        await createAssociatedTokenAccount(
          provider.connection,
          keypair,
          tokenMint,
          recipient
        );

        const ataAddr = await getAssociatedTokenAddress(tokenMint, recipient);

        await mintToChecked(
          provider.connection,
          keypair,
          tokenMint,
          ataAddr,
          keypair,
          //@ts-ignore
          amount[i],
          decimals[i]
        );
      }
    }

    mints.push(tokenMint);
    decimalsRes.push(decimals[i]);
    i++;
  }
  return {
    tokens: mints,
    decimals: decimalsRes,
  };
};
