use crate::poly::multivar_poly::MPolynomial;
use crate::poly::univar_poly::Polynomial;
use crate::sumcheck::Proofs;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::{poly_to_bytes, Transcript};
use crate::utils::convert_to_binary;
use bls12_381::Scalar;

pub struct Prover {
    g: MPolynomial,
    v: usize,
    challenges: Vec<usize>, // challenges: r1, r2, ..., rv. (In implement, r1 is a random usize, which is easy to construct a Field)
}

impl Prover {
    pub fn new(v: usize, mpoly: MPolynomial) -> Self {
        Self {
            g: mpoly,
            v,
            challenges: vec![],
        }
    }

    // sum all the evaluations on hypercube of a mpoly
    // obtain C1, which claimed equal H.
    pub fn statement(&self) -> Scalar {
        let n = 1 << self.g.var_num;
        (0..n)
            .map(|i| {
                let domain = convert_to_binary(&self.g.var_num, i);
                self.g.evaluate(&domain)
            })
            .sum()
    }

    fn prepare_for_next_round(
        &mut self,
        g_i: &Polynomial,
        proofs: &mut Proofs,
        transcript: &mut Keccak256Transcript,
    ) {
        assert_eq!(self.challenges.len(), proofs.g_i_vec.len());

        // generate r1, r2, ..., rv
        transcript.append(&poly_to_bytes(g_i));
        self.challenges.push(transcript.challenge());

        // cache g_i
        proofs.g_i_vec.push(g_i.clone());
    }

    pub fn prove(&mut self) -> Proofs {
        let mut proofs = Proofs::default();
        let mut transcript = Keccak256Transcript::default();

        // round 1
        let g1 = self.round_1();
        // self.challenges.push(prepare_next_round(&g1));
        self.prepare_for_next_round(&g1, &mut proofs, &mut transcript);

        // round 2 - (v-1)
        for _ in 2..self.v {
            let g_j = self.recursive_round_j();
            self.prepare_for_next_round(&g_j, &mut proofs, &mut transcript);
        }

        // round v
        let g_v = self.round_v();
        self.prepare_for_next_round(&g_v, &mut proofs, &mut transcript);

        // finally check
        let target = self.evaluate();
        proofs.target = target;

        proofs
    }

    // Return g1(X) = sum g(X, x_2, ..., x_v)
    fn round_1(&self) -> Polynomial {
        self.g.partial_evaluate(&vec![])
    }

    // 1 < j < v, total v-2 rounds
    // Return g_j = (r1, ..., r_j-1, X, x_j+1, ..., x_v)
    fn recursive_round_j(&self) -> Polynomial {
        self.g.partial_evaluate(&self.challenges)
    }

    // Return g_v = (r1, r2, ..., r_v-1, X_v)
    fn round_v(&self) -> Polynomial {
        self.g.partial_evaluate(&self.challenges)
    }

    fn evaluate(&self) -> Scalar {
        self.g.evaluate(&self.challenges)
    }
}
