import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorError } from "@coral-xyz/anchor";
import { NedWalletProgram } from "../target/types/ned_wallet_program";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import * as shortuuid from 'short-uuid'


describe("ned-wallet-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NedWalletProgram as Program<NedWalletProgram>;
  const connection = anchor.getProvider().connection
  const provider = anchor.workspace.NedWalletProgram.provider

  // const currentMint = new PublicKey("AmBD9hM7DwztwLiXgU3zVG7cdnwXxfoecYdCmFey2JNS")
  const currentMint = null

  let accountName = (Math.random() + 1).toString(36).substring(2); //'Account 1'
  let accountNameBuffer = Buffer.from(accountName)

  const SAVINGS_PDA_SEED = Buffer.from("SAVINGS_PDA")

  let savingsVault = null

  let mint = null

  const [dataAccount,] = PublicKey.findProgramAddressSync(
    [SAVINGS_PDA_SEED, provider.publicKey.toBuffer()],
    program.programId
  );

  const identifier = shortuuid.generate()
  const identifierBuffer = Buffer.from(identifier)

  it("Create a new account", async () => {

    mint = !currentMint ? await createMint(
      connection,
      provider.wallet.payer,
      provider.publicKey,
      provider.publicKey,
      6
    ) : currentMint

    try {

      const [vaultAccount,] = PublicKey.findProgramAddressSync(
        [SAVINGS_PDA_SEED, provider.publicKey.toBuffer(), identifierBuffer],
        program.programId
      );

      const tx = await program.methods
        .createSavingsVault(accountNameBuffer, identifierBuffer, { none: {} })
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

      const account = await program.account.userSavingsManager.fetch(
        dataAccount
      );

      savingsVault = account.accounts.find(x => {
        const nameBufferSlice = Buffer.from(x.name.slice(0, accountNameBuffer.length))
        return nameBufferSlice.toString() == accountName
      })

      assert.isTrue(savingsVault != undefined)
      assert.strictEqual(savingsVault.nameLength, accountName.length);
      assert.isTrue(Buffer.from(savingsVault.name.slice(0, savingsVault.nameLength)).toString() == accountName)

    } catch (_error: any) {
      console.log('Create account error =>', _error)
      if (_error instanceof AnchorError) {

        const alreadyInitializedMsg = 'This account was already initialized'

        assert.isTrue(_error instanceof AnchorError);
        assert.strictEqual(_error.error.errorMessage, alreadyInitializedMsg);
        assert.strictEqual(_error.error.errorCode.code, 'AlreadyInitialized');
        assert.strictEqual(_error.error.errorCode.number, 6000);
        assert.strictEqual(
          _error.program.toString(),
          program.programId.toString()
        );

        assert.fail("Failed to create a new savings account, error received was correct but not expected in the test. Reset test validator and try again.")
      } else {
        assert.fail("Unexpected error type, console.log _error variable")
      }
    }

  });

  it("Get number of available accounts", async () => {


    const account = await program.account.userSavingsManager.fetch(
      dataAccount
    );

    const availableSpots = account.accounts
      .map(x => x.nameLength == 0)
      .filter(x => x == true)
      .length

    assert.isTrue(availableSpots >= 0);
    assert.isTrue(availableSpots <= 20); // Only 20 accounts max are allowed, check program

  })


  it("Update account vault", async () => {
    try {

      accountName = "New account"
      accountNameBuffer = Buffer.from(accountName)

      const tx = await program.methods
        .updateSavingsVault(savingsVault.identifier, accountNameBuffer, { none: {} })
        .accounts({
          owner: provider.publicKey,
          dataAccount,
          vaultAccount: savingsVault.pubKey,
          mint,
        })
        .signers([provider.wallet.payer])
        .rpc();

      console.log("Your transaction signature", tx);

      const account = await program.account.userSavingsManager.fetch(
        dataAccount
      );

      savingsVault = account.accounts.find(x => {
        const nameBufferSlice = Buffer.from(x.name.slice(0, accountNameBuffer.length))
        return nameBufferSlice.toString() == accountName
      })

      assert.isTrue(savingsVault != undefined)
      assert.strictEqual(savingsVault.nameLength, accountName.length);
      assert.isTrue(Buffer.from(savingsVault.name.slice(0, savingsVault.nameLength)).toString() == accountName)

    } catch (_error: any) {
      console.log(_error)
      assert.fail("Unexpected error type, console.log _error variable")

    }

  })

  it("Delete account vault", async () => {

    const mintAta = await getOrCreateAssociatedTokenAccount(
      connection,
      provider.wallet.payer,
      mint,
      provider.publicKey
    )

    const tx = await program.methods
      .deleteSavingsVault(savingsVault.identifier)
      .accounts({
        owner: provider.publicKey,
        dataAccount,
        vaultAccount: savingsVault.pubKey,
        mint,
        userTokenAccount: mintAta.address
      })
      .signers([provider.wallet.payer])
      .rpc();

    console.log("Your transaction signature", tx);

    const account = await program.account.userSavingsManager.fetch(
      dataAccount
    );

    savingsVault = account.accounts.find(x => {
      const nameBufferSlice = Buffer.from(x.name.slice(0, accountNameBuffer.length))
      return nameBufferSlice.toString() == accountName
    })

    assert.isTrue(savingsVault == undefined)

  });

});
