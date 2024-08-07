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

use snarkvm::prelude::{MainnetV0 as CurrentNetwork, PrivateKey, Process, Program, TestRng, Value};

use afl;

type CurrentAleo = snarkvm::circuit::network::AleoV0;

fn main() {
    let rng = &mut TestRng::fixed(7777777);
    let private_key = PrivateKey::<CurrentNetwork>::new(rng).unwrap();

    afl::fuzz!(|program_inputs: (Program<CurrentNetwork>, Vec<Value<CurrentNetwork>>)| {
        let (program, inputs) = program_inputs;

        let Some(function_name) = program.functions().values().next().map(|foo| foo.name()) else {
            return;
        };

        let mut process = Process::load().unwrap();
        process.add_program(&program).unwrap();

        let authorization = process
            .authorize::<CurrentAleo, _>(&private_key, program.id(), function_name, inputs.into_iter(), rng)
            .unwrap();

        let (_, mut trace) = process.execute::<CurrentAleo, _>(authorization, rng).unwrap();
    });
}
