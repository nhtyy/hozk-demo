# Succinct <> HOZK: Tornado Cash Demo

> [!CAUTION] 
> 
> This repository is intended for education purposes only
> this repoistory is not audited and is provided as is and is not intended for production use.

## Requirements

Rust: Install the [Rust compiler](https://www.rust-lang.org/tools/install)

SP1: Install the [cargo prove](https://docs.succinct.xyz/docs/sp1/getting-started/install) CLI tool.

Forge: Install [Forge](https://getfoundry.sh/introduction/installation).

Go: Install [Go](https://go.dev/doc/install)

## Usage

First run:
`pushd contracts && forge install && popd`

Start two terminals, in the first run
`anvil`

First deploy the protocol in the second terminal:
`cargo run --bin deploy --release`

In the second you can create deposits via:
`cargo run --bin deposit --release`

and you can initiate a withdraw with:
`cargo run --bin withdraw --release`