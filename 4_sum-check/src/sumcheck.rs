use crate::poly::multivar_poly::MPolynomial;
use crate::sumcheck::prover::Prover;
use crate::sumcheck::verifier::Verifier;
use bls12_381::Scalar;
use std::env::var;
use std::iter::Sum;

mod prover;
mod verifier;

struct Sumcheck {
    g: MPolynomial,
    challengers: Vec<Scalar>, // challengers: r
    // prover: Prover,
    verifier: Verifier,
}

impl Sumcheck {
    fn new(g: MPolynomial) -> Self {
        let var_num = g.var_num;
        let challengers = Vec::with_capacity(var_num);

        let proof = Prover::proof(&g);
        let verifier = Verifier { proof };

        Self {
            g,
            challengers,
            verifier,
        }
    }

    fn run_prototal() {}
}
