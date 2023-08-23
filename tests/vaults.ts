import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Vaults } from "../target/types/vaults";
import { VAULTS_PROGRAM_ID } from "./utils";
import { initializeMints } from "./token-utils";

describe("vaults", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vaults as Program<Vaults>;

  const provider = anchor.getProvider() as anchor.AnchorProvider;

  it("Initialize vaults", async () => {
    // Add your test here
    const vaultKeypair = anchor.web3.Keypair.generate();

    const res = await initializeMints(
      provider,
      1,
      [9],
      [provider.wallet.publicKey],
      [new anchor.BN(1_000_000_000)]
    );

    const token = res.tokens[0];

    const [vaultAddr, ] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        provider.wallet.publicKey.toBuffer(),
        token.toBuffer(),
        vaultKeypair.publicKey.toBuffer()
      ],
      VAULTS_PROGRAM_ID
    );

    const tx = await program
      .methods
      .initializeVault()
      .accounts({
        owner: provider.wallet.publicKey,
        token: token,
        vaultKey: vaultKeypair.publicKey,
        vault: vaultAddr
    })
    .rpc();

    console.log("vault created: ", tx);

  });
});
