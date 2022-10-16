import {
  Connection,
  Keypair,
  SystemProgram,
  Transaction
} from '@solana/web3.js';
import BN from 'bn.js';
import { expect } from 'chai';
import { TestProgramInstruction } from '../client/test_program.instruction-service';
import {
  executeTransaction,
  executeTransaction2,
  SolanaService,
  TokenProgramInstructionService,
  TokenProgramService
} from '../src';
import {
  SolanaConfigService,
  TestAccountService,
  TokenName
} from '../src/config';
import { PROGRAM_ID } from './common';

describe('message_log_processing_test', function() {

  const connection = new Connection('http://localhost:8899', 'confirmed')
  let defaultAccount: Keypair
  let ownerAccount: Keypair
  let testAccount1: Keypair
  let usdcTokenAccount: Keypair

  this.beforeAll(async function() {
    defaultAccount = await SolanaConfigService.getDefaultAccount();
    ownerAccount = await TestAccountService.getAccount(0);
    testAccount1 = await TestAccountService.getAccount(1);
    usdcTokenAccount = await TestAccountService.getTokenAccount(TokenName.USDC);

    await TokenProgramService.createTokenMint(
      connection,
      defaultAccount,
      usdcTokenAccount,
      6,
      ownerAccount.publicKey,
      null,
    );
  });

  it('get_message_log_multi_instructions', async function() {
    const transaction = new Transaction();
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: defaultAccount.publicKey,
      toPubkey: testAccount1.publicKey,
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
      toPubkey: testAccount1.publicKey,
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

  it('get_message_log_standard_program_error', async function() {
    const transaction = new Transaction();
    const testAccount0Balance = await SolanaService.getAccountBalance(
      connection,
      testAccount1.publicKey,
    );
    const transferInstruction = SystemProgram.transfer({
      fromPubkey: testAccount1.publicKey,
      toPubkey: defaultAccount.publicKey,
      lamports: testAccount0Balance.toNumber() + 1000000,
    });
    transaction.add(transferInstruction);

    const [, txLog] = await executeTransaction2(connection, transaction, [
      defaultAccount,
      testAccount1,
    ]);

    expect(txLog).is.not.null;
  });

  it('get_message_log_standard_program_error', async function() {
    const testAccount1UsdcTokenAddress = await TokenProgramService.createAssociatedTokenAccount(
      connection,
      defaultAccount,
      testAccount1.publicKey,
      usdcTokenAccount.publicKey,
    );
    const transaction = new Transaction();
    const mintInstruction = TokenProgramInstructionService.mint(
      testAccount1.publicKey,
      testAccount1UsdcTokenAddress,
      testAccount1UsdcTokenAddress,
      new BN('1000000'),
    );
    transaction.add(mintInstruction);

    const [, txLog] = await executeTransaction2(connection, transaction, [
      defaultAccount,
      testAccount1,
    ]);

    expect(txLog).is.not.null;
  });

  it('get_message_log_anchor_program_error', async function() {
    const transaction = new Transaction();
    const announceInstruction = TestProgramInstruction.announce(
      defaultAccount.publicKey,
      'An announcement should be short and must not exceed 64 characters.',
      PROGRAM_ID,
    );
    transaction.add(announceInstruction);

    const [, txLog] = await executeTransaction2(connection, transaction, [
      defaultAccount,
    ]);

    expect(txLog).is.not.null;
  });

  it('get_message_log_unhandled_exception', async function() {
    const transaction = new Transaction();
    const multiplyInstruction = TestProgramInstruction.multiply(
      320,
      512,
      PROGRAM_ID,
    );
    const forwardInstruction = TestProgramInstruction.foward(
      multiplyInstruction,
      PROGRAM_ID,
    );
    transaction.add(forwardInstruction);

    const [, txLog] = await executeTransaction2(connection, transaction, [
      defaultAccount,
    ]);

    expect(txLog).is.not.null;
  });
});
