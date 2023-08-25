pub mod prover;
pub mod verifier;

use self::prover::Prover;
use self::verifier::Verifier;
use crate::merkle_tree::proof::MerkleProof;
use crate::poly::*;
use bls12_381::Scalar;
use ff::Field;
use rand_core::OsRng;
use std::env::consts::OS;
use std::iter::Scan;

#[derive(Default)]
pub struct LDTProof {
    pub commits: Vec<MerkleProof>,    // commit of fi
    pub evals: Vec<(Scalar, Scalar)>, // The open values on challenge z for fi: (f0(z), f0(−z)), f1(z^2), f1(−z^2)
    pub last_const: (Scalar, Scalar), // (p_L, p_R)
}

// Both P and V have oracle access to function f.
// V wants to test if f is polynomial with deg(f) ≤ d.
pub struct LDT {
    prover: Prover,
    verifier: Verifier,
}

impl LDT {
    pub fn new(degree: usize) -> Self {
        let poly = random_poly(degree);
        let z = Scalar::random(OsRng);
        let challenge = poly.coeffs().get(0).unwrap();
        let prover = Prover::init(poly, z, challenge.clone());

        let verifier = Verifier::init(degree, z, challenge.clone());

        Self { prover, verifier }
    }

    pub fn run_protocol(&self) {
        let proofs = self.prover.prove();

        self.verifier.verify(proofs);
    }
}

#[cfg(test)]
mod test {
    use crate::ldt::LDT;

    #[test]
    fn test() {
        let ldt = LDT::new(3);
        ldt.run_protocol();
    }
}
