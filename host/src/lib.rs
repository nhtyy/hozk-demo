use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use clap::Parser;
use sp1_sdk::include_elf;
use std::path::PathBuf;

// Define the bindings for the contract.
sol! {
    #[sol(rpc)]
    contract SP1Tornado {
        function deposit(bytes32 noteHash) public payable;
        function withdraw(bytes32 _nullifierHash, bytes calldata proof) public;
        function root() public view returns (bytes32);

        event LeafInserted(bytes32 leaf, uint32 index);
    }
}

/// The ELF binary for the program
pub const ELF: &[u8] = include_elf!("sp1-tc-demo-program");

#[derive(Parser)]
pub struct Args {
    /// The address of the contract.
    #[clap(long, default_value = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9")]
    pub contract_address: Address,

    #[clap(long, default_value = "http://localhost:8545")]
    pub rpc_url: String,

    #[clap(
        long,
        default_value = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
    )]
    pub signer: String,

    #[clap(long, default_value = "note.bin")]
    pub note_path: PathBuf,
}

impl Args {
    pub fn signer(&self) -> PrivateKeySigner {
        self.signer.parse().expect("Invalid signer")
    }

    pub fn provider(&self) -> impl Provider {
        ProviderBuilder::new()
            .wallet(self.signer())
            .connect_http(self.rpc_url.parse().expect("Invalid RPC URL"))
    }
}
