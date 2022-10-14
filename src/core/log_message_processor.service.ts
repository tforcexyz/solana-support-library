import { PublicKey } from '@solana/web3.js';

const PROGRAM_LOG = 'Program log: ';
const PROGRAM_DATA = 'Program data: ';
const PROGRAM_LOG_START_INDEX = PROGRAM_LOG.length;
const PROGRAM_DATA_START_INDEX = PROGRAM_DATA.length;

export enum ProgramLogCategory {
  ProgramStart = 1,
  ProgramSuccess = 2,
  CpiCall = 3,
  ProgramMessage = 4,
  ProgramData = 5,
  ProgramReturn = 6,
  Others = 0,
}

export interface InstructionLog {
  publicKey: PublicKey
  messages: [ProgramLogCategory, string][]
  datas: string[]
  return: string
  children: InstructionLog[]
}

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
          return: null,
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
      if(category == ProgramLogCategory.ProgramSuccess) {
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
    return [ProgramLogCategory.ProgramMessage, message.slice(PROGRAM_LOG_START_INDEX)];
  }
  // This is a `sol_log_data` log
  if(message.startsWith(PROGRAM_DATA)) {
    return [ProgramLogCategory.ProgramData, message.slice(PROGRAM_DATA_START_INDEX)];
  }
  const match2 = message.match(/^Program return: (.*) (.*)/);
  if(match2 !== null) {
    return [ProgramLogCategory.ProgramReturn, match2.at(2)];
  }
  const match3 = message.match(/^Program (.*) success/);
  if(match3 !== null) {
    return [ProgramLogCategory.ProgramSuccess, match3.at(1)];
  }
  return [ProgramLogCategory.Others, message];
}
