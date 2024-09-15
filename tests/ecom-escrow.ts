import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EcomEscrow } from "../target/types/ecom_escrow";
import { assert } from "chai";
import {wallet} from "../wallet"

describe("ecom-escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const connection = provider.connection
  const admin = anchor.web3.Keypair.fromSecretKey(new Uint8Array(wallet))
  const seller = anchor.web3.Keypair.generate();
  const reciever = anchor.web3.Keypair.generate();
  const hecker = anchor.web3.Keypair.generate();
  const program = anchor.workspace.EcomEscrow as Program<EcomEscrow>;
  const globalState = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("global")], program.programId)[0]
  let orderId = "xyz"
  it("Initalize wallets and accounts", async () => {
    await connection.confirmTransaction(await connection.requestAirdrop(seller.publicKey, anchor.web3.LAMPORTS_PER_SOL))
    await connection.confirmTransaction(await connection.requestAirdrop(reciever.publicKey, anchor.web3.LAMPORTS_PER_SOL))
    await connection.confirmTransaction(await connection.requestAirdrop(admin.publicKey, anchor.web3.LAMPORTS_PER_SOL * 5))
  })
  it("can initalize global state", async() => {
    console.log(globalState.toString())
    const tx = await program.methods.initGlobalState()
    .accountsPartial({
      admin: admin.publicKey,
      globalState,
    })
    .signers([admin])
    .rpc()
    const stateData = await program.account.globalState.fetch(globalState)
    assert.ok(stateData.admin.equals(admin.publicKey))
    assert.ok(stateData.protocolFee === 0)
    console.log(tx)
    const tx2 = await program.methods.updateGlobalState(10)
    .accountsPartial({
      admin: admin.publicKey,
      globalState
    }).signers([admin]).rpc()
    const stateData2 = await program.account.globalState.fetch(globalState)
    assert.ok(stateData2.protocolFee == 10)
  })
  it("can create new orders!", async () => {
    // Add your test here.
    let amount = anchor.web3.LAMPORTS_PER_SOL * 0.05
    let rent = await connection.getMinimumBalanceForRentExemption(0)
    const tx = await program.methods
    .createOrder(orderId, new anchor.BN(amount))
    .accounts({
      user: reciever.publicKey,
      seller: seller.publicKey,
    })
    .signers([reciever])
    .rpc()

    const order = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("order"), reciever.publicKey.toBuffer(), Buffer.from(orderId)], program.programId)[0]
    const orderAccount = await program.account.order.fetch(order)
    const orderVault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderVault"), order.toBuffer()], program.programId)[0]
    const vaultBalance = await connection.getBalance(orderVault)

    assert(orderAccount.seller.equals(seller.publicKey))
    assert(orderAccount.reciever.equals(reciever.publicKey))
    assert(vaultBalance === amount + rent)
    console.log("Your transaction signature", tx);
  });
  it("can cancel order and refund amount to the reciever!", async() => {
    const order = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("order"), reciever.publicKey.toBuffer(), Buffer.from(orderId)], program.programId)[0]
    const orderState = await program.account.order.fetch(order)
    const orderVault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderVault"), order.toBuffer()], program.programId)[0]

    assert(orderState.orderId === orderId)
    const tx = await program.methods.cancelOrder(orderId)
    .accountsStrict({
      user: reciever.publicKey,
      order,
      orderVault,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([reciever])
    .rpc()
    const vaultBalance = await connection.getBalance(orderVault)
    const orderBalace = await connection.getBalance(order)
    assert(vaultBalance === 0)
    assert(orderBalace === 0)
  })
  it("cannot cancel order if accounts are not specified properly", async() => {
    try {
      const tx1 = await program.methods
      .createOrder("some other id", new anchor.BN(10))
      .accounts({
        user: reciever.publicKey,
        seller: seller.publicKey
      })
      .signers([reciever])
      .rpc()
      const order = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("order"), reciever.publicKey.toBuffer(), Buffer.from("some other id")], program.programId)[0]
      const orderVault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderVault"), order.toBuffer()], program.programId)[0]
      const tx2 = await program.methods.cancelOrder("some other id")
      .accountsStrict({
        user: hecker.publicKey,
        order,
        orderVault,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([reciever])
      .rpc()
      assert.fail("Should have thrown an error")
    } catch {
      assert.ok(true)
    }
  })
  it("can complete order and transfer amount to the seller!", async() => {
      const tx1 = await program.methods
      .createOrder("some other id2", new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * 0.05))
      .accounts({
        user: reciever.publicKey,
        seller: seller.publicKey
      })
      .signers([reciever])
      .rpc()
      console.log(tx1)
      const order = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("order"), reciever.publicKey.toBuffer(), Buffer.from("some other id2")], program.programId)[0]
      const orderVault = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("orderVault"), order.toBuffer()], program.programId)[0]
      const tx2 = await program.methods.finalizeOrder("some other id2")
      .accountsPartial({
        user: seller.publicKey,
        order,
        orderVault,
        reciever: reciever.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        admin: admin.publicKey
      })
      .signers([seller])
      .rpc()
      const expectedBalance = 10*(10/10000)
      const sellerBalance = await connection.getBalance(seller.publicKey)
      console.log("reciever = ", reciever.publicKey.toString())
      console.log("seller = ", seller.publicKey.toString())

      console.log(tx2)
  })
});
