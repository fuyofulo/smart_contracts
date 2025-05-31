import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { expect } from "chai";
import { Anchor } from "../target/types/anchor";

import * as borsh from "borsh";

// Native contract's Borsh schema
class Data {
  value: number;
  constructor(fields: { value: number }) {
    this.value = fields.value;
  }
}

const DataSchema = new Map([
  [Data, { kind: "struct", fields: [["value", "u32"]] }]
]);

describe("anchor-to-native-cpi", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Anchor as Program<Anchor>;

  const nativeProgramId = new PublicKey(
    "57CdWVWHsoi9UHR3DYK2g5ggZhJDZRzbMet7ZQWw2ZQM"
  );

  const signer = provider.wallet.publicKey;
  let dataAccount: Keypair;

  it("init -> double -> half", async () => {
    dataAccount = Keypair.generate();

    // ✅ init: this will create and initialize the data account
    await program.methods
      .init(42)
      .accounts({
        dataAccount: dataAccount.publicKey,
        signer,
        nativeProgram: nativeProgramId
      })
      .signers([dataAccount])
      .rpc();

    let accountInfo = await provider.connection.getAccountInfo(dataAccount.publicKey);
    let data = borsh.deserialize(DataSchema, Data, accountInfo!.data);
    expect(data.value).to.equal(42);

    // ✅ double: should change 42 → 84
    await program.methods
      .double()
      .accounts({
        dataAccount: dataAccount.publicKey,
        signer,
        nativeProgram: nativeProgramId,
      })
      .rpc();

    accountInfo = await provider.connection.getAccountInfo(dataAccount.publicKey);
    data = borsh.deserialize(DataSchema, Data, accountInfo!.data);
    expect(data.value).to.equal(84);

    // ✅ half: should change 84 → 42
    await program.methods
      .half()
      .accounts({
        dataAccount: dataAccount.publicKey,
        signer,
        nativeProgram: nativeProgramId,
      })
      .rpc();

    accountInfo = await provider.connection.getAccountInfo(dataAccount.publicKey);
    data = borsh.deserialize(DataSchema, Data, accountInfo!.data);
    expect(data.value).to.equal(42);
  });
});
