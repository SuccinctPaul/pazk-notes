use crate::poly::multivar_poly::MPolynomial;
use crate::poly::univar_poly::Polynomial;
use crate::sumcheck::prover::Prover;
use crate::sumcheck::verifier::Verifier;
use bls12_381::Scalar;
use std::env::var;
use std::iter::Sum;

mod prover;
mod verifier;

struct SumCheck {
    prover: Prover,
    verifier: Verifier,
}

impl SumCheck {
    fn new(g: MPolynomial) -> Self {
        let var_num = g.var_num;

        let prover = Prover::new(g);
        let proof = prover.proof();
        let verifier = Verifier::new(var_num, proof);

        Self { prover, verifier }
    }

    fn run_protocol() {}
}
