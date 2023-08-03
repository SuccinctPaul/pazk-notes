use crate::poly::multivar_poly::MPolynomial;
use crate::poly::univar_poly::Polynomial;
use crate::utils::convert_to_binary;
use bls12_381::Scalar;
use crate::sumcheck::Proofs;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::{poly_to_bytes, Transcript};

pub struct Prover {
    g: MPolynomial,
}

impl Prover {
    pub fn new(mpoly: MPolynomial) -> Self {
        Self { g: mpoly }
    }


    // sum all the evaluations on hypercube of a mpoly
    // obtain C1, which claimed equal H.
    fn proof(&self) -> Scalar {
        let n = 1 << self.g.var_num;
        (0..n)
            .map(|i| {
                let domain = convert_to_binary(&self.g.var_num, i);
                self.g.evaluate(&domain)
            })
            .sum()
    }


    pub fn prove(&mut self) -> Proofs {
        let mut proofs = Proofs::default();
        let mut transcript = Keccak256Transcript::default();

        // round 1
        let g1 = self.prover.round_1();
        transcript.append(&poly_to_bytes(&g1));
        proofs.g_i_vec.push(g1);

        // round 2 - (v-1)
        for j in 2..self.v {
            let challenges =transcript.challenge();
            let g_j = self.prover.recursive_round_j(&challenges);

            transcript.append(&poly_to_bytes(&g_j));
            proofs.g_i_vec.push(g_j);
            drop(challenges);
        }

        // round v
        let challenges =transcript.challenge();
        let g_v = self.prover.round_v(&challenges);
        transcript.append(&poly_to_bytes(&g_v));
        proofs.g_i_vec.push(g_v);

        // finally check
        let challenges =transcript.challenge();
        let target = self.prover.evaluate(&challenges);
        proofs.target = target;

        proofs
    }


    // Return g1(X) = sum g(X, x_2, ..., x_v)
    fn round_1(&self) -> Polynomial {
        self.g.partial_evaluate(&vec![])
    }

    // 1 < j < v, total v-2 rounds
    // Return g_j = (r1, ..., r_j-1, X, x_j+1, ..., x_v)
    fn recursive_round_j(&self, challenges: &Vec<usize>) -> Polynomial {
        self.g.partial_evaluate(challenges)
    }

    // Return g_v = (r1, r2, ..., r_v-1, X_v)
    fn round_v(&self, challenges: &Vec<usize>) -> Polynomial {
        self.g.partial_evaluate(challenges)
    }

    fn evaluate(&self, challenges: &Vec<usize>) -> Scalar {
        self.g.evaluate(challenges)
    }
}
