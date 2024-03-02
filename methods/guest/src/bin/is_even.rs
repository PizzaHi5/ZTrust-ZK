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

extern crate alloc;
use alloy_primitives::U256;
use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;

/*
    GOAL: Prove a malicious state change has occurred

    Input: Current State
    Input: Expected State

    If expect != output, proof fails

    ANOTHER TRY
    Input: Calldata
    Input: Function Signature
    Input: Address

    Run_simulation
*/

fn main() {
    // Read the input data for this application.

    // Read
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Decode and parse the input
    let number = <U256>::abi_decode(&input_bytes, true).unwrap();
    //let str: String = <String>::abi_en
    //env::stdin().read_words("words")
    // Run the computation.
    // In this case, asserting that the provided number is even.
    assert!(number.bit(0) == false, "number is not even");



    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.

    // commit_slice gets pushed to the smart contract, public output
    env::commit_slice(number.abi_encode().as_slice());
}
