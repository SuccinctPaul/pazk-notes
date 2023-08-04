use crate::poly::univar_poly::Polynomial;
use crate::sumcheck::Proofs;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::{poly_to_bytes, Transcript};
use bls12_381::Scalar;
use ff::{Field, PrimeField};

pub struct Verifier {
    // The C1
    statement: Scalar,
    v: usize,
    challenges: Vec<usize>, // challenges: r1, r2, ..., rv. (In implement, r1 is a random usize, which is easy to construct a Field)
}

impl Verifier {
    pub fn new(v: usize, statement: Scalar) -> Self {
        Self {
            statement,
            v,
            challenges: vec![],
        }
    }
    fn prepare_for_next_round(
        &mut self,
        j: usize,
        g_j: &Polynomial,
        transcript: &mut Keccak256Transcript,
    ) {
        assert!(j >= 1);
        assert_eq!(self.challenges.len(), j - 1);

        // generate r1, r2, ..., rv
        transcript.append(&poly_to_bytes(g_j));
        self.challenges.push(transcript.challenge());
    }

    pub fn verify(&mut self, proofs: Proofs) {
        assert_eq!(proofs.g_i_vec.len(), self.v);

        let mut transcript = Keccak256Transcript::default();

        let g_i_vec = proofs.g_i_vec;

        // round 1
        let g1 = &g_i_vec[0];
        self.round_1(&g1);
        self.prepare_for_next_round(1, &g1, &mut transcript);

        // round 2 - (v-1)
        for j in 2..self.v {
            let g_j = &g_i_vec[j - 1];
            let g_j_minus_1 = &g_i_vec[j - 2];
            self.check_round_j(j, g_j_minus_1, g_j);

            self.prepare_for_next_round(j, &g_j, &mut transcript);
        }

        // round v
        let g_v = &g_i_vec[self.v - 1];
        let g_v_minus_1 = &g_i_vec[self.v - 2];
        self.round_v(g_v_minus_1, g_v);
        // prepare r_v and store g_v.
        self.prepare_for_next_round(self.v, &g_v, &mut transcript);

        // finally check.
        self.check(proofs.target, &g_v);
    }

    // Check: C1 = g_1(0) + g_1(1)
    fn round_1(&mut self, g1: &Polynomial) {
        let actual = g1.evaluate(Scalar::one()) + g1.evaluate(Scalar::zero());

        assert_eq!(actual, self.statement, "No-equal in round_1");
    }

    // Check: gv−1 (rv−1 ) = gv (0) + gv (1).
    fn round_v(&mut self, g_v_minus_1: &Polynomial, g_v: &Polynomial) {
        self.check_round_j(self.v, g_v_minus_1, g_v);
    }

    // 1 < j < v, total v-2 rounds
    // check: g_j-1(r_j-1) = g_j(0) + g_j(1)
    fn check_round_j(&mut self, j: usize, g_j_minus_1: &Polynomial, g_j: &Polynomial) {
        assert_eq!(
            j - 1,
            self.challenges.len(),
            "length of challenges != (j-1)"
        );

        // r_j-1
        let r_j_minus_1 = self.challenges.last().unwrap().clone() as u128;
        // g_j(0) + g_j(1)
        let actual = g_j.evaluate(Scalar::zero()) + g_j.evaluate(Scalar::one());
        let target = g_j_minus_1.evaluate(Scalar::from_u128(r_j_minus_1));

        assert_eq!(actual, target, "Not-equal in round_{}", j);
    }

    // 3. check: gv(rv) = g(r1 , . . . , rv )
    // target = g(r1 , . . . , rv )
    fn check(&self, target: Scalar, g_v: &Polynomial) {
        assert_eq!(
            self.v,
            self.challenges.len(),
            "length of challenges != (j-1)"
        );
        let r_v = self.challenges.last().unwrap().clone() as u128;
        let actual = g_v.evaluate(Scalar::from_u128(r_v));

        assert_eq!(actual, target, "Verifier rejected the proof");
        println!("Verifier accepted the proof");
    }
}
