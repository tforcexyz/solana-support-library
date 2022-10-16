
import {
  BorshCoder,
  Idl
} from '@project-serum/anchor';
import {
  AccountMeta,
  PublicKey,
  TransactionInstruction
} from '@solana/web3.js';
import ProgramIdl from '../target/idl/tfx_test_framework.json';

const coder = new BorshCoder(ProgramIdl as Idl);

export class TestProgramInstruction {

  static announce(
    sender: PublicKey,
    content: string,
    testFrameworkProgramId: PublicKey,
  ): TransactionInstruction {
    const request = {
      content: Buffer.from(content, 'utf8'),
    };
    const data = coder.instruction.encode('announce', request);

    const keys: AccountMeta[] = [
      <AccountMeta>{ pubkey: sender, isSigner: true, isWritable: false, },
    ];

    return new TransactionInstruction({
      data,
      keys,
      programId: testFrameworkProgramId,
    });
  }

  static foward(
    instruction: TransactionInstruction,
    testFrameworkProgramId: PublicKey,
  ): TransactionInstruction {
    const request = {
      data: instruction.data,
    };
    const data = coder.instruction.encode('forward', request);

    const keys: AccountMeta[] = [...instruction.keys];
    keys.push(<AccountMeta>{ pubkey: instruction.programId, isSigner: false, isWritable: false, });

    return new TransactionInstruction({
      data,
      keys,
      programId: testFrameworkProgramId,
    });
  }

  static multiply(
    firstNumber: number,
    secondNumber: number,
    testFrameworkProgramId: PublicKey,
  ): TransactionInstruction {
    const request = {
      firstNumber,
      secondNumber,
    };
    const data = coder.instruction.encode('multiply', request);

    const keys: AccountMeta[] = [];

    return new TransactionInstruction({
      data,
      keys,
      programId: testFrameworkProgramId,
    });
  }
}
