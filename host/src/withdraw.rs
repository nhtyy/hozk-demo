use alloy::{hex, primitives::FixedBytes, providers::Provider};
use clap::Parser;
use sp1_sdk::{include_elf, Prover, SP1Proof, SP1Stdin};
use sp1_tc_demo_bin::{Args, SP1Tornado::SP1TornadoInstance};
use sp1_tc_demo_merkle_lib::{proof::ProofInput, MerkleTree};
use tiny_keccak::{Hasher, Keccak};

const ELF: &[u8] = include_elf!("sp1-tc-demo-program");

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let prover = sp1_sdk::CpuProver::new();
    let (pk, vk) = prover.setup(ELF);
    let provider = args.provider();

    // Load the note.
    let note = std::fs::read(&args.note_path).expect("Failed to read note");
    assert!(note.len() == 68, "Note must be 68 bytes long");
    let sk: FixedBytes<32> = note[0..32]
        .try_into()
        .expect("Failed to convert sk to bytes");
    let nullifier: FixedBytes<32> = note[32..64]
        .try_into()
        .expect("Failed to convert nullifier to bytes");
    let index: u32 = u32::from_le_bytes(
        note[64..68]
            .try_into()
            .expect("Failed to convert index to bytes"),
    );

    // Get the contract root
    let contract = SP1TornadoInstance::new(args.contract_address, &provider);
    let root = contract.root().call().await.expect("Failed to get root");

    // Build the tree locally
    let leafs = get_all_leafs(&contract).await;
    let tree = MerkleTree::from_leaves(leafs.clone());
    
    println!("Claimed index: {:?}", index);
    println!("Local root: {:?}", tree.root());
    println!("Contract root: {:?}", root);


    let mut stdin = SP1Stdin::new();
    let proof_input = ProofInput {
        sk,
        nullifier,
        root,
        index,
        path: tree.proof(index),
    };
    stdin.write(&proof_input);

    println!("Proving...");
    let proof = prover
        .prove(&pk, &stdin)
        .plonk()
        .run()
        .expect("Failed to prove");

    match &proof.proof {
        SP1Proof::Plonk(plonk) => {
            println!("Plonk vkey hash: {:?}", hex::encode(plonk.plonk_vkey_hash));
        }
        _ => panic!("Unsupported proof type"),
    };

    prover.verify(&proof, &vk).expect("Failed to verify proof locally.");
    let proof_bytes = proof.bytes();
    let nullifier_hash = {
        let mut hasher = Keccak::v256();
        hasher.update(nullifier.as_slice());
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
        hash
    };

    println!("Sending withdrawal...");
    let receipt = contract
        .withdraw(nullifier_hash.into(), proof_bytes.into())
        .send()
        .await
        .expect("Failed to send transaction")
        .get_receipt()
        .await
        .expect("Failed to get receipt");

    println!("Withdraw successful");
    println!("tx hash: {}", receipt.transaction_hash);
}

async fn get_all_leafs<P: Provider>(contract: &SP1TornadoInstance<P>) -> Vec<FixedBytes<32>> {
    let mut leafs = Vec::new();
    let mut events = contract
        .LeafInserted_filter()
        .from_block(0)
        .query()
        .await
        .unwrap();

    events.sort_by_key(|e| e.0.index);

    for (event, _) in events {
        leafs.push(event.leaf);
    }

    leafs
}
