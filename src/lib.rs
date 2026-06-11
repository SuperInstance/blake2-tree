//! Blake2-tree: merkle tree hashing using BLAKE2b-style compression
//!
//! Provides a simple merkle tree construction and verification.

/// Simple BLAKE2b-inspired compression for demonstration.
fn compress(state: &[u64; 8], block: &[u8; 64]) -> [u64; 8] {
    let mut v = *state;
    for (i, chunk) in block.chunks(8).enumerate() {
        if i >= 8 { break; }
        let val = u64::from_le_bytes(chunk[..8].try_into().unwrap_or([0u8; 8]));
        v[i] = v[i].wrapping_add(val);
        v[i] = v[i].rotate_left(32);
    }
    v
}

/// Compute a simple hash of a 64-byte block from a zero state.
pub fn hash_block(block: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 64];
    let len = block.len().min(64);
    padded[..len].copy_from_slice(&block[..len]);
    let state: [u64; 8] = [
        0x6a09e667f3bcc908, 0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b, 0xa54ff53a5f1d36f1,
        0x510e527fade682d1, 0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b, 0x5be0cd19137e2179,
    ];
    let result = compress(&state, &padded);
    let mut out = [0u8; 32];
    for (i, &v) in result[..4].iter().enumerate() {
        out[i * 8..(i + 1) * 8].copy_from_slice(&v.to_le_bytes());
    }
    out
}

/// Build a simple binary merkle tree and return the root hash.
pub fn merkle_root(leaves: &[&[u8]]) -> [u8; 32] {
    if leaves.is_empty() {
        return hash_block(b"empty");
    }
    let mut layer: Vec<[u8; 32]> = leaves.iter().map(|l| hash_block(l)).collect();
    while layer.len() > 1 {
        let mut next = Vec::new();
        let mut i = 0;
        while i < layer.len() {
            let left = layer[i];
            let right = if i + 1 < layer.len() { layer[i + 1] } else { left };
            let mut combined = [0u8; 64];
            combined[..32].copy_from_slice(&left);
            combined[32..].copy_from_slice(&right);
            next.push(hash_block(&combined));
            i += 2;
        }
        layer = next;
    }
    layer[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic() {
        let a = hash_block(b"test data");
        let b = hash_block(b"test data");
        assert_eq!(a, b);
    }

    #[test]
    fn test_merkle_single() {
        let root = merkle_root(&[b"leaf"]);
        assert_ne!(root, [0u8; 32]);
    }
}
