use crate::poly::multivar_poly::MPolynomial;
use crate::poly::univar_poly::Polynomial;
use crate::sumcheck::prover::Prover;
use crate::sumcheck::verifier::Verifier;
use bls12_381::Scalar;
use std::env::var;
use std::iter::Sum;

pub mod prover;
pub mod verifier;

pub struct SumCheck {
    v: usize,
    prover: Prover,
    verifier: Verifier,
}

impl SumCheck {
    pub fn new(g: MPolynomial) -> Self {
        let var_num = g.var_num;

        let prover = Prover::new(g);
        let proof = prover.proof();
        let verifier = Verifier::new(var_num, proof);

        Self {
            v: var_num,
            prover,
            verifier,
        }
    }

    pub fn run_protocol(&mut self) {
        // round 1
        let g1 = self.prover.round_1();
        self.verifier.round_1(g1);

        // round 2 - (v-1)
        for j in 2..self.v {
            let challenges = self.verifier.challenges();
            let g_j = self.prover.recursive_round_j(&challenges);
            self.verifier.recursive_round_j(j, g_j);
            drop(challenges);
        }

        // round v
        let challenges = self.verifier.challenges();
        let g_v = self.prover.round_v(&challenges);
        self.verifier.round_v(g_v);
        // drop(challenges);

        // finally check
        let challenges = self.verifier.challenges();
        let target = self.prover.evaluate(&challenges);
        self.verifier.check(target);
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
