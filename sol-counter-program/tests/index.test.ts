import { expect, test } from 'bun:test';
import { Keypair, Connection, LAMPORTS_PER_SOL, SystemProgram, PublicKey, Transaction } from '@solana/web3.js';
import { COUNTER_SIZE } from './types';
import { schema } from './types';
import * as borsh from 'borsh';

// creating new accounts
let adminAccount = Keypair.generate();
let dataAccount = Keypair.generate();

const PROGRAM_ID = new PublicKey("Ga2weqC9AmcRpqEja9Nzj4XrWuTAYHKG6EaTTLWcbtUB");

test('account is initialized', async() => {

    // connecting with the local blockchain
    const connection = await new Connection("http://127.0.0.1:8899");

    // airdropping some solana to the new account
    const txn = await connection.requestAirdrop(adminAccount.publicKey, 10 * LAMPORTS_PER_SOL);
    await connection.confirmTransaction(txn);

    const adminAccountInfo = await connection.getAccountInfo(adminAccount.publicKey);
    console.log(adminAccountInfo);

    const lamports = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

    const ix = await SystemProgram.createAccount({
        fromPubkey: adminAccount.publicKey,
        lamports,
        space: COUNTER_SIZE,
        programId: PROGRAM_ID,
        newAccountPubkey: dataAccount.publicKey
    })

    const createAccountTxn = new Transaction();
    createAccountTxn.add(ix);
    const signature = await connection.sendTransaction(createAccountTxn, [adminAccount, dataAccount]);

    await connection.confirmTransaction(signature);
    console.log(dataAccount.publicKey.toBase58());

    const dataAccountInfo = await connection.getAccountInfo(dataAccount.publicKey);
    const counter = borsh.deserialize(schema, dataAccountInfo!.data);
    console.log(counter!.count);
    expect(counter!.count).toBe(0);

})