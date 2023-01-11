pub mod client;
pub mod framework;

use solana_sdk::{
  signer::{
    Signer,
  },
};
use crate::framework::{
  account::{
    get_account,
  },
  context::{
    assert_instruction_error,
    create_context,
    process_transaction2,
  },
  system::{
    airdrop_lamport,
  },
};
use test_framework::{
  ID as PROGRAM_ID,
};

#[tokio::test]
async fn announce_test() {
  let mut context = create_context(
    &[
      ("test_framework", PROGRAM_ID),
    ],
  ).await;

  let default_account = get_account(0);
  airdrop_lamport(&mut context, &default_account.pubkey(), 10_000_000).await;

  let announce_ixn = client::create_announce_instruction(
    &default_account.pubkey(),
    b"An announcement should be short and must not exceed 64 characters.".to_vec(),
  );

  let txn_result = process_transaction2(&mut context, &default_account, &[announce_ixn], &[&default_account])
    .await;
  assert_instruction_error(
    txn_result,
    0,
    6000,
  );
}
