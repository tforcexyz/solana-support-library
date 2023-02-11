import {
  assert
} from 'chai';
import {
  MerkleTreeKeccak
} from '../src';
import {
  TestAccountService
} from '../src/config';

describe('merkle_tree_test', function(){

  it('merkle_tree_2_element', async function() {
    const account1 = await TestAccountService.getAccount(1);
    const account2 = await TestAccountService.getAccount(2);

    const hashes = [
      account1.publicKey.toBuffer(),
      account2.publicKey.toBuffer(),
    ]

    const merkleTree = new MerkleTreeKeccak(hashes);
    assert(
      merkleTree.root().hash.equals(
        Uint8Array.from([154,35,116,72,131,150,251,193,18,224,94,216,241,48,18,205,234,120,143, 216, 214,195,129,10,102,79,173,80,39,116,215,243])
      )
    );
  });

  it('merkle_tree_2_element_reverse', async function() {
    const account1 = await TestAccountService.getAccount(1);
    const account2 = await TestAccountService.getAccount(2);

    const hashes = [
      account2.publicKey.toBuffer(),
      account1.publicKey.toBuffer(),
    ]

    const merkleTree = new MerkleTreeKeccak(hashes);
    assert(
      merkleTree.root().hash.equals(
        Uint8Array.from([154,35,116,72,131,150,251,193,18,224,94,216,241,48,18,205,234,120,143, 216, 214,195,129,10,102,79,173,80,39,116,215,243])
      )
    );
  });

  it('merkle_tree_4_element', async function() {
    const account1 = await TestAccountService.getAccount(1);
    const account2 = await TestAccountService.getAccount(2);
    const account3 = await TestAccountService.getAccount(3);
    const account4 = await TestAccountService.getAccount(4);

    const hashes = [
      account1.publicKey.toBuffer(),
      account2.publicKey.toBuffer(),
      account3.publicKey.toBuffer(),
      account4.publicKey.toBuffer(),
    ]

    const merkleTree = new MerkleTreeKeccak(hashes);
    assert(
      merkleTree.root().hash.equals(
        Uint8Array.from([137,105,47,126,64,187,197,93,109,172,134,23,239,171,68,227,185,192,200,229,60,22,180,70,49,135,3,62,39,108,146,22])
      )
    );
  });

  it('merkle_tree_4_element_reverse', async function() {
    const account1 = await TestAccountService.getAccount(1);
    const account2 = await TestAccountService.getAccount(2);
    const account3 = await TestAccountService.getAccount(3);
    const account4 = await TestAccountService.getAccount(4);

    const hashes = [
      account4.publicKey.toBuffer(),
      account3.publicKey.toBuffer(),
      account2.publicKey.toBuffer(),
      account1.publicKey.toBuffer(),
    ]

    const merkleTree = new MerkleTreeKeccak(hashes);
    assert(
      merkleTree.root().hash.equals(
        Uint8Array.from([137,105,47,126,64,187,197,93,109,172,134,23,239,171,68,227,185,192,200,229,60,22,180,70,49,135,3,62,39,108,146,22])
      )
    );
  });
})
