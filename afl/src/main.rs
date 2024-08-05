// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use snarkvm::prelude::{MainnetV0 as CurrentNetwork, Program};

use afl;

fn main() {
    afl::fuzz!(|program: Program<CurrentNetwork>| {
        // // Initialize the VM.
        // if let Ok(vm) = VM::<CurrentNetwork>::new() {
        //     // Initialize the RNG.
        //     let rng = &mut test_crypto_rng();

        //     // Deploy.
        //     if let Ok(transaction) = vm.deploy(&program, rng) {
        //         // Verify.
        //         vm.verify(&transaction);
        //     }
        // }
    });
}
