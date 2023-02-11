pub mod framework;
pub mod program;

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
    process_transaction2,
  },
  system::{
    airdrop_lamport,
  },
};
use crate::program::{
  client,
  test_context::{
    create_test_context,
  },
};

#[tokio::test]
async fn announce_test() {
  let mut context = create_test_context().await;

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
