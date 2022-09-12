use solana_program_test::{
  ProgramTestContext,
};
use solana_sdk::{
  instruction::{
    Instruction,
  },
  pubkey::{
    Pubkey,
  },
  signature::{
    Keypair,
    Signer,
  },
  transaction::{
    Transaction,
  },
  system_instruction,
  system_program::{
    ID as SYSTEM_PROGRAM_ID,
  }
};
use spl_token::{
  ID as TOKEN_PROGRAM_ID,
};

pub async fn airdrop_lamport(
  context: &mut ProgramTestContext,
  recipient: &Pubkey,
  amount: u64,
) {
  let instruction = system_instruction::transfer(
    &context.payer.pubkey(),
    recipient,
    amount,
  );
  let payer = Keypair::from_bytes(&context.payer.to_bytes())
    .unwrap();

  process_transaction(
      context,
      &payer,
      &[instruction],
      &[&payer],
    )
    .await;
}


pub async fn get_account_type(
  context: &mut ProgramTestContext,
  address: &Pubkey,
) -> u8 {
  let account_option = context.banks_client
    .get_account(*address)
    .await.unwrap();
  if account_option.is_none() {
    return 0u8;
  }
  let account_info = account_option.unwrap();
  if account_info.owner == SYSTEM_PROGRAM_ID {
    return 1u8;
  }
  if account_info.owner ==TOKEN_PROGRAM_ID {
    return 2u8;
  }

  255u8
}

pub async fn get_account_balance(
  context: &mut ProgramTestContext,
  address: &Pubkey,
) -> u64 {
  context.banks_client
    .get_balance(*address)
    .await.unwrap()
}

pub async fn process_transaction(
  context: &mut ProgramTestContext,
  payer: &Keypair,
  instructions: &[Instruction],
  signers: &[&Keypair],
) {
  let tx = Transaction::new_signed_with_payer(
    &instructions.to_vec(),
    Some(&payer.pubkey()),
    &signers.to_vec(),
    context.last_blockhash,
  );

  context.banks_client.process_transaction(tx)
    .await.unwrap();
}

pub async fn transfer_lamport(
  context: &mut ProgramTestContext,
  sender: &Keypair,
  recipient: &Pubkey,
  amount: u64,
) {
  let instruction = system_instruction::transfer(
    &sender.pubkey(),
    recipient,
    amount,
  );
  let payer = Keypair::from_bytes(&context.payer.to_bytes())
    .unwrap();
  process_transaction(
      context,
      &payer,
      &[instruction],
      &[&payer, &sender],
    )
    .await;
}
