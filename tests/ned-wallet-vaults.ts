import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program } from "@coral-xyz/anchor";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, getAccount, getOrCreateAssociatedTokenAccount, mintTo, transfer } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { assert } from "chai";
import * as shortuuid from 'short-uuid';
import { NedWalletVaults } from "../target/types/ned_wallet_vaults";


describe("ned-wallet-vaults", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  console.log('NedWalletVaults', anchor.workspace)

  const program = anchor.workspace.NedWalletVaults as Program<NedWalletVaults>;
  // const savingsProgram = anchor.workspace.NedWalletVaultsSavings as Program<NedWalletVaultsSavings>;
  const connection = anchor.getProvider().connection
  const provider = anchor.workspace.NedWalletVaults.provider

  let accountName = (Math.random() + 1).toString(36).substring(2); //'Account 1'
  let accountNameBuffer = Buffer.from(accountName)

  const VAULTS_PDA_DATA = Buffer.from("VAULTS_PDA_DATA")
  const VAULTS_PDA_ACCOUNT = Buffer.from("VAULTS_PDA_ACCOUNT")

  // const currentMint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU") // USDC
  const currentMint = new PublicKey("So11111111111111111111111111111111111111112") // SOL
  let mint = null
  let decimals = 9
  let mintAta = null

  const identifier = shortuuid.generate()
  const identifierBuffer = Buffer.from(identifier)

  let savingsVault = null

  const [dataAccount,] = PublicKey.findProgramAddressSync(
    [VAULTS_PDA_DATA, provider.publicKey.toBuffer()],
    program.programId
  );

  const [vaultAccount,] = PublicKey.findProgramAddressSync(
    [VAULTS_PDA_ACCOUNT, provider.publicKey.toBuffer(), identifierBuffer],
    program.programId
  );


  // Start Meteora
  const vaultLpMint = new PublicKey("BvoAjwEDhpLzs3jtu4H72j96ShKT5rvZE9RP1vgpfSM")
  const vaultProgram = new PublicKey("24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi")
  const vault = new PublicKey("FERjPVNEa7Udq8CEv68h6tPL46Tq7ieE49HrE2wea3XT")

  const [tokenVault,] = PublicKey.findProgramAddressSync(
    [Buffer.from("token_vault"), vault.toBuffer()],
    vaultProgram
  );
  // End Meteora

  before(async () => {

    mint = !currentMint ? await createMint(
      connection,
      provider.wallet.payer,
      provider.publicKey,
      provider.publicKey,
      decimals
    ) : currentMint

    mintAta = await getOrCreateAssociatedTokenAccount(
      connection,
      provider.wallet.payer,
      mint,
      provider.publicKey
    )

    // console.log(
    //   provider.wallet.payer,
    //   mintAta,
    //   provider.publicKey,
    //   SystemProgram.programId)

    // const tx = await closeAccount(
    //   connection,
    //   provider.wallet.payer,
    //   mintAta.address,
    //   provider.publicKey,
    //   provider.publicKey
    // )
    // console.log('tx', tx)


    if (!currentMint) {
      mintTo(
        connection,
        provider.wallet.payer,
        mint,
        mintAta.address,
        provider.publicKey,
        100 * Math.pow(10, decimals)
      )
    } else {

      // console.log('mintAta.address', mintAta.address)
      // const fundTransfer = await transfer(
      //   connection,
      //   provider.wallet.payer,
      //   provider.publicKey,
      //   mintAta.address,
      //   provider.wallet.payer,
      //   2.5 * Math.pow(10, decimals)
      // )
      // console.log("fundTransfer", provider.publicKey)
      // const tx = await createWrappedNativeAccount(
      //   connection,
      //   provider.wallet.payer,
      //   SystemProgram.programId,
      //   2.5 * Math.pow(10, 9)
      // )
      // console.log('tx', tx)


      // const transaction = new Transaction().add(
      //   SystemProgram.transfer({
      //     fromPubkey: provider.publicKey,
      //     toPubkey: mintAta.address,
      //     lamports: 2.5 * Math.pow(10, 9),
      //   }),
      // );

      // const tx = await sendAndConfirmTransaction(connection, transaction, [provider.wallet.payer]);
      // console.log('tx', tx)

    }

    // const solATA = getAssociatedTokenAddressSync(
    //   mint,
    //   provider.publicKey,
    //   false,
    //   TOKEN_PROGRAM_ID,
    //   ASSOCIATED_TOKEN_PROGRAM_ID
    // );

    // const solATAAccount = await getAccount(
    //   connection,
    //   solATA
    // )

    // console.log('solATAAccount', solATAAccount)

    // if (!solATAAccount.isInitialized) {
    //   createWrappedNativeAccount(
    //     connection,
    //     provider.wallet.payer,
    //     provider.publicKey,
    //     1 * Math.pow(10, 9)
    //   )
    // } else {

    // }



  })

  // console.log("dataAccount", dataAccount.toBase58())
  // console.log("vaultAccount", vaultAccount.toBase58())



  it("Create a new account", async () => {

    try {

      const tx = await program.methods
        .createVault(accountNameBuffer, identifierBuffer, { none: {} })
        .accounts({
          owner: provider.publicKey,
          dataAccount,
          vaultAccount,
          mint,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        })
        .signers([provider.wallet.payer])
        .rpc();

      console.log("Your transaction signature", tx);

      const account = await program.account.vaultManager.fetch(
        dataAccount
      );

      savingsVault = account.accounts.find(x => {
        const nameBufferSlice = Buffer.from(x.name.slice(0, accountNameBuffer.length))
        return nameBufferSlice.toString() == accountName
      })
      console.log('create savingsVault.pubKey ==> ', savingsVault.pubKey.toBase58())

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

  it("deposit money to fund", async () => {

    try {

      const amount = 0.2 * Math.pow(10, decimals)

      const transferTx = await transfer(
        connection,
        provider.wallet.payer,
        mintAta.address,
        savingsVault.pubKey,
        provider.publicKey,
        amount
      )


      const firstAccountFetched = await getAccount(connection, savingsVault.pubKey)
      console.log('amount-->', firstAccountFetched.amount)

      assert.isTrue(Number(firstAccountFetched.amount) == amount);
    } catch (error) {
      console.log('error', error)
    }

  })

  it("deposit liquidity from savings", async () => {
    try {

      const userLpToken = await getOrCreateAssociatedTokenAccount(
        connection,
        provider.wallet.payer,
        vaultLpMint,
        dataAccount,
        true
      )

      console.log('userToken', userLpToken.address)

      const tx = await program.methods
        .investOnSavings(identifierBuffer, new anchor.BN(0.2 * Math.pow(10, decimals)))
        .accounts({
          owner: provider.publicKey,
          dataAccount,
          vaultAccount: savingsVault.pubKey,
          mint,
          vaultProgram,
          vault,
          tokenVault,
          vaultLpMint,
          user: dataAccount,
          userToken: savingsVault.pubKey,
          userLp: userLpToken.address,
          tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([provider.wallet.payer])
        .rpc();

      console.log("Your deposit liquidity tx =", tx);

    } catch (_error: any) {
      console.log(_error)
      assert.fail("Unexpected error type, console.log _error variable")

    }
  })

  it("withdraw liquidity to savings", async () => {
    try {

      const userLpToken = await getOrCreateAssociatedTokenAccount(
        connection,
        provider.wallet.payer,
        vaultLpMint,
        dataAccount,
        true
      )

      console.log('userToken', userLpToken.address, userLpToken.amount)

      const tx = await program.methods
        .withdrawSavings(identifierBuffer, new anchor.BN(userLpToken.amount))
        .accounts({
          owner: provider.publicKey,
          dataAccount,
          vaultAccount: savingsVault.pubKey,
          mint,
          vaultProgram,
          vault,
          tokenVault,
          vaultLpMint,
          user: dataAccount,
          userToken: savingsVault.pubKey,
          userLp: userLpToken.address,
          tokenProgram: TOKEN_PROGRAM_ID
        })
        .signers([provider.wallet.payer])
        .rpc();

      console.log("Your withdraw liquidity tx =", tx);

    } catch (_error: any) {
      console.log(_error)
      assert.fail("Unexpected error type, console.log _error variable")

    }
  })

  xit("Get number of available accounts", async () => {


    const account = await program.account.vaultManager.fetch(
      dataAccount
    );

    const availableSpots = account.accounts
      .map(x => x.nameLength == 0)
      .filter(x => x == true)
      .length

    assert.isTrue(availableSpots >= 0);
    assert.isTrue(availableSpots <= 20); // Only 20 accounts max are allowed, check program

  })

  xit("Update account vault", async () => {
    try {

      accountName = "New account" + (Math.random() + 1).toString(36).substring(2)
      accountNameBuffer = Buffer.from(accountName)

      const tx = await program.methods
        .updateVault(identifierBuffer, accountNameBuffer, { spare: {} })
        .accounts({
          owner: provider.publicKey,
          dataAccount,
          vaultAccount: savingsVault.pubKey,
          mint,
        })
        .signers([provider.wallet.payer])
        .rpc();

      console.log("Your transaction signature", tx);

      const account = await program.account.vaultManager.fetch(
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

  xit("Delete account vault", async () => {

    console.log('delete savingsVault.pubKey ==> ', savingsVault.pubKey.toBase58())

    const tx = await program.methods
      .deleteVault(identifierBuffer)
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

    const account = await program.account.vaultManager.fetch(
      dataAccount
    );

    savingsVault = account.accounts.find(x => {
      const nameBufferSlice = Buffer.from(x.name.slice(0, accountNameBuffer.length))
      return nameBufferSlice.toString() == accountName
    })

    assert.isTrue(savingsVault == undefined)

  });

  it("Delete all account vault", async () => {

    // console.log('delete savingsVault.pubKey ==> ', savingsVault.pubKey.toBase58())

    try {

      const { accounts } = await program.account.vaultManager.fetch(
        dataAccount
      );

      const accountsToDelete = (accounts as any[])
        .filter(x => {
          return x.nameLength > 0
        })

      // .forEach(async x => {
      for (let index = 0; index < accountsToDelete.length; index++) {
        const account = accountsToDelete[index];

        // console.log('account-->', account.pubKey, account.tokenPubKey)

        const tx = await program.methods
          .deleteVault(account.identifier)
          .accounts({
            owner: provider.publicKey,
            dataAccount,
            vaultAccount: account.pubKey,
            mint: account.tokenPubKey,
            userTokenAccount: mintAta.address
          })
          .signers([provider.wallet.payer])
          .rpc();

        // console.log("Your transaction signature", tx);

      }



      const updatedAccount = await program.account.vaultManager.fetch(
        dataAccount
      );

      const unDeletedAccounts = updatedAccount.accounts.filter(x => {
        return x.nameLength > 0
        // const nameBufferSlice = Buffer.from(x.name.slice(0, accountNameBuffer.length))
        // return nameBufferSlice.toString() == accountName
      })

      assert.isTrue(unDeletedAccounts.length == 0)
    } catch (error) {
      console.log(error)
    }

  });

});
