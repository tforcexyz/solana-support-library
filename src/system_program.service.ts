import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction
} from '@solana/web3.js';
import { sendTransaction } from './core/solana_web3.service';

export class SystemProgramService {
  static async transfer(
    connection: Connection,
    payerAccount: Keypair,
    recipientAddress: PublicKey,
    amount: number,
  ): Promise<boolean> {
    const transaction: Transaction = new Transaction()
    transaction.add(SystemProgram.transfer({
      fromPubkey: payerAccount.publicKey,
      toPubkey: recipientAddress,
      lamports: amount,
    }))
    const signers = [
      payerAccount
    ]
    const txSign = await sendTransaction(connection, transaction, signers)
    console.log(`Transferred ${amount} lamports from ${payerAccount.publicKey.toBase58()} to ${recipientAddress.toBase58()}`, '---', txSign, '\n')
    return true
  }
}
