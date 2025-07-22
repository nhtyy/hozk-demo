use alloy::{
    primitives::utils::parse_ether,
};
use clap::Parser;
use rand::Rng;
use sp1_tc_demo_bin::{Args, SP1Tornado};
use tiny_keccak::{Hasher, Keccak};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let provider = args.provider();

    let mut rng = rand::rng();
    let sk = {
        let mut sk = [0u8; 32];
        rng.fill(&mut sk);
        sk
    };

    let nullifier = {
        let mut nullifier = [0u8; 32];
        rng.fill(&mut nullifier);
        nullifier
    };

    let note_hash = {
        let mut hasher = Keccak::v256();
        let mut hash = [0u8; 32];
        hasher.update(sk.as_slice());
        hasher.update(nullifier.as_slice());
        hasher.finalize(&mut hash);
        hash
    };

    let contract = SP1Tornado::new(args.contract_address, &provider);
    let reciept = contract
        .deposit(note_hash.into())
        .value(parse_ether("1").unwrap())
        .send()
        .await
        .expect("Failed to send transaction")
        .get_receipt()
        .await
        .expect("Failed to wait for transaction");

    let index = reciept
        .logs()
        .first()
        .expect("Failed to get log")
        .log_decode::<SP1Tornado::LeafInserted>()
        .expect("Failed to decode log")
        .data()
        .index;

    let note_bytes_raw = {
        let mut empty = [0u8; 68];
        empty[0..32].copy_from_slice(&sk);
        empty[32..64].copy_from_slice(&nullifier);
        empty[64..68].copy_from_slice(&index.to_le_bytes());
        empty
    };

    std::fs::write(&args.note_path, note_bytes_raw).expect("Failed to write note");

    println!("Deposit successful");
}
