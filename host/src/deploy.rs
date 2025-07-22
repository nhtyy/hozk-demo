use sp1_sdk::{CpuProver, HashableKey, Prover};
use sp1_tc_demo_bin::ELF;

fn main() {
    let prover = CpuProver::new();
    let (_, vk) = prover.setup(ELF);
    println!("vk: {:?}", vk.bytes32());

    // Deploy to anvil using the default private key.
    let forge_status = std::process::Command::new("forge")
        .args(&[
            "script",
            "script/Deploy.s.sol",
            "--private-key=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            "--sender=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
            "--broadcast",
            "--rpc-url=http://localhost:8545",
        ])
        .current_dir("contracts")
        .env("SP1_PROGRAM_VKEY", vk.bytes32())
        .status()
        .expect("failed to run forge");

    if !forge_status.success() {
        panic!("forge script failed");
    }
}
