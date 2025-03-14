// Copyright 2024 Aleo Network Foundation
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

use circuit::{AleoV0, Plaintext};
use console::{
    account::{Address, PrivateKey},
    network::MainnetV0,
    program::{Identifier, Record, Value},
};
use ledger_query::Query;
use ledger_store::{BlockStore, helpers::memory::BlockMemory};
use synthesizer_process::Process;
use synthesizer_program::Program;
use utilities::TestRng;

use criterion::{Criterion, criterion_group, criterion_main};
use std::str::FromStr;

type CurrentAleo = AleoV0;
type CurrentNetwork = MainnetV0;

fn transfer_public_to_private(c: &mut Criterion) {
    let mut rng = TestRng::default();

    // Initialize a new caller account.
    let private_key = PrivateKey::<CurrentNetwork>::new(&mut rng).unwrap();
    let caller = Address::try_from(&private_key).unwrap();

    // Initialize a new program.
    let program = Program::<CurrentNetwork>::credits().unwrap();

    // Declare the inputs.
    let r0 = Value::<CurrentNetwork>::from_str(&format!("{caller}")).unwrap();
    let r1 = Value::<CurrentNetwork>::from_str("1_500_000_000_000_000_u64").unwrap();

    // Construct a new process.
    let process = Process::load().unwrap();

    /* AUTHORIZE */

    // Authorize the function call.
    let authorization = process
        .authorize::<CurrentAleo, _>(
            &private_key,
            program.id(),
            Identifier::from_str("transfer_public_to_private").unwrap(),
            [r0, r1].iter(),
            &mut rng,
        )
        .unwrap();

    // Authorize the fee.
    let execution_id = authorization.to_execution_id().unwrap();
    let fee_authorization =
        process.authorize_fee_public::<CurrentAleo, _>(&private_key, 300000, 10, execution_id, &mut rng).unwrap();

    /* EXECUTE FUNCTION */

    // Construct the query and the locator of the main function.
    let block_store = BlockStore::<CurrentNetwork, BlockMemory<_>>::open(None).unwrap();
    let query = Query::from(block_store);
    let locator = {
        let request = authorization.peek_next().unwrap();
        console::program::Locator::new(*request.program_id(), *request.function_name())
    };

    // Execute and prove.
    let (_response, mut trace) = process.execute::<CurrentAleo, _>(authorization, &mut rng).unwrap();
    trace.prepare(query.clone()).unwrap();

    c.bench_function("prove_execution", |b| {
        b.iter(|| {
            let execution = trace.prove_execution::<CurrentAleo, _>(&locator.to_string(), &mut rng).unwrap();
        })
    });

    // assert!(process.verify_execution(&execution).is_ok());

    /* EXECUTE FEE */

    // let (_, mut trace) = process.execute::<CurrentAleo, _>(fee_authorization, &mut rng).unwrap();
    // trace.prepare(query).unwrap();
    // let fee = trace.prove_fee::<CurrentAleo, _>(&mut rng).unwrap();

    // assert!(process.verify_fee(&fee, execution_id).is_ok());
}

criterion_group! {
    name = transfer_public;
    config = Criterion::default().sample_size(100);
    targets = transfer_public_to_private
}
criterion_main!(transfer_public);
