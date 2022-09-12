use crate::framework::{
  system::{
    get_account_type,
    process_transaction,
  },
};
use solana_program_test::{
  ProgramTestContext,
};
use solana_sdk::{
  instruction::{
    Instruction,
  },
  program_pack::{
    Pack,
  },
  pubkey::{
    Pubkey,
  },
  signature::{
    Keypair,
    Signer,
  },
  system_instruction, account::ReadableAccount,
};
use spl_token::{
  state::{
    Account as TokenAccount,
    Mint as TokenMint,
  },
};

pub async fn create_token_mint(
  context: &mut ProgramTestContext,
  token_mint: &Keypair,
  decimals: u8,
  authority: &Pubkey,
  freeze_authority: Option<&Pubkey>,
) {
  let payer = Keypair::from_bytes(&context.payer.to_bytes())
    .unwrap();
  let rent = context.banks_client
    .get_rent()
    .await.unwrap();

  let create_account_ix = system_instruction::create_account(
    &payer.pubkey(),
    &token_mint.pubkey(),
    rent.minimum_balance(TokenMint::LEN),
    TokenMint::LEN as u64,
    &spl_token::id(),
  );
  let initialize_mint_ix = spl_token::instruction::initialize_mint(
    &spl_token::id(),
    &token_mint.pubkey(),
    &authority,
    freeze_authority,
    decimals,
  ).unwrap();

  process_transaction(
    context,
    &payer,
    &[create_account_ix, initialize_mint_ix],
    &[&payer, &token_mint],
  ).await;
}

pub async fn create_associated_token_account(
  context: &mut ProgramTestContext,
  owner: &Pubkey,
  token_mint: &Pubkey,
) -> Pubkey {
  let payer = Keypair::from_bytes(&context.payer.to_bytes())
    .unwrap();

  let create_ata_ix = spl_associated_token_account::create_associated_token_account(
    &payer.pubkey(),
    &owner,
    token_mint,
  );

  process_transaction(
    context,
    &payer,
    &[create_ata_ix],
    &[&payer]
  ).await;

  get_associated_token_address(
    owner,
    token_mint,
  )
}

pub fn get_associated_token_address(
  owner: &Pubkey,
  token_mint: &Pubkey,
) -> Pubkey {
  spl_associated_token_account::get_associated_token_address(
    &owner,
    &token_mint,
  )
}

pub async fn get_token_account_balance(
  context: &mut ProgramTestContext,
  address: &Pubkey,
) -> u64 {
  let token_account = get_token_account_info(context, address)
    .await;
  token_account.amount
}

pub async fn get_token_account_info(
  context: &mut ProgramTestContext,
  address: &Pubkey,
) -> TokenAccount {
  let token_account_option = context.banks_client
    .get_account(*address)
    .await.unwrap();
  let token_account_info = token_account_option.unwrap();
  TokenAccount::unpack(token_account_info.data())
    .unwrap()
}

pub async fn get_token_account_mint(
  context: &mut ProgramTestContext,
  address: &Pubkey,
) -> Pubkey {
  let token_account = get_token_account_info(context, address)
    .await;
  token_account.mint
}

pub async fn mint_token(
  context: &mut ProgramTestContext,
  authority: &Keypair,
  token_mint: &Pubkey,
  recipient: &Pubkey,
  amount: u64,
) {
  let payer = Keypair::from_bytes(&context.payer.to_bytes())
    .unwrap();

  let mut instructions: Vec<Instruction> = Vec::new();
  let (recipient_address, create_ata_ix_option) = check_and_create_ata_ix(
      context,
      recipient,
      &payer,
      token_mint
    ).await;
  if create_ata_ix_option.is_some() {
    instructions.push(create_ata_ix_option.unwrap());
  }
  instructions.push(
    spl_token::instruction::mint_to(
      &spl_token::id(),
      &token_mint,
      &recipient_address,
      &authority.pubkey(),
      &[],
      amount
    ).unwrap()
  );
  process_transaction(
      context,
      &payer,
      &instructions,
      &[&payer, &authority],
    ).await;
}

pub async fn transfer_token(
  context: &mut ProgramTestContext,
  sender: &Keypair,
  sender_token: &Pubkey,
  recipient: &Pubkey,
  amount: u64,
) {
  let payer = Keypair::from_bytes(&context.payer.to_bytes())
    .unwrap();

  let mut instructions: Vec<Instruction> = Vec::new();
  let token_mint = get_token_account_mint(context, sender_token).await;
  let (recipient_address, create_ata_ix_option) = check_and_create_ata_ix(
      context,
      recipient,
      &payer,
      &token_mint,
    ).await;
  if create_ata_ix_option.is_some() {
    instructions.push(create_ata_ix_option.unwrap());
  }
  instructions.push(spl_token::instruction::transfer(
      &spl_token::id(),
      &sender_token,
      &recipient_address,
      &sender.pubkey(),
      &[],
      amount,
    ).unwrap()
  );

  process_transaction(
      context,
      &payer,
      &instructions,
      &[&payer, &sender],
    ).await;
}

async fn check_and_create_ata_ix(
  context: &mut ProgramTestContext,
  recipient: &Pubkey,
  payer: &Keypair,
  token_mint: &Pubkey,
) -> (Pubkey, Option<Instruction>) {
  let account_type = get_account_type(context, recipient)
    .await;
  let mut recipient_address = *recipient;
  if account_type == 0u8 || account_type == 1u8 {
    recipient_address = get_associated_token_address(
      recipient,
      token_mint,
    );
    let create_ata_ix = spl_associated_token_account::create_associated_token_account(
      &payer.pubkey(),
      &recipient,
      &token_mint,
    );
    return (recipient_address, Some(create_ata_ix));
  }
  (recipient_address, None) // account_type == 2u8
}
