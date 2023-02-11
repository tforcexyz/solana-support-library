pub mod framework;
pub mod program;

use anchor_lang::{
  AnchorSerialize
};
use solana_sdk::{
  signer::{
    Signer,
  },
};
use crate::{
  framework::{
    account as test_account,
    merkle_tree::{
      MerkleTree,
    },
  },
};

#[tokio::test]
pub async fn merkle_tree_2_element() {
  let account_1 = test_account::get_account(1);
  let account_2 = test_account::get_account(2);

  let account_1_pubkey = account_1.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let account_2_pubkey = account_2.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let hashes = vec![
    account_1_pubkey,
    account_2_pubkey,
  ];

  let merkle_tree = MerkleTree::new(hashes);
  assert_eq!(merkle_tree.height, 2);
  assert_eq!(
    merkle_tree.root.hash,
    vec![154,35,116,72,131,150,251,193,18,224,94,216,241,48,18,205,234,120,143, 216, 214,195,129,10,102,79,173,80,39,116,215,243]
  );
}

#[tokio::test]
pub async fn merkle_tree_2_element_reverse() {
  let account_1 = test_account::get_account(1);
  let account_2 = test_account::get_account(2);

  let account_1_pubkey = account_1.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let account_2_pubkey = account_2.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let hashes = vec![
    account_2_pubkey,
    account_1_pubkey,
  ];

  let merkle_tree = MerkleTree::new(hashes);
  assert_eq!(merkle_tree.height, 2);
  assert_eq!(
    merkle_tree.root.hash,
    vec![154,35,116,72,131,150,251,193,18,224,94,216,241,48,18,205,234,120,143, 216, 214,195,129,10,102,79,173,80,39,116,215,243]
  );
}

#[tokio::test]
pub async fn merkle_tree_4_element() {
  let account_1 = test_account::get_account(1);
  let account_2 = test_account::get_account(2);
  let account_3 = test_account::get_account(3);
  let account_4 = test_account::get_account(4);

  let account_1_pubkey = account_1.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let account_2_pubkey = account_2.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let account_3_pubkey = account_3.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let account_4_pubkey = account_4.pubkey().to_bytes()
      .try_to_vec().unwrap();
  let hashes = vec![
    account_1_pubkey,
    account_2_pubkey,
    account_3_pubkey,
    account_4_pubkey,
  ];

  let merkle_tree = MerkleTree::new(hashes);
  assert_eq!(merkle_tree.height, 3);
  assert_eq!(
    merkle_tree.root.hash,
    vec![137,105,47,126,64,187,197,93,109,172,134,23,239,171,68,227,185,192,200,229,60,22,180,70,49,135,3,62,39,108,146,22]
  );
}

#[tokio::test]
pub async fn merkle_tree_4_element_reverse() {
  let account_1 = test_account::get_account(1);
  let account_2 = test_account::get_account(2);
  let account_3 = test_account::get_account(3);
  let account_4 = test_account::get_account(4);

  let account_1_pubkey = account_1.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let account_2_pubkey = account_2.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let account_3_pubkey = account_3.pubkey().to_bytes()
    .try_to_vec().unwrap();
  let account_4_pubkey = account_4.pubkey().to_bytes()
      .try_to_vec().unwrap();
  let hashes = vec![
    account_4_pubkey,
    account_3_pubkey,
    account_2_pubkey,
    account_1_pubkey,
  ];

  let merkle_tree = MerkleTree::new(hashes);
  assert_eq!(merkle_tree.height, 3);
  assert_eq!(
    merkle_tree.root.hash,
    vec![137,105,47,126,64,187,197,93,109,172,134,23,239,171,68,227,185,192,200,229,60,22,180,70,49,135,3,62,39,108,146,22]
  );
}
