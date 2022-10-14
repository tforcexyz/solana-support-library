import {
  Connection,
  Keypair,
  PublicKey
} from '@solana/web3.js';
import BN from 'bn.js';
import {
  InstructionLog,
  LogMessageProcessor,
} from './core/log_message_processor.service';

export interface TransactionLog {
  txSignature: string
  instructionLogs: InstructionLog[]
  rawLogMessages: string[]
}

export class SolanaService {

  static async getAccountBalance(
    connection: Connection,
    address: PublicKey,
  ): Promise<BN> {
    const lamports = await connection.getBalance(address);
    return new BN(lamports);
  }

  static async getMinimumBalanceForRentExemption(
    connection: Connection,
    space: number,
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(space)
  }

  static async getTransactionLogMessages(
    connection: Connection,
    txSignature: string,
  ): Promise<TransactionLog> {
    const transactionResponse = await connection.getTransaction(txSignature);
    if(transactionResponse == null) {
      return null;
    }
    const rawLogMessages = transactionResponse.meta.logMessages;
    const instructionLogs = LogMessageProcessor.processLogs(rawLogMessages);
    return <TransactionLog>{
      txSignature,
      instructionLogs,
      rawLogMessages,
    };
  }

  static async generateKeypairFromSeed(fromPublicKey: PublicKey,
    seed: string,
    programId: PublicKey,
  ): Promise<Keypair> {
    const seedPubKey = await PublicKey.createWithSeed(fromPublicKey, seed, programId);
    const seedBytes = seedPubKey.toBytes();
    return Keypair.fromSeed(seedBytes);
  }

  static async isAddressAvailable(
    connection: Connection,
    address: PublicKey,
  ): Promise<boolean> {
    const programInf = await connection.getAccountInfo(address);
    return programInf === null;
  }

  static async isAddressInUse(
    connection: Connection,
    address: PublicKey,
  ): Promise<boolean> {
    const programInf = await connection.getAccountInfo(address);
    return programInf !== null;
  }

  static async isProgramAccount(
    connection: Connection,
    address: PublicKey,
  ): Promise<boolean> {
    const programInf = await connection.getAccountInfo(address);
    if (programInf === null) {
      console.log(`Program ${address.toBase58()} does not exist`, '\n');
      return false;
    }
    else if (!programInf.executable) {
      console.log(`Program ${address.toBase58()} is not executable`, '\n');
      return false;
    }
    return true;
  }
}
