import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NedWalletProgram } from "../target/types/ned_wallet_program";
import { PublicKey, Keypair } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync } from "@solana/spl-token";

describe("ned-wallet-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NedWalletProgram as Program<NedWalletProgram>;
  const connection = anchor.getProvider().connection
  const provider = anchor.workspace.NedWalletProgram.provider

  // const currentMint = new PublicKey("AmBD9hM7DwztwLiXgU3zVG7cdnwXxfoecYdCmFey2JNS")
  const currentMint = null

  const accountName = (Math.random() + 1).toString(36).substring(2); //'Account 1'
  const accountNameBuffer = Buffer.from(accountName)

  const SAVINGS_PDA_SEED = Buffer.from("SAVINGS_PDA")

  it("Create a new account", async () => {
    // Add your test here.
    console.log('provider', provider)

    const mint = !currentMint ? await createMint(
      connection,
      provider.wallet.payer,
      provider.publicKey,
      provider.publicKey,
      6
    ) : currentMint

    const [dataAccount,] = PublicKey.findProgramAddressSync(
      [SAVINGS_PDA_SEED, provider.publicKey.toBuffer()],
      program.programId
    );

    const [vaultAccount,] = PublicKey.findProgramAddressSync(
      [SAVINGS_PDA_SEED, provider.publicKey.toBuffer(), accountNameBuffer],
      program.programId
    );

    console.log('mint', mint)

    const tx = await program.methods
      .createSavingsVault(accountNameBuffer, { manual: {} })
      .accounts({
        owner: provider.publicKey,
        dataAccount,
        vaultAccount: vaultAccount,
        mint,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([provider.wallet.payer])
      .rpc();

    console.log("Your transaction signature", tx);

  });
});
