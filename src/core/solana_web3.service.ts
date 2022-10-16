import {
  ConfirmOptions,
  Connection,
  sendAndConfirmRawTransaction,
  sendAndConfirmTransaction,
  Signer,
  Transaction
} from '@solana/web3.js';
import { SignatureTuple } from './interfaces';
import {
  SolanaService,
  TransactionLog
} from './solana.service';

export async function getProgramReturn(
  connection: Connection,
  txHash: string,
): Promise<string> {
  const txInfo = await connection.getTransaction(txHash);
  const logMessages = txInfo.meta.logMessages;
  for(const message of logMessages) {
    if(message.startsWith('Program return: ')) {
      const base64Value = message.slice(61);
      return Buffer.from(base64Value, 'base64').toString('hex');
    }
  }
  return null;
}

export async function executeRawTransaction(
  connection: Connection,
  rawTransaction: Buffer,
  signatures: SignatureTuple[],
): Promise<string> {

  const transaction = Transaction.from(rawTransaction);
  for(let signature of signatures) {
    transaction.addSignature(signature.publicKey, signature.signature);
  }

  try {
    return await sendAndConfirmRawTransaction(
      connection,
      transaction.serialize(),
    );
  }
  catch(err) {
    console.info(err.toString());
    try {
      const txLog = await handleRpcError(
        connection,
        err,
      );
      if(txLog && txLog.errorMessage) {
        console.info(txLog.errorMessage);
      }
    }
    catch {};
    return null;
  }
}

export async function executeRawTransaction2(
  connection: Connection,
  rawTransaction: Buffer,
  signatures: SignatureTuple[],
  options?: ConfirmOptions,
): Promise<[string, TransactionLog]> {

  const transaction = Transaction.from(rawTransaction);
  for(let signature of signatures) {
    transaction.addSignature(signature.publicKey, signature.signature);
  }

  try {
    const txSign = await sendAndConfirmRawTransaction(
      connection,
      transaction.serialize(),
      options,
    );
    return [txSign, null];
  }
  catch(err) {
    const txLog = await handleRpcError(
      connection,
      err,
    );
    return [null, txLog];
  }
}

export async function executeTransaction(
  connection: Connection,
  transaction: Transaction,
  signers: Signer[],
): Promise<string> {

  try {
    const txSign = await sendAndConfirmTransaction(
      connection,
      transaction,
      signers,
    );
    return txSign;
  }
  catch(err) {
    console.info(err.toString());
    try {
      const txLog = await handleRpcError(
        connection,
        err,
      );
      if(txLog && txLog.errorMessage) {
        console.info(txLog.errorMessage);
      }
    }
    catch {};
    return null;
  }
}

export async function executeTransaction2(
  connection: Connection,
  transaction: Transaction,
  signers: Signer[],
  options?: ConfirmOptions,
): Promise<[string, TransactionLog]> {

  try {
    const txSign = await sendAndConfirmTransaction(
      connection,
      transaction,
      signers,
      options,
    );
    return [txSign, null];
  }
  catch(err) {
    const txLog = await handleRpcError(
      connection,
      err,
    );
    return [null, txLog];
  }
}

async function handleRpcError(
  connection: Connection,
  error: any,
): Promise<TransactionLog> {
  const errorMessage = error.toString();
  const hasPreflight = Object.getOwnPropertyNames(error).indexOf('logs') > -1;
  if(hasPreflight) {
    return SolanaService.formatLogMessages(error.logs);
  }
  const extractTxSignMatch = errorMessage.match(/Error: Transaction (.*) failed/);
  if(extractTxSignMatch === null) {
    return null;
  }
  const txSign = extractTxSignMatch.at(1);
  const transactionLog = await SolanaService.getTransactionLogMessages(connection, txSign);
  return transactionLog;
}
