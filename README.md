# Pitch
This repo is focused on using Risc-Zero to generate Zero Knowledge Proofs using Rust and Foundry to prove only vetted Security Reviewers with certain score reqirements are allowed to attest to a contract. 

# Descriptions
Paid company provides funds for security reviewers to aynonmously submit reviews on behalf of the company. Company decides whether the security reviewers are private or public. 

# Installation
[Reference](https://github.com/risc0/bonsai-foundry-template/tree/main?tab=readme-ov-file)

Clone this repo

Install rust and foundry

Use this command to get rust/risczero dependancies

```
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

Get started with downloading dependancies

```
cargo build
```
```
forge build
```

For making this auditing review, forge test and cargo test were not re-done to accomidate the new files.

Build your .env to look like below. We will be copy-pasting this into terminal. Shortens deployment commands. 

```
export BONSAI_API_KEY="YOUR_API_KEY"
export BONSAI_API_URL="BONSAI_URL"
export INFURA_API_KEY="YOUR_INFURA_API_KEY"
export ETH_WALLET_PRIVATE_KEY="YOUR_WALLET_PRIVATE_KEY" 
```

## Prepare for Deployment

```
cargo build
```

Get that deployment in!

```
forge script script/Deploy.s.sol --rpc-url https://sepolia.infura.io/v3/${INFURA_API_KEY} --broadcast --verify -vvvv
```

Record your ValidReviewer address (reference ours [here](#deployments))

Use this command to publish data to the blockchain. Only approved users with certain score inputs will be approved. Review [this](https://github.com/PizzaHi5/ZTrust-ZK/blob/main/methods/guest/src/bin/is_valid_hack.rs)

```
cargo run --bin publisher -- \
    --chain-id=11155111 \
    --rpc-url=https://sepolia.infura.io/v3/${INFURA_API_KEY} \
    --contract=0x294b9b0135eE74BC31c3E166Ab56176F5962288a \
    --score 4 1
```

# Deployments
Sepolia Deployed Contracts:
ValidReviewer = 0x4f442938F71EC57656957533A943eeE721499709
RiscZeroGroth16Verifier = 0x4f433AFCF539f910a4570645805323E890C77924