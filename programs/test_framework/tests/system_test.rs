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
  },
  system::{
    airdrop_lamport,
    get_account_balance,
    transfer_lamport,
  },
};

#[tokio::test]
async fn airdrop_lamport_test() {
  let mut context = ProgramTest::default().start_with_context().await;

  let default_account = get_account(0);
  airdrop_lamport(&mut context, &default_account.pubkey(), 1_000_000_000).await;

  let default_account_balance = get_account_balance(&mut context, &default_account.pubkey()).await;
  assert!(default_account_balance == 1_000_000_000, "default_account_balance invalid");
}

#[tokio::test]
async fn transfer_lamport_test() {
  let mut context = ProgramTest::default().start_with_context().await;

  let default_account = get_account(0);
  airdrop_lamport(&mut context, &default_account.pubkey(), 2_000_000_000).await;
  let default_account_balance = get_account_balance(&mut context, &default_account.pubkey()).await;
  assert!(default_account_balance == 2_000_000_000, "default_account_balance invalid");

  let account_1 = get_account(1);
  transfer_lamport(&mut context, &default_account, &account_1.pubkey(), 500_000_000).await;

  let default_account_balance = get_account_balance(&mut context, &default_account.pubkey()).await;
  let account_1_balance = get_account_balance(&mut context, &account_1.pubkey()).await;
  assert!(default_account_balance == 1_500_000_000, "default_account_balance invalid");
  assert!(account_1_balance == 500_000_000, "account_1_balance invalid");
}
