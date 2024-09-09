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

use std::{
    panic::AssertUnwindSafe,
    str::{self, FromStr},
};

use snarkvm::prelude::{Address, MainnetV0 as CurrentNetwork, PrivateKey, Process, Program, TestRng, ValueType};
use snarkvm::synthesizer::program::StackProgram;

type CurrentAleo = snarkvm::circuit::network::AleoV0;

fn main() {
    // Prepare some objects that don't need to be recreated.
    let rng = &mut TestRng::fixed(7777777);
    let private_key = PrivateKey::<CurrentNetwork>::new(rng).unwrap();
    let burner_private_key = PrivateKey::new(rng).unwrap();
    let burner_address = Address::try_from(&burner_private_key).unwrap();
    // The way the Process is used in the fuzzer is unwind-safe.
    let mut process = AssertUnwindSafe(Process::load().unwrap());

    afl::fuzz!(|input: &[u8]| {
        // Convert the input bytes to a string.
        let Ok(program_string) = str::from_utf8(input) else {
            return;
        };

        // Parse the program string as an Aleo program.
        let Ok(program) = Program::<CurrentNetwork>::from_str(&program_string) else {
            return;
        };

        // At least a single function must be present.
        if program.functions().is_empty() {
            return;
        };

        // Reset the process to its initial state.
        process.reset();

        // Attempt to introduce the input program.
        if process.add_program(&program).is_err() {
            return;
        }

        if cfg!(any(feature = "deploy", feature = "verify_deployment", feature = "full")) {
            // Attempt to deploy the program.
            let Ok(deployment) = process.deploy::<CurrentAleo, _>(&program, rng) else {
                return;
            };

            if cfg!(any(feature = "verify_deployment", feature = "full")) {
                // Attempt to verify the deployment.
                if process.verify_deployment::<CurrentAleo, _>(&deployment, rng).is_err() {
                    return;
                }
            }
        }

        if cfg!(any(feature = "authorize", feature = "execute", feature = "full")) {
            // Process all the functions.
            for function in program.functions().values() {
                // Sample inputs applicable to the given functions.
                let input_types = function.input_types();
                let stack = process.get_stack(program.id()).unwrap();
                let Ok(inputs) = input_types
                    .iter()
                    .map(|input_type| match input_type {
                        ValueType::ExternalRecord(locator) => {
                            let stack = stack.get_external_stack(locator.program_id())?;
                            stack.sample_value(&burner_address, &ValueType::Record(*locator.resource()), rng)
                        }
                        _ => {
                            stack.sample_value(&burner_address, &input_type, rng)
                        }
                    })
                    .collect::<Result<Vec<_>, _>>() else
                {
                    return;
                };

                // Attempt to authorize the function with its inputs.
                let Ok(authorization) = process.authorize::<CurrentAleo, _>(
                    &private_key,
                    program.id(),
                    function.name(),
                    inputs.into_iter(),
                    rng,
                ) else {
                    return;
                };

                if cfg!(any(feature = "execute", feature = "full")) {
                    // Attempt to execute the process (which will eventually fail due to lack of key synthesis).
                    let _response_and_trace = process.execute::<CurrentAleo, _>(authorization, rng);
                }
            }
        }
    });
}
