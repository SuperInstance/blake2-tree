# blake2-tree

**Binary Merkle tree construction with BLAKE2b-style compression — deterministic tamper-evident data structures.**

A Merkle tree is a binary hash tree where leaf nodes are hashes of data blocks, and interior nodes are hashes of their children. The **root hash** serves as a compact cryptographic commitment to the entire dataset: any change to any leaf propagates up to the root. This crate implements Merkle trees using a BLAKE2b-inspired compression function for the underlying hash primitive.

## Why It Matters

Merkle trees are the backbone of verifiable data systems:

- **Blockchains** — Bitcoin uses Merkle trees to commit all transactions in a block into a single 32-byte header field. SPV (Simplified Payment Verification) clients verify transaction inclusion via Merkle proofs without downloading the full block.
- **Distributed storage** — IPFS, Cassandra, and Apache Cassandra use Merkle trees to detect inconsistent replicas via **Merkle tree exchange** (anti-entropy repair).
- **Version control** — Git's object store is a content-addressed Merkle DAG. Each commit hashes its tree, which hashes its subtrees, recursively.
- **Certificate transparency** — CT logs use Merkle trees to provide append-only proofs that certificates haven't been backdated.

The key property: **O(log n) inclusion proofs**. To prove that a leaf is part of an n-leaf tree, you need only ⌈log₂ n⌉ sibling hashes, not the entire tree.

## How It Works

### Hash Function

The crate uses a simplified BLAKE2b-inspired compression function. (Note: this is a teaching implementation; production use requires the full BLAKE2b spec with its 12-round G function, message schedule, and counter/offset words.)

The compression function operates on a 512-bit state (8 × u64 words) initialized with SHA-256 IV constants:

> IV₀ = 0x6a09e667f3bcc908 ( fractional part of √2 )
> IV₁ = 0xbb67ae8584caa73b ( fractional part of √3 )
> ... (SHA-256 initialization vector)

Each 512-bit (64-byte) block is absorbed into the state via addition and rotation:

```
for i in 0..8:
    v[i] += block_word[i]
    v[i] = rotate_left(v[i], 32)
```

The output is the first 256 bits (4 words) of the resulting state, serialized little-endian.

### Merkle Tree Construction

Given n leaves (data blocks), construction proceeds bottom-up:

```
Layer 0 (leaves):   h₀ = hash(leaf₀),  h₁ = hash(leaf₁),  ...
Layer 1:            h₀₁ = hash(h₀ || h₁),  h₂₃ = hash(h₂ || h₃),  ...
Layer 2:            h₀₃ = hash(h₀₁ || h₂₃), ...
...
Root:               Single hash = Merkle root
```

**Odd node handling**: If a layer has an odd number of nodes, the last node is **duplicated** (hashed with itself). This is the Bitcoin convention. Alternative: pad with a zero-hash.

### Inclusion Proofs

To prove leaf i is in the tree, provide the ⌈log₂ n⌉ sibling hashes from leaf to root:

```
path = []
node = leaf_i_hash
for layer in 0..height:
    sibling = left(node) if node is right child, else right(node)
    path.push(sibling)
    node = hash(node || sibling) or hash(sibling || node)
```

Verification: reconstruct the root from the leaf + proof path and compare.

### Properties

1. **Collision resistance**: If the hash function is collision-resistant (output ≥ 256 bits), finding two datasets with the same root requires ~2¹²⁸ work.
2. **Tamper-evidence**: Modifying any single leaf changes the root with probability 1 - 2⁻²⁵⁶.
3. **Determinism**: The same input always produces the same root.
4. **Incremental updates**: Updating one leaf requires re-hashing only O(log n) nodes.

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `hash_block(data)` | O(1) | O(1) |
| `merkle_root(n leaves)` | O(n) | O(n) |
| Inclusion proof (tree of n leaves) | O(log n) | O(log n) |
| Proof verification | O(log n) | O(log n) |
| Update one leaf | O(log n) | O(log n) |

Space for the tree: O(n) total nodes (2n - 1 for a full binary tree).

## Quick Start

```rust
use blake2_tree::{hash_block, merkle_root};

// Hash a single block
let h = hash_block(b"my data block");
assert_eq!(h.len(), 32);

// Deterministic
assert_eq!(hash_block(b"my data block"), h);

// Build a Merkle tree over multiple leaves
let leaves: Vec<&[u8]> = vec![b"transaction1", b"transaction2", b"transaction3"];
let root = merkle_root(&leaves);
println!("Merkle root: {:02x?}", root);

// Any change to any leaf changes the root
let leaves2: Vec<&[u8]> = vec![b"transaction1", b"MODIFIED", b"transaction3"];
let root2 = merkle_root(&leaves2);
assert_ne!(root, root2);

// Empty tree has a defined root
let empty_root = merkle_root(&[]);
assert_ne!(empty_root, [0u8; 32]);
```

## API

- **`hash_block(data: &[u8]) → [u8; 32]`** — Hash a single data block (padded to 64 bytes). Uses BLAKE2b-inspired compression from SHA-256 IV.
- **`merkle_root(leaves: &[&[u8]]) → [u8; 32]`** — Build Merkle tree and return root hash. Odd nodes duplicated (Bitcoin convention). Empty input hashes the string "empty".

## Architecture Notes

The γ+η=C identity: γ (generative capacity) is the size of the data set the tree can commit to — unbounded, since any number of leaves produce a fixed-size 32-byte root. η (evaluative depth) is the hash function's cryptographic strength — 256-bit output provides 128-bit collision resistance. C = verifiable integrity: the system can detect any tampering with O(n) data using a 32-byte commitment, and prove any leaf's inclusion in O(log n) space. The γ/η tradeoff is hash function choice: stronger hashing (higher η) is slower but more secure.

## References

1. Merkle, R. (1979). "Secrecy, Authentication, and Public Key Systems." PhD Thesis, Stanford. — Original Merkle tree proposal.
2. Aumasson, J.-P. et al. (2013). "BLAKE2: Simpler, Smaller, Fast as MD5." *SAC 2013*. — BLAKE2b specification.
3. Nakamoto, S. (2008). *Bitcoin Whitepaper*, §4. — Merkle trees for transaction commitment.
4. Benet, J. (2014). *IPFS Whitepaper*. — Content-addressed Merkle DAG design.

## License

MIT
