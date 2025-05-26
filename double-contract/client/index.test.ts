import { test, expect } from 'bun:test';
import { LiteSVM } from "litesvm";
import {
	PublicKey,
	Transaction,
	SystemProgram,
	Keypair,
	LAMPORTS_PER_SOL,
	TransactionInstruction,
} from "@solana/web3.js";

test("one transfer", () => {
	const svm = new LiteSVM();
	const contractPubkey = PublicKey.unique();
	svm.addProgramFromFile(contractPubkey, "/home/fuyofulo/smart_contracts/double-contract/target/deploy/double_contract.so");
	const payer = new Keypair();
	svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));
	const dataAccount = new Keypair();
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
	const balanceAfter = svm.getBalance(dataAccount.publicKey);
	console.log(balanceAfter?.toString());
	expect(balanceAfter).toBe(svm.minimumBalanceForRentExemption(BigInt(4)));


	function doubleIt() {
		const ix2 = [new TransactionInstruction({
			keys: [
				{ pubkey: dataAccount.publicKey, isSigner: false, isWritable: true},
			],
			programId: contractPubkey,
			data: Buffer.from("")
		})]
		const tx2 = new Transaction();
		const blockhash2 = svm.latestBlockhash();
		tx2.recentBlockhash = blockhash2;
		tx2.feePayer = payer.publicKey;
		tx2.add(...ix2);
		tx2.sign(payer);
		svm.sendTransaction(tx2);
		svm.expireBlockhash();
	}

	doubleIt();
	doubleIt();
	doubleIt();
	doubleIt();
	

	const countData = svm.getAccount(dataAccount.publicKey);
	console.log(countData);

	expect(countData?.data[0]).toBe(8);
	expect(countData?.data[1]).toBe(0);
	expect(countData?.data[2]).toBe(0);
	expect(countData?.data[3]).toBe(0);

});