import { test, expect } from 'bun:test';
import { LiteSVM } from 'litesvm';
import { PublicKey, LAMPORTS_PER_SOL, Keypair, SystemProgram, Transaction, TransactionInstruction } from '@solana/web3.js'

test("cpi test", async () => {
    let svm = new LiteSVM();
    let doubleContract = PublicKey.unique();
    let cpiContract = PublicKey.unique();

    svm.addProgramFromFile(doubleContract, "/home/fuyofulo/smart_contracts/double-contract/target/deploy/double_contract.so");
    svm.addProgramFromFile(cpiContract, "/home/fuyofulo/smart_contracts/cpi-contract/target/deploy/cpi_contract.so");

    let userAcc = new Keypair();
    let dataAcc = new Keypair();

    svm.airdrop(userAcc.publicKey, BigInt(LAMPORTS_PER_SOL));

    createDataAccOnChain(svm, dataAcc, userAcc, doubleContract);

    let ix = new TransactionInstruction({
        keys: [
            { pubkey: dataAcc.publicKey, isSigner: true, isWritable: true },
            { pubkey: doubleContract, isSigner: false, isWritable: false }
        ],
        programId: cpiContract,
        data: Buffer.from("")
    })

    const blockhash = svm.latestBlockhash();
    let txn = new Transaction().add(ix);
    txn.recentBlockhash = blockhash;
    txn.feePayer = userAcc.publicKey;
	txn.sign(userAcc, dataAcc);

    svm.sendTransaction(txn);

    const dataAccData = svm.getAccount(dataAcc.publicKey);
    console.log(dataAccData)


	expect(dataAccData?.data[0]).toBe(8);
    expect(dataAccData?.data[0]).toBe(0);
    expect(dataAccData?.data[0]).toBe(0);
    expect(dataAccData?.data[0]).toBe(0);

})

function createDataAccOnChain(svm: LiteSVM, dataAccount: Keypair, payer: Keypair, contractPubkey: PublicKey) {
    const blockhash = svm.latestBlockhash();
    const ixs = [SystemProgram.createAccount({
		programId: contractPubkey,
		fromPubkey: payer.publicKey,
		newAccountPubkey: dataAccount.publicKey,
		lamports: Number(svm.minimumBalanceForRentExemption(BigInt(4))),
		space: 4
	})]
    const tx = new Transaction();
	tx.recentBlockhash = blockhash;
	tx.add(...ixs);
	tx.sign(payer, dataAccount);
	svm.sendTransaction(tx);
}