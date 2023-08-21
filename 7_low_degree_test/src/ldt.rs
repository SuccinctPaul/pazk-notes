use sumcheck::poly::univar_poly::Polynomial;
use sumcheck::sumcheck::prover::Prover;
use sumcheck::sumcheck::verifier::Verifier;

pub mod prover;
pub mod verifier;

// Both P and V have oracle access to function f.
// V wants to test if f is polynomial with deg(f) ≤ d.
pub struct LDT {
    prover: Prover,
    verifier: Verifier,
}

impl LDT {
    // pub fn new(g: Polynomial) -> Self {
    //
    //     let prover = Prover::new(var_num, g);
    //     let statement = prover.statement();
    //
    //     let verifier = Verifier::new(var_num, statement);
    //
    //     Self { prover, verifier }
    // }

    pub fn run_protocol(&mut self) {
        // let proofs = self.prover.prove();
        //
        // self.verifier.verify(proofs);
    }
}
