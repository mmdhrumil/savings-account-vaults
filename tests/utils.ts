import * as anchor from "@project-serum/anchor";

export const VAULTS_PROGRAM_ID = new anchor.web3.PublicKey("5j3KuMK2u7KFtoEwiLTexUeooHq5NPQX96rYp5dhuze9");

export const createKeypair = async (provider: anchor.Provider) => {
  const keypair = new anchor.web3.Keypair();
  const txn = await provider.connection.requestAirdrop(
    keypair.publicKey,
    1 * anchor.web3.LAMPORTS_PER_SOL
  );
  await provider.connection.confirmTransaction(txn);
  return keypair;
};
  
export const delay = (time) => {
  return new Promise((resolve) => setTimeout(resolve, time));
};