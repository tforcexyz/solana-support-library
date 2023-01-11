pub mod framework;

use solana_program_test::{
  ProgramTest,
};
use solana_sdk::{
  signer::{
    Signer,
  },
};

use crate::framework::{
  account::{
    get_account,
    get_token_account_named,
    TokenName,
  },
  system::{
    airdrop_lamport,
  },
  token::{
    create_token_mint,
    create_associated_token_account,
    get_associated_token_address,
    get_token_account_balance,
    mint_token,
    transfer_token,
  },
};

#[tokio::test]
async fn mint_token_test() {
  let mut context = ProgramTest::default().start_with_context().await;
  let default_account = get_account(0);

  let token_mint_account = get_token_account_named(TokenName::USDT);
  create_token_mint(&mut context, &token_mint_account, 6, &default_account.pubkey(), None).await;

  let account_2 = get_account(2);
  let account_2_token_address = create_associated_token_account(&mut context, &account_2.pubkey(), &token_mint_account.pubkey()).await;
  mint_token(&mut context, &default_account, &token_mint_account.pubkey(), &account_2_token_address, 100_000_000).await;

  let account2_token_balance = get_token_account_balance(&mut context, &account_2_token_address).await;
  assert!(account2_token_balance == 100_000_000, "account2_token_balance invalid");
}

#[tokio::test]
async fn transfer_token_test() {
  let mut context = ProgramTest::default().start_with_context().await;
  let default_account = get_account(0);
  airdrop_lamport(&mut context, &default_account.pubkey(),1_000_000_000).await;

  let token_mint_account = get_token_account_named(TokenName::USDT);
  create_token_mint(&mut context, &token_mint_account, 6, &default_account.pubkey(), None).await;

  let account_2 = get_account(2);
  let account_2_token_address = create_associated_token_account(&mut context, &account_2.pubkey(), &token_mint_account.pubkey()).await;
  mint_token(&mut context, &default_account, &token_mint_account.pubkey(), &account_2_token_address, 500_000_000).await;
  let account_2_token_balance = get_token_account_balance(&mut context, &account_2_token_address).await;
  assert!(account_2_token_balance == 500_000_000, "account_2_token_balance invalid");

  let account_3 = get_account(3);
  transfer_token(&mut context, &account_2, &account_2_token_address, &account_3.pubkey(), 100_000_000).await;

  let account_2_token_balance = get_token_account_balance(&mut context, &account_2_token_address).await;
  let account_3_token_addres = get_associated_token_address(&account_3.pubkey(), &token_mint_account.pubkey());
  let account_3_token_balance = get_token_account_balance(&mut context, &account_3_token_addres).await;
  assert!(account_2_token_balance == 400_000_000, "account_2_token_balance invalid");
  assert!(account_3_token_balance == 100_000_000, "account_3_token_balance invalid");
}
