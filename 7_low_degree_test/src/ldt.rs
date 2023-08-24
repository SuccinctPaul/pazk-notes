pub mod prover;
pub mod verifier;

use self::prover::Prover;
use self::verifier::Verifier;
use crate::poly::*;
use bls12_381::Scalar;
use std::iter::Scan;

// Both P and V have oracle access to function f.
// V wants to test if f is polynomial with deg(f) ≤ d.
pub struct LDT {
    prover: Prover,
    verifier: Verifier,
    z: Scalar, // the random one, with
}

impl LDT {
    pub fn new(degree: usize) -> Self {
        let poly = random_poly(degree);

        let prover = Prover::init(poly);

        let verifier = Verifier { target_deg: degree };

        Self { prover, verifier }
    }

    pub fn run_protocol(&mut self) {
        // let proofs = self.prover.prove();
        //
        // self.verifier.verify(proofs);
    }
}
