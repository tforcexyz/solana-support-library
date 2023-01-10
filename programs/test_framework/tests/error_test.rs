pub mod client;
pub mod framework;

use solana_sdk::{
  pubkey::{
    Pubkey,
  },
  signer::{
    Signer,
  },
};

use crate::framework::{
  account::{
    get_account,
  },
  context::{
    assert_transaction_result,
    create_context,
    process_transaction2,
  },
  system::{
    airdrop_lamport,
  },
};

#[tokio::test]
async fn announce_test() {
  let mut context = create_context(
    &[
      ("test_framework", Pubkey::new_from_array([6, 185, 135, 63, 199, 196, 100, 20, 13, 120, 6, 157, 78, 207, 62, 49, 88, 159, 133, 53, 153, 195, 152, 208, 254, 106, 194, 190, 138, 172, 180, 2])),
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
  assert_transaction_result(
    txn_result,
    false,
    &"custom program error: 0x1770",
  );
}
