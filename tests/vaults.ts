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
      .initializeVault()
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

    // Deposit some funds
    const depositTx = await program
      .methods
      .depositFunds(new anchor.BN(50).mul(new anchor.BN(1_000_000_000)))
      .accounts({
        owner: provider.wallet.publicKey,
        vault: vaultAddress,
        token: token,
        tokenUserAc,
        tokenVaultAc
      })
      .rpc();

    console.log("Deposit sig: ", depositTx);

    await delay(2_000);

    const postDeposit = await program.account.vault.fetch(vaultAddress);
    
    const depositBalance = postDeposit.balance;

    assert(depositBalance.eq(new anchor.BN(50).mul(new anchor.BN(1_000_000_000))), "Vault balance did not update post deposit tx");

    // Withdraw some funds
    const withdrawTx = await program
    .methods
    .withdrawFunds(new anchor.BN(24).mul(new anchor.BN(1_000_000_000)))
    .accounts({
      owner: provider.wallet.publicKey,
      vault: vaultAddress,
      token: token,
      tokenUserAc,
      tokenVaultAc
    })
    .rpc();

  console.log("Withdraw sig: ", withdrawTx);
    
    await delay(2_000);

    const postWithdraw = await program.account.vault.fetch(vaultAddress);
    
    const withdrawBalance = postWithdraw.balance;
    
    assert(withdrawBalance.eq(new anchor.BN(26).mul(new anchor.BN(1_000_000_000))), "Vault balance did not update post withdraw tx");

    
  });

  it("Pay interest", async() => {

    await delay(10_000);

    const tokenPayerAc = await getAssociatedTokenAddress(
      token,
      interestPayerKeypair.publicKey
    );

    const tx = await program
      .methods
      .payInterest()
      .accounts({
        payer: interestPayerKeypair.publicKey,
        token,
        vault: vaultAddress,
        tokenPayerAc,
        tokenVaultAc
      })
      .signers([interestPayerKeypair])
      .rpc();

    console.log("Pay interest sig: ", tx);
  })
});
