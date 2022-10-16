import { PublicKey } from '@solana/web3.js';
import {
  InstructionLog,
  ProgramLogCategory
} from './interfaces';

const ANCHOR_PROGRAM_ERROR = 'Program log: AnchorError ';
const PROGRAM_DATA = 'Program data: ';
const PROGRAM_ERROR = 'Program log: Error: ';
const PROGRAM_LOG = 'Program log: ';
const PROGRAM_PANIC = 'Program log: panicked at';

export class LogMessageProcessor {
  static processLogs(
    messages: string[]
  ): InstructionLog[] {
    let results: InstructionLog[] = [];
    let currentResult: InstructionLog;
    let processingLevel = -1;
    let processingResults: InstructionLog[] = [];
    for(let message of messages) {
      let [category, content] = categorizeLog(message);
      if(category == ProgramLogCategory.ProgramStart || category == ProgramLogCategory.CpiCall) {
        processingLevel++;
        currentResult = <InstructionLog>{
          publicKey: new PublicKey(content),
          messages: [],
          datas: [],
          isSuccess: true,
          return: null,
          errorCode: null,
          errorMessage: null,
          children: [],
        };
        processingResults.push(currentResult);
      }
      currentResult.messages.push([category, content]);
      if(category == ProgramLogCategory.ProgramData) {
        currentResult.datas.push(content);
      }
      if(category == ProgramLogCategory.ProgramReturn) {
        currentResult.return = content;
      }
      if(category == ProgramLogCategory.ProgramError) {
        currentResult.errorMessage = content;
      }
      if(category == ProgramLogCategory.ProgramSuccess || category == ProgramLogCategory.ProgramFailed) {
        if(currentResult.children.length === 0) {
          currentResult.isSuccess = category == ProgramLogCategory.ProgramSuccess;
        }
        else {
          currentResult.isSuccess = currentResult.children.every(child => child.isSuccess);
        }
        if(category == ProgramLogCategory.ProgramFailed) {
          const errorCodeHex = content.split('|')[1];
          const errorCodeDec = parseInt(errorCodeHex, 16);
          currentResult.errorCode = `${errorCodeHex}|${errorCodeDec}`;
        }
        processingLevel--;
        if(processingLevel === -1) {
          results.push(currentResult);
          currentResult = null;
        }
        else {
          const parentResult = processingResults[processingLevel];
          parentResult.children.push(currentResult);
          currentResult = parentResult;
        }
        processingResults.pop();
      }
    }
    return results;
  }
}

function categorizeLog(
  message: string
): [ProgramLogCategory, string] {
  const match1 = message.match(/^Program (.*) invoke \[(\d+)\]/);
  if(match1 !== null && match1.at(2).toString() == '1') {
    return [ProgramLogCategory.ProgramStart, match1.at(1)];
  }
  if(match1 !== null && match1.at(2) != '1') {
    return [ProgramLogCategory.CpiCall, match1.at(1)];
  }
  // This is a `msg!` log
  if(message.startsWith(PROGRAM_LOG)) {
    if(message.startsWith(PROGRAM_ERROR)) {
      return [ProgramLogCategory.ProgramError, `Reason: ${message.slice(PROGRAM_ERROR.length)}`];
    }
    else if(message.startsWith(ANCHOR_PROGRAM_ERROR)) {
      return [ProgramLogCategory.ProgramError, `Reason: ${message.slice(PROGRAM_LOG.length)}`];
    }
    else if(message.startsWith(PROGRAM_PANIC)) {
      return [ProgramLogCategory.ProgramError, `Reason: ${message.slice(PROGRAM_LOG.length)}`];
    }
    else {
      return [ProgramLogCategory.ProgramMessage, message.slice(PROGRAM_LOG.length)];
    }
  }
  // This is a `sol_log_data` log
  if(message.startsWith(PROGRAM_DATA)) {
    return [ProgramLogCategory.ProgramData, message.slice(PROGRAM_DATA.length)];
  }
  const match2 = message.match(/^Program return: (.*) (.*)/);
  if(match2 !== null) {
    return [ProgramLogCategory.ProgramReturn, match2.at(2)];
  }
  const match3 = message.match(/^Program (.*) success/);
  if(match3 !== null) {
    return [ProgramLogCategory.ProgramSuccess, match3.at(1)];
  }
  const match4 = message.match(/Program (.*) failed: custom program error: (.*)/);
  if(match4 !== null) {
    return [ProgramLogCategory.ProgramFailed, `${match4.at(1)}|${match4.at(2)}`];
  }
  const match5 = message.match(/Program (.*) failed: Program failed to complete/);
  if(match5 !== null) {
    return [ProgramLogCategory.ProgramFailed, `${match5.at(1)}|0x0`];
  }
  return [ProgramLogCategory.Others, message];
}

/**** SAMPLE MESSAGES
 * Error: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x1770
 * Program TFXeSSo3gA2uXnZfwtHNodvAQnkMMdkZ1soXPqjXaem invoke [1]
 * Program log: Instruction: Forward
 * Program log: AnchorError thrown in programs/test_framework/src/lib.rs:28. Error Code: ContentTooLong. Error Number: 6000. Error Message: Content is too long.
 * Program TFXeSSo3gA2uXnZfwtHNodvAQnkMMdkZ1soXPqjXaem consumed 3256 of 200000 compute units
 * Program TFXeSSo3gA2uXnZfwtHNodvAQnkMMdkZ1soXPqjXaem failed: custom program error: 0x1770
 * Error: Transaction 54qGgWWk5otzrKVmw4zEWTN8aygPPe16UyueqYTC4NtuEHZvFqNTrCSGkyHuMaMPzJ6W9XJi76tiMYW7TYWMuzWq failed ({"err":{"InstructionError":[0,{"Custom":6000}]}})
 * Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]
 * Program log: Instruction: MintTo
 * Program log: Error: Account not associated with this Mint
 * Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 2713 of 200000 compute units
 * Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA failed: custom program error: 0x3
 * Program log: panicked at 'called `Option::unwrap()` on a `None` value', programs/test_framework/src/lib.rs:83:58
 * Program failed to complete: BPF program panicked
 * Program TFXeSSo3gA2uXnZfwtHNodvAQnkMMdkZ1soXPqjXaem failed: Program failed to complete
****/
