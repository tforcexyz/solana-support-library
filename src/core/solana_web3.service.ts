import {
  Connection,
  sendAndConfirmTransaction,
  Signer,
  Transaction
} from '@solana/web3.js'

export async function getProgramReturn(
  connection: Connection,
  txHash: string,
): Promise<string> {
  const txInfo = await connection.getTransaction(txHash)
  const logMessages = txInfo.meta.logMessages
  for(const message of logMessages) {
    if(message.startsWith('Program return: ')) {
      const base64Value = message.slice(61)
      return Buffer.from(base64Value, 'base64').toString('hex')
    }
  }
  return null
}

export async function sendTransaction(
  connection: Connection,
  transaction: Transaction,
  signers: Signer[],
): Promise<string> {
  try {
    return sendAndConfirmTransaction(
      connection,
      transaction,
      signers,
      {
        skipPreflight: true,
      },
    )
  } catch(err) {
    if(err) {
      const props = []
      for(const prop in err) {
        props.push(prop)
      }
      console.debug(props)
    }
    console.error(err)
    throw err
  }
}
