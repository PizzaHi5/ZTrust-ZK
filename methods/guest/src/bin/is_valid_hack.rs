// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Read;

use alloy_primitives::{U256, Address}; //address is also useful
use alloy_sol_types::{sol, SolValue, SolType}; //SolInterface, SolStruct
use risc0_zkvm::guest::env;
/*
    
*/
fn main() {
    // Read the input data for this application.
    type InputData = sol!((uint256, uint256, address));
    
    // Read
    let mut input_bytes = Vec::<u8>::new();
    //let (score, code_line) = InputData::abi_decode(&journal, true).context("decoding journal data")?;
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Decode and parse the input
    let (score, code_line, sender) = InputData::abi_decode(&input_bytes, true).unwrap();
    //let number = <U256>::abi_decode(&input_bytes, true).unwrap();

    // Run the computation
    // ZKP portion
    let max_score = <U256>::from(6);
    assert!(score.lt(&max_score), "ERROR: Score > MaxScore");

    // Security Reviewer 1
    let checksummed = "0x096f6A2b185d63D942750A2D961f7762401cbA17";
    //let expected = address!("0x096f6A2b185d63D942750A2D961f7762401cbA17");
    let address = Address::parse_checksummed(checksummed, None).expect("valid checksum");
    let mut addresses: Vec<Address> = Vec::new();

    addresses.push(address);
    
    let mut is_valid_address = false;
    for address in addresses.iter() {
        if &sender == address {
            is_valid_address = true;
        }
    }
    assert!(is_valid_address, "ERROR: Invalid User");
    //future: address interacting is an approved address

    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    // commit_slice gets pushed to the smart contract, public output
    env::commit_slice(score.abi_encode().as_slice());
    env::commit_slice(code_line.abi_encode().as_slice());
}
