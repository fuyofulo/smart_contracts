import { test, expect } from 'bun:test';
import { LiteSVM } from 'litesvm';
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js'

let svm = new LiteSVM();

test('creating a pda', async () => {

    let user = new Keypair();
    svm.airdrop(user.publicKey, BigInt(2*LAMPORTS_PER_SOL));

    let contractPubkey = PublicKey.unique();
    svm.addProgramFromFile(contractPubkey, '/home/fuyofulo/smart_contracts/pda/target/deploy/pda.so');

    const [pda, bump] = PublicKey.findProgramAddressSync([user.publicKey.toBuffer(), Buffer.from("user")], contractPubkey);
    console.log(pda.toString());
    console.log(bump.toString());

    let ix = [new TransactionInstruction({
        keys:[
            { pubkey: user.publicKey, isSigner: true, isWritable: true },
            { pubkey: pda, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
        ],
        programId: contractPubkey,
        data: Buffer.from("")
    })];

    let blockhash = svm.latestBlockhash();

    const txn = new Transaction;
    txn.recentBlockhash = blockhash;
    txn.add(...ix);
    txn.sign(user);
    await svm.sendTransaction(txn);
    svm.expireBlockhash();

    let pda_balance = await svm.getBalance(pda);
    console.log(pda_balance);
    expect(Number(pda_balance)).toBeGreaterThan(0);
})