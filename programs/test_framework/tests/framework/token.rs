use solana_program_test::{
  ProgramTestContext,
};
use solana_sdk::{
  account::{
    ReadableAccount,
  },
  instruction::{
    Instruction,
  },
  pubkey::{
    Pubkey,
  },
  program_pack::{
    Pack,
  },
  signature::{
    Keypair,
    Signer,
  },
  system_instruction,
};
use super::{
  context::{
    get_payer,
    get_rent,
    process_transaction,
  },
  system::{
    get_account_type,
  },
  spl_atoken::{
    create_associated_token_account_instruction,
  },
  spl_token::{
    approve_token_instruction,
    AuthorityType,
    burn_token_instruction,
    change_mint_authority_instruction,
    close_token_account_instruction,
    freeze_token_account_instruction,
    initialize_token_account_instruction,
    initialize_token_mint_instruction,
    mint_token_instruction,
    ID as TOKEN_PROGRAM_ID,
    revoke_token_instruction,
    thaw_token_account_instruction,
    transfer_token_instruction,
    TokenAccount,
    TOKEN_ACCOUNT_LENGTH,
    TOKEN_MINT_LENGTH,
  },
};
pub use super::spl_atoken::get_associated_token_account_address;

pub async fn approve_token(
  context: &mut ProgramTestContext,
  owner: &Keypair,
  owner_token_address: &Pubkey,
  delegate_address: &Pubkey,
  amount: u64,
) {
  let payer = get_payer(context);

  let approve_token_ix = approve_token_instruction(
    &owner.pubkey(),
    &owner_token_address,
    &delegate_address,
    amount,
  );

  process_transaction(
    context,
    &payer,
    &[approve_token_ix],
    &[&payer, &owner],
  ).await;
}

pub async fn burn_token(
  context: &mut ProgramTestContext,
  owner: &Keypair,
  owner_token_address: &Pubkey,
  token_mint_address: &Pubkey,
  amount: u64,
) {
  let payer = get_payer(context);

  let burn_token_ix = burn_token_instruction(
    &owner.pubkey(),
    &owner_token_address,
    &token_mint_address,
    amount,
  );

  process_transaction(
    context,
    &payer,
    &[burn_token_ix],
    &[&payer, &owner],
  ).await;
}

pub async fn change_mint_authority(
  context: &mut ProgramTestContext,
  authority: &Keypair,
  token_mint_address: &Pubkey,
  auhtority_type: AuthorityType,
  new_authority_address: Option<&Pubkey>,
) {
  let payer = get_payer(context);

  let change_authority_ix = change_mint_authority_instruction(
    &authority.pubkey(),
    &token_mint_address,
    auhtority_type,
    new_authority_address,
  );

  process_transaction(
    context,
    &payer,
    &[change_authority_ix],
    &[&payer, &authority],
  ).await;
}

pub async fn close_token_account(
  context: &mut ProgramTestContext,
  owner: &Keypair,
  owner_token_address: &Pubkey,
) {
  let payer = get_payer(context);

  let close_token_account_ix = close_token_account_instruction(
    &owner.pubkey(),
    &owner_token_address,
  );

  process_transaction(
    context,
    &payer,
    &[close_token_account_ix],
    &[&payer, &owner],
  ).await;
}

pub async fn create_token_account(
  context: &mut ProgramTestContext,
  wallet_address:&Pubkey,
  token_mint_address: &Pubkey,
  token_account: &Keypair,
) {
  let payer = get_payer(context);
  let rent = get_rent(context).await;

  let create_account_ix = system_instruction::create_account(
    &payer.pubkey(),
    &token_account.pubkey(),
    rent.minimum_balance(TOKEN_ACCOUNT_LENGTH),
    TOKEN_ACCOUNT_LENGTH as u64,
    &TOKEN_PROGRAM_ID,
  );
  let initialize_account_ix = initialize_token_account_instruction(
    &wallet_address,
    &token_account.pubkey(),
    &token_mint_address,
  );

  process_transaction(
    context,
    &payer,
    &[create_account_ix, initialize_account_ix],
    &[&payer, &token_account],
  ).await;
}

pub async fn create_token_mint(
  context: &mut ProgramTestContext,
  token_mint: &Keypair,
  decimals: u8,
  authority: &Pubkey,
  freeze_authority: Option<&Pubkey>,
) {
  let payer = get_payer(context);
  let rent = get_rent(context).await;

  let create_account_ix = system_instruction::create_account(
    &payer.pubkey(),
    &token_mint.pubkey(),
    rent.minimum_balance(TOKEN_MINT_LENGTH),
    TOKEN_MINT_LENGTH as u64,
    &TOKEN_PROGRAM_ID,
  );
  let initialize_mint_ix = initialize_token_mint_instruction(
    &token_mint.pubkey(),
    decimals,
    &authority,
    freeze_authority,
  );

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
  let payer = get_payer(context);

  let create_ata_ix = create_associated_token_account_instruction(
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

  get_associated_token_account_address(
    owner,
    token_mint,
  )
}

pub async fn freeze_token_account(
  context: &mut ProgramTestContext,
  authority: &Keypair,
  token_mint_address: &Pubkey,
  token_account_address: &Pubkey,
) {
  let payer = get_payer(context);

  let freeze_token_account_ix = freeze_token_account_instruction(
    &authority.pubkey(),
    &token_mint_address,
    &token_account_address,
  );

  process_transaction(
    context,
    &payer,
    &[freeze_token_account_ix],
    &[&payer, &authority],
  ).await;
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
  let payer = get_payer(context);

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

  let mint_token_ix = mint_token_instruction(
    &authority.pubkey(),
    &token_mint,
    &recipient_address,
    amount,
  );
  instructions.push(mint_token_ix);

  process_transaction(
      context,
      &payer,
      &instructions,
      &[&payer, &authority],
    ).await;
}

pub async fn revoke_token(
  context: &mut ProgramTestContext,
  owner: &Keypair,
  owner_token_address: &Pubkey,
) {
  let payer = get_payer(context);

  let revoke_token_ix = revoke_token_instruction(
    &owner.pubkey(),
    &owner_token_address,
  );

  process_transaction(
    context,
    &payer,
    &[revoke_token_ix],
    &[&payer, &owner],
  ).await;
}

pub async fn thaw_token_account(
  context: &mut ProgramTestContext,
  authority: &Keypair,
  token_mint_address: &Pubkey,
  token_account_address: &Pubkey,
) {
  let payer = get_payer(context);

  let thaw_token_account_ix = thaw_token_account_instruction(
    &authority.pubkey(),
    &token_mint_address,
    &token_account_address,
  );

  process_transaction(
    context,
    &payer,
    &[thaw_token_account_ix],
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
  let payer = get_payer(context);

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

  let transfer_token_ix = transfer_token_instruction(
    &sender.pubkey(),
    &sender_token,
    &recipient_address,
    amount,
  );
  instructions.push(transfer_token_ix);

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
    recipient_address = get_associated_token_account_address(
      recipient,
      token_mint,
    );
    let create_ata_ix = create_associated_token_account_instruction(
      &payer.pubkey(),
      &recipient,
      &token_mint,
    );
    return (recipient_address, Some(create_ata_ix));
  }
  (recipient_address, None) // account_type == 2u8
}
