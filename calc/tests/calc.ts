import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calc } from "../target/types/calc";
import { assert } from "chai";

describe("calc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.calc as Program<Calc>;
  const newAccount = anchor.web3.Keypair.generate();

  it('initialize', async () => {
    const number = 10;
    const tx = await program.methods.init(number).accounts({
      account: newAccount.publicKey,
      signer: anchor.getProvider().publicKey,
    }).signers([newAccount]).rpc();

    console.log('your transactiopn signature: ', tx);

  })


  it("add!", async () => {

    const number = 4;

    const tx = await program.methods.add(number).accounts({
      account: newAccount.publicKey,
      signer: anchor.getProvider().wallet.publicKey
    }).rpc();

    console.log("your transaction signature: ", tx);

    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.equal(account.num, 14);
  });

  it("subtract", async () => {
    const number = 2;

    const tx = await program.methods.sub(number).accounts({
      account: newAccount.publicKey,
      signer: anchor.getProvider().publicKey
    }).rpc();

    console.log("your transaction signature: ", tx);

    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.equal(account.num, 12);
  })

  it("multiply", async () => {
    const number = 5;

    const tx = await program.methods.multiply(number).accounts({
      account: newAccount.publicKey,
      signer: anchor.getProvider().publicKey
    }).rpc();

    console.log("your transaction signature: ", tx);

    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.equal(account.num, 60);
  })

  it("half", async () => {
    const tx = await program.methods.half().accounts({
      account: newAccount.publicKey,
      signer: anchor.getProvider().publicKey
    }).rpc();

    console.log("your transaction signature: ", tx);

    const account = await program.account.dataShape.fetch(newAccount.publicKey);
    assert.equal(account.num, 30);
  })

});
