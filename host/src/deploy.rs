use sp1_sdk::{CpuProver, HashableKey, Prover};
use sp1_tc_demo_bin::{ELF, Args};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let prover = CpuProver::new();
    let (_, vk) = prover.setup(ELF);
    println!("vk: {:?}", vk.bytes32());

    let forge_install_status = std::process::Command::new("forge")
        .arg("install")
        .current_dir("contracts")
        .status()
        .expect("failed to run forge install");

    if !forge_install_status.success() {
        panic!("forge install failed");
    }

    // Deploy to anvil using the default private key.
    let forge_status = std::process::Command::new("forge")
        .args(&[
            "script",
            "script/Deploy.s.sol",
            "--private-key",
            args.private_key.as_str(),
            "--sender",
            args.signer().address().to_string().as_str(),
            "--broadcast",
            "--rpc-url",
            args.rpc_url.as_str(),
        ])
        .current_dir("contracts")
        .env("SP1_PROGRAM_VKEY", vk.bytes32())
        .status()
        .expect("failed to run forge");

    if !forge_status.success() {
        panic!("forge script failed");
    }
}
