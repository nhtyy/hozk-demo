#![no_main]
// Set the entrypoint for the program
sp1_zkvm::entrypoint!(main);

use merkle_lib::{
    verify_proof,
    proof::{ProofCommitment, ProofInput},
};
use tiny_keccak::{Hasher, Keccak};


fn main() {
    let ProofInput {
        sk,
        nullifier,
        root,
        index,
        path,
    } = sp1_zkvm::io::read();
    
    // Compute the claimed leaf
    let claimed_leaf = {
        let mut hasher = Keccak::v256();
        let mut hash = [0u8; 32];
        hasher.update(sk.as_slice());
        hasher.update(nullifier.as_slice());
        hasher.finalize(&mut hash);
        hash
    };

    // Verify the proof
    assert!(
        verify_proof(root, claimed_leaf.into(), index, path),
        "Proof verification failed."
    );

    // Compute the nullifier hash
    let nullifier_hash: [u8; 32] = {
        let mut hasher = Keccak::v256();
        hasher.update(nullifier.as_slice());
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        hash
    };

    // Create the proof commitment
    let proof_commitment = ProofCommitment {
        nullifier_hash: nullifier_hash.into(),
        root,
    };

    sp1_zkvm::io::commit_slice(&proof_commitment.abi_encode());
}