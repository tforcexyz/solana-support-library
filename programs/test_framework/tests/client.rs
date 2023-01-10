use anchor_lang::{
  InstructionData,
  ToAccountMetas,
};
use solana_sdk::{
  instruction::{
    Instruction,
  },
  pubkey::{
    Pubkey,
  },
};
use test_framework::{
  accounts as p_context,
  ID as PROGRAM_ID,
  instruction as p_instruction,
};

pub fn create_announce_instruction(
  sender: &Pubkey,
  content: Vec<u8>,
) -> Instruction {

  let data = p_instruction::Announce {
    content,
  }.data();

  let accounts = p_context::AnnounceContext {
    sender: *sender,
  }.to_account_metas(None);

  Instruction {
    data,
    accounts,
    program_id: PROGRAM_ID,
  }
}
