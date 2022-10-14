import {
  Connection,
  Keypair,
  SystemProgram,
  Transaction
} from '@solana/web3.js';
import { expect } from 'chai';
import { TestProgramInstruction } from '../client/test_program.instruction-service';
import {
  executeTransaction,
  SolanaService
} from '../src';
import {
  SolanaConfigService,
  TestAccountService
} from '../src/config';
import { PROGRAM_ID } from './common';

describe('message_log_processing_test', function() {

  const connection = new Connection('http://localhost:8899', 'confirmed')
  let defaultAccount: Keypair
  let testAccount0: Keypair

  this.beforeAll(async function() {
    defaultAccount = await SolanaConfigService.getDefaultAccount();
    testAccount0 = await TestAccountService.getAccount(0);
  });

  it('get_message_log_multi_instructions', async function() {
    const transaction = new Transaction();
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: defaultAccount.publicKey,
      toPubkey: testAccount0.publicKey,
      lamports: 1000000,
    });
    transaction.add(transferInstruction);
    const announceInstruction = TestProgramInstruction.announce(
      defaultAccount.publicKey,
      'Hello World',
      PROGRAM_ID,
    );
    transaction.add(announceInstruction);

    const txSign = await executeTransaction(connection, transaction, [
      defaultAccount,
    ]);
    const transactionLog = await SolanaService.getTransactionLogMessages(connection, txSign);

    expect(transactionLog.txSignature).eq(txSign);
    expect(transactionLog.instructionLogs.length).eq(2);
    expect(transactionLog.instructionLogs[0].publicKey.toBase58()).eq(SystemProgram.programId.toBase58());
    expect(transactionLog.instructionLogs[0].children.length).eq(0);
    expect(transactionLog.instructionLogs[1].publicKey.toBase58()).eq(PROGRAM_ID.toBase58());
    expect(transactionLog.instructionLogs[1].children.length).eq(0);
  });

  it('get_message_log_nested_instructions', async function() {
    const transaction = new Transaction();
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: defaultAccount.publicKey,
      toPubkey: testAccount0.publicKey,
      lamports: 1000000,
    });
    const forwardInstruction = TestProgramInstruction.foward(
      transferInstruction,
      PROGRAM_ID,
    );
    transaction.add(forwardInstruction);

    const txSign = await executeTransaction(connection, transaction, [
      defaultAccount,
    ]);
    const transactionLog = await SolanaService.getTransactionLogMessages(connection, txSign);

    expect(transactionLog.txSignature).eq(txSign);
    expect(transactionLog.instructionLogs.length).eq(1);
    expect(transactionLog.instructionLogs[0].publicKey.toBase58()).eq(PROGRAM_ID.toBase58());
    expect(transactionLog.instructionLogs[0].children.length).eq(1);
    expect(transactionLog.instructionLogs[0].children[0].publicKey.toBase58()).eq(SystemProgram.programId.toBase58());
  });
})
