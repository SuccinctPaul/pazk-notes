mod prover;
mod verifier;

use crate::poly::Polynomial;
use crate::sumcheck::prover::Prover;
use crate::sumcheck::verifier::Verifier;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::{poly_to_bytes, Transcript};
use bls12_381::Scalar;
use ff::Field;
use std::env::var;
use std::iter::Sum;

#[derive(Default)]
pub struct Proofs<F: Field> {
    poly_f: Polynomial<F>, // degree = D-n
    poly_h: Polynomial<F>, // degree = n-1
}

pub struct SumCheck<F: Field> {
    prover: Prover<F>,
    verifier: Verifier,
}

impl<F: Field> SumCheck<F> {
    pub fn new(g: Polynomial<F>) -> Self {
        let var_num = g.var_num;

        let prover = Prover::new(var_num, g);
        let statement = prover.statement();

        let verifier = Verifier::new(var_num, statement);

        Self { prover, verifier }
    }

    pub fn run_protocol(&mut self) {
        let proofs = self.prover.prove();

        self.verifier.verify(proofs);
    }
}

#[cfg(test)]
mod test {
    use crate::poly::multivar_poly::MPolynomial;
    use crate::sumcheck::SumCheck;
    use bls12_381::Scalar;
    use ff::PrimeField;

    fn gen_mpoly() -> MPolynomial {
        // let g(x1, x2, x3) = 9 + 2*x3 + 3*x2 + 2 * x1 * x2 + 4* x1 * x2 * x3
        // term0: exp: (0,0,0) = 9
        // term1: exp: (0,0,1) = 2*x3
        // term2: exp: (0,1,0) = 3*x2
        // term3-6: exp: (0,1,0) = 0.
        // term6: exp: (1,1,0) = 2 * x1 * x2
        // term7: exp: (1,1,1) = 4 * x1 * x2 * x3

        let var_num = 3;

        MPolynomial {
            var_num,
            coeffs: vec![
                Scalar::from_u128(9),
                Scalar::from_u128(2),
                Scalar::from_u128(3),
                Scalar::zero(),
                Scalar::zero(),
                Scalar::zero(),
                Scalar::from_u128(2),
                Scalar::from_u128(4),
            ],
        }
    }

    #[test]
    fn test_sumcheck() {
        let mpoly = gen_mpoly();

        let mut sumcheck = SumCheck::new(mpoly);

        sumcheck.run_protocol();
    }
}
