import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Vaults } from "../target/types/vaults";
import { VAULTS_PROGRAM_ID, createKeypair, delay } from "./utils";
import { initializeMints } from "./token-utils";
import { getAssociatedTokenAddress } from "@solana/spl-token-latest";
import { assert } from "chai";

describe("vaults", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vaults as Program<Vaults>;

  const provider = anchor.getProvider() as anchor.AnchorProvider;

  let vaultAddress: anchor.web3.PublicKey;
  let vaultKey: anchor.web3.PublicKey;
  let token: anchor.web3.PublicKey;
  let tokenVaultAc: anchor.web3.PublicKey;
  let interestPayerKeypair: anchor.web3.Keypair;
  let depositReceipt: anchor.web3.PublicKey;

  it("Initialize vaults", async () => {
    // Add your test here
    const vaultKeypair = anchor.web3.Keypair.generate();
    vaultKey = vaultKeypair.publicKey;

    interestPayerKeypair = await createKeypair(provider);

    const res = await initializeMints(
      provider,
      1,
      [9],
      [provider.wallet.publicKey, interestPayerKeypair.publicKey],
      [
        new anchor.BN(100).mul(new anchor.BN(1_000_000_000)),
        new anchor.BN(100).mul(new anchor.BN(1_000_000_000))
      ]
    );

    token = res.tokens[0];

    const tokenVaultAcKeypair = anchor.web3.Keypair.generate();
    tokenVaultAc = tokenVaultAcKeypair.publicKey;

    const [vaultAddr, ] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        provider.wallet.publicKey.toBuffer(),
        token.toBuffer(),
        vaultKeypair.publicKey.toBuffer()
      ],
      VAULTS_PROGRAM_ID
    );
    vaultAddress = vaultAddr;

    const tx = await program
      .methods
      .initializeVault(new anchor.BN(10))
      .accounts({
        owner: provider.wallet.publicKey,
        token: token,
        vaultKey: vaultKey,
        tokenVaultAc: tokenVaultAc,
        vault: vaultAddress
    })
    .signers([tokenVaultAcKeypair])
    .rpc();

    console.log("vault created: ", tx);

  });

  it("Deposit and withdraw funds", async () => {

    let tokenUserAc = await getAssociatedTokenAddress(
      token,
      provider.wallet.publicKey,
      false
    );

    let [depositAddr, ] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        provider.wallet.publicKey.toBuffer(),
        vaultAddress.toBuffer()
      ],
      VAULTS_PROGRAM_ID
    );
    depositReceipt = depositAddr;
    
    // Deposit some funds
    const depositTx = await program
      .methods
      .depositFunds(new anchor.BN(50).mul(new anchor.BN(1_000_000_000)))
      .accounts({
        owner: provider.wallet.publicKey,
        vault: vaultAddress,
        depositReceipt,
        token: token,
        tokenUserAc,
        tokenVaultAc
      })
      .rpc();

    console.log("Deposit sig: ", depositTx);
    
  });

  it("Top up interest", async() => {

    await delay(10_000);

    let tokenPayerAc = await getAssociatedTokenAddress(
      token,
      provider.wallet.publicKey,
      false
    );

    const tx = await program
      .methods
      .topupInterest(new anchor.BN(10).mul(new anchor.BN(1_000_000_000)))
      .accounts({
        owner: provider.wallet.publicKey,
        token,
        vault: vaultAddress,
        tokenPayerAc,
        tokenVaultAc
      })
      .rpc();

    console.log("Top up interest sig: ", tx);
  });

  it("Withdraw from vault", async() => {

    await delay(2_000);

    let tokenUserAc = await getAssociatedTokenAddress(
      token,
      provider.wallet.publicKey,
      false
    );

    // Withdraw some funds
    const withdrawTx = await program
    .methods
    .withdrawFunds()
    .accounts({
      owner: provider.wallet.publicKey,
      vault: vaultAddress,
      token: token,
      depositReceipt,
      tokenUserAc,
      tokenVaultAc
    })
    .rpc();

    console.log("Withdraw sig: ", withdrawTx);
  })
});
