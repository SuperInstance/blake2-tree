# BLAKE2 Tree

**A Rust library for building Merkle trees using BLAKE2b-style compression** — provides cryptographic hash commitments over sets of data with efficient inclusion proofs.

## Why It Matters

A Merkle tree (named after Ralph Merkle, 1979) is a binary tree where each leaf is the hash of a data block, and each internal node is the hash of its two children. The root hash commits to the entire dataset: changing any single leaf changes every node on the path to the root.

Merkle trees power:
- **Blockchains** — Bitcoin and Ethereum use Merkle trees to commit to transactions in blocks, enabling lightweight SPV (Simple Payment Verification) clients to verify inclusion without downloading the entire chain
- **Distributed storage** — IPFS, Cassandra, and DynamoDB use Merkle trees for anti-entropy synchronization
- **Certificate transparency** — Google's CT logs use Merkle trees to make certificate issuance auditable
- **Version control** — Git's object store is a content-addressed Merkle DAG

BLAKE2 is a cryptographic hash function faster than MD5 and SHA-1, yet as secure as SHA-3. It was designed by Jean-Philippe Aumasson et al. (2013) and is widely deployed in Zcash, WireGuard, and Argon2.

## How It Works

**Hash compression**: The library implements a simplified BLAKE2b-style compression function. The state starts with SHA-256-like initialization constants (the first 8 words of the BLAKE2 IV). Each 64-byte block is absorbed: block words are added to state words, rotations are applied, and the state is mixed. This is a pedagogical implementation, not suitable for production cryptography.

**Merkle tree construction** (`merkle_root`):
1. Hash each data block (`leaves`) using `hash_block` to produce 32-byte leaf hashes
2. Pair adjacent hashes, concatenate them into a 64-byte block, and hash to produce parent nodes
3. If there's an odd number of nodes, duplicate the last one
4. Repeat until only the root remains

**Properties**: The root hash is O(1) size regardless of dataset size. An inclusion proof for any leaf is O(log n) hashes. Changing one leaf invalidates the root.

## Quick Start

```rust
use blake2_tree::{hash_block, merkle_root};

// Hash a single block
let h = hash_block(b"my data block");
assert_eq!(h.len(), 32);

// Build a Merkle tree over multiple data blocks
let leaves: Vec<&[u8]> = vec![b"tx1", b"tx2", b"tx3", b"tx4"];
let root = merkle_root(&leaves);
println!("Merkle root: {:02x?}", root);

// Changing any leaf changes the root
let modified: Vec<&[u8]> = vec![b"tx1", b"DIFFERENT", b"tx3", b"tx4"];
let root2 = merkle_root(&modified);
assert_ne!(root, root2);
```

## API

- **`hash_block(data)` → `[u8; 32]`** — BLAKE2b-style hash of a single block
- **`merkle_root(leaves)` → `[u8; 32]`** — Root hash of the Merkle tree

## Architecture Notes

Provides the commitment-tree primitive for SuperInstance data integrity verification. The Merkle tree construction is used in fleet state verification, log tamper-detection, and distributed consistency checks. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
