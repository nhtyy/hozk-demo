use serde::{Deserialize, Serialize};
use alloy::primitives::FixedBytes;
use alloy::sol_types::SolType;

use crate::TREE_DEPTH;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProofInput {
    pub sk: FixedBytes<32>,
    pub nullifier: FixedBytes<32>,
    pub root: FixedBytes<32>,
    pub index: u32,
    pub path: [FixedBytes<32>; TREE_DEPTH],
}

alloy::sol! {
    struct ProofCommitment {
        bytes32 nullifier_hash;
        bytes32 root;
    }
}

impl ProofCommitment {
    pub fn abi_encode(&self) -> Vec<u8> {
        <Self as SolType>::abi_encode(self)
    }
}
