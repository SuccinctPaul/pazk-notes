use crate::poly::multivar_poly::MPolynomial;
use bls12_381::Scalar;

mod prover;
mod verifier;

struct Sumcheck {
    g: MPolynomial,
    // challengers: r
    challengers: Vec<Scalar>,
}
