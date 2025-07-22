use alloy::primitives::FixedBytes;
use tiny_keccak::{Hasher, Keccak};

pub mod proof;

const TREE_DEPTH: usize = 32;

lazy_static::lazy_static! {
    static ref ZERO_HASHES: [FixedBytes<32>; TREE_DEPTH] = {
        let mut hashes = [FixedBytes::ZERO; TREE_DEPTH];
        let mut cur = FixedBytes::ZERO;

        for lvl in 0..TREE_DEPTH {
            cur = keccak_pair(&cur, &cur);
            hashes[lvl] = cur.clone().into();
        }
        hashes
    };
}

pub struct MerkleTree {
    pub leaves: Vec<FixedBytes<32>>,
}

impl MerkleTree {
    pub fn from_leaves(leaves: Vec<FixedBytes<32>>) -> Self {
        Self { leaves }
    }

    pub fn root(&self) -> FixedBytes<32> {
        if self.leaves.is_empty() {
            return ZERO_HASHES[TREE_DEPTH - 1]; // root of an empty tree
        }

        let mut layer = self.leaves.to_vec();
        let mut lvl = 0;
        while layer.len() > 1 {
            let mut next = Vec::with_capacity((layer.len() + 1) / 2);
            for chunk in layer.chunks(2) {
                let left = chunk[0];
                let right = if chunk.len() == 2 {
                    chunk[1]
                } else {
                    ZERO_HASHES[lvl]
                };
                next.push(keccak_pair(&left, &right));
            }
            layer = next;
            lvl += 1;
        }

        // 2️⃣ hash with zeros for all *remaining* empty levels
        let mut hash = layer[0];
        for upper in lvl..TREE_DEPTH {
            hash = keccak_pair(&hash, &ZERO_HASHES[upper]);
        }
        hash
    }

    pub fn proof(&self, index: u32) -> [FixedBytes<32>; TREE_DEPTH] {
        if index as usize >= self.leaves.len() {
            panic!("index out of bounds");
        }

        // Work layer-by-layer, collecting sibling at each height.
        let mut path = [FixedBytes::ZERO; TREE_DEPTH];
        let mut idx = index;
        let mut layer = self.leaves.clone();

        for lvl in 0..TREE_DEPTH {
            let sib_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };

            let sibling = if sib_idx < layer.len() as u32 {
                layer[sib_idx as usize]
            } else {
                ZERO_HASHES[lvl]
            };
            path[lvl] = sibling;

            // collapse layer
            let mut next = Vec::with_capacity((layer.len() + 1) / 2);
            for chunk in layer.chunks(2) {
                let left = chunk[0];
                let right = if chunk.len() == 2 {
                    chunk[1]
                } else {
                    ZERO_HASHES[lvl]
                };
                next.push(keccak_pair(&left, &right));
            }
            layer = next;
            idx /= 2;
        }

        path
    }
}

pub fn verify_proof(
    root: FixedBytes<32>,
    leaf: FixedBytes<32>,
    index: u32,
    proof: [FixedBytes<32>; TREE_DEPTH],
) -> bool {
    let mut hash = leaf;
    let mut idx = index;

    for lvl in 0..TREE_DEPTH {
        let sibling = proof[lvl];
        let (left, right) = if idx % 2 == 0 {
            (hash, sibling)
        } else {
            (sibling, hash)
        };
        hash = keccak_pair(&left, &right);
        idx /= 2;
    }

    hash == root
}

/// 32-byte Keccak(left ∥ right)
fn keccak_pair(l: &[u8; 32], r: &[u8; 32]) -> FixedBytes<32> {
    let mut k = Keccak::v256();
    k.update(l);
    k.update(r);
    let mut out = [0u8; 32];
    k.finalize(&mut out);
    out.into()
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_merkle_root_is_expected() {
        let leaves = vec![
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000003").unwrap(),
        ];

        // Note these values are taken from the solidity implementation
        let expected_root = vec![
            FixedBytes::from_str("0xe144de6e6c738abf71e1cd2d2ea747fd9327746b6fa4d469c3c5a797e6a74786").unwrap(),
            FixedBytes::from_str("0x4d800872b3b72b37b4180dc29d249abc309ba773a0736b0811661ef0e64e3d67").unwrap(),
            FixedBytes::from_str("0xe257a1fa6503077313c3c33dcbf24a24ae33e6f69f54f080b2ded61281c5207e").unwrap(),
        ];

        let tree = MerkleTree::from_leaves(leaves[..1].to_vec());
        assert_eq!(tree.root(), expected_root[0]);

        let tree = MerkleTree::from_leaves(leaves[..2].to_vec());
        assert_eq!(tree.root(), expected_root[1]);

        let tree = MerkleTree::from_leaves(leaves[..3].to_vec());
        assert_eq!(tree.root(), expected_root[2]);
    }

    #[test]
    fn test_proof_verification() {
        let leaves = vec![
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000003").unwrap(),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());

        let index: u32 = 1;
        let proof = tree.proof(1);
        let root = tree.root();

        println!("root: {:?}", root);
        println!("leaf: {:?}", leaves[index as usize]);
        println!("proof: {:?}", proof);

        assert!(verify_proof(root, leaves[index as usize], index, proof));
    }

    #[test]
    fn test_bad_proof_does_not_verify() {
        let leaves = vec![
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(),
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap(),
            FixedBytes::from_str("0x0000000000000000000000000000000000000000000000000000000000000003").unwrap(),
        ];

        let tree = MerkleTree::from_leaves(leaves.clone());

        let index: u32 = 1;
        let proof = tree.proof(1);
        let root = tree.root();

        // Passing the wrong leaf should not verify
        assert!(!verify_proof(root, leaves[index as usize + 1], index, proof));

        // Passing the wrong index should not verify
        assert!(!verify_proof(root, leaves[index as usize], index + 1, proof));

        // Passing the wrong proof should not verify
        assert!(!verify_proof(root, leaves[index as usize], index, [FixedBytes::ZERO; TREE_DEPTH]));
    }
}   