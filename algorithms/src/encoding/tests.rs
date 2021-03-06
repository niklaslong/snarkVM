// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::encoding::Elligator2;
use snarkvm_curves::edwards_bls12::*;
use snarkvm_models::curves::Zero;
use snarkvm_utilities::rand::UniformRand;

use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

pub(crate) const ITERATIONS: usize = 10000;

#[test]
fn test_elligator2_encode_decode() {
    let rng = &mut XorShiftRng::seed_from_u64(1231275789u64);

    for _ in 0..ITERATIONS {
        let original: Fq = Fq::rand(rng);

        let (encoded, fq_high) = Elligator2::<EdwardsParameters, EdwardsProjective>::encode(&original).unwrap();
        let decoded = Elligator2::<EdwardsParameters, EdwardsProjective>::decode(&encoded, fq_high).unwrap();

        assert_eq!(original, decoded)
    }
}

#[test]
fn test_elligator2_zero() {
    let encode = Elligator2::<EdwardsParameters, EdwardsProjective>::encode(&Fq::zero());
    assert!(encode.is_err());

    let decode = Elligator2::<EdwardsParameters, EdwardsProjective>::decode(&EdwardsAffine::zero(), false);
    assert!(decode.is_err());
}
