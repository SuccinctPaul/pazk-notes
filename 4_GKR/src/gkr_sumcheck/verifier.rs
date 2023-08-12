use crate::poly::Polynomial;
use bls12_381::Scalar;
use ff::{Field, PrimeField};
use rand_core::{OsRng, RngCore};

pub struct Verifier {
    proof: Scalar, // C1 in sumcheck, means mi in GKR.
    v: usize,
    cached_g_j: Vec<Polynomial>,
    challenges: Vec<usize>, // challenges: r1, r2, ..., rv. (In implement, r1 is a random usize, which is easy to construct a Field)
}

impl Verifier {
    pub fn new(v: usize, proof: Scalar) -> Self {
        Self {
            proof,
            v,
            cached_g_j: vec![],
            challenges: vec![],
        }
    }

    pub fn challenges(&self) -> Vec<usize> {
        self.challenges.clone()
    }

    // generate r1, r2, ..., rv
    fn gen_challenge() -> usize {
        let k = OsRng.next_u32() % 1000;
        k as usize
    }
    fn prepare_next_round(&mut self, g_i: Polynomial) {
        assert_eq!(self.challenges.len(), self.cached_g_j.len());

        self.cached_g_j.push(g_i);
        self.challenges.push(Verifier::gen_challenge());
    }

    // Check: C1 = g_1(0) + g_1(1)
    pub fn round_1(&mut self, g1: Polynomial) {
        let actual = g1.evaluate(Scalar::one()) + g1.evaluate(Scalar::zero());

        assert_eq!(actual, self.proof, "No-equal in round_1");

        // prepare for next round
        self.prepare_next_round(g1);
    }

    // 1 < j < v, total v-2 rounds
    // check: g_j-1(r_j-1) = g_j(0) + g_j(1)
    pub fn recursive_round_j(&mut self, round_num: usize, g_j: Polynomial) {
        // check
        self.check_round_j(round_num, &g_j);
        // prepare for next round
        self.prepare_next_round(g_j);
    }

    // Check: gv−1 (rv−1 ) = gv (0) + gv (1).
    pub fn round_v(&mut self, g_v: Polynomial) {
        self.check_round_j(self.v, &g_v);

        // prepare r_v and store g_v.
        self.prepare_next_round(g_v);
    }

    // check: g_j-1(r_j-1) = g_j(0) + g_j(1),  1<j<=v
    fn check_round_j(&mut self, j: usize, g_j: &Polynomial) {
        assert_eq!(
            j - 1,
            self.cached_g_j.len(),
            "length of cached_g_j != (j-1)"
        );
        assert_eq!(
            j - 1,
            self.challenges.len(),
            "length of challenges != (j-1)"
        );

        let r = self.challenges.last().unwrap().clone() as u128;
        let actual = g_j.evaluate(Scalar::zero()) + g_j.evaluate(Scalar::one());
        let target = self
            .cached_g_j
            .last()
            .unwrap()
            .evaluate(Scalar::from_u128(r));

        assert_eq!(actual, target, "Not-equal in round_{}", j);
    }

    // todo:  V checks d􏰌irectly that md = W􏰌d (rd ) using Lemma 3.8.
    //     // 2. gv (rv ) = g(r1 , . . . , rv )
    //     pub fn check(&self, target: Scalar) {
    //         assert_eq!(
    //             self.v,
    //             self.cached_g_j.len(),
    //             "length of cached_g_j != (j-1)"
    //         );
    //         assert_eq!(
    //             self.v,
    //             self.challenges.len(),
    //             "length of challenges != (j-1)"
    //         );
    //         let r_v = self.challenges.last().unwrap().clone() as u128;
    //         let g_v = self.cached_g_j.last().unwrap().clone();
    //         let actual = g_v.evaluate(Scalar::from_u128(r_v));
    //
    //         assert_eq!(actual, target, "Verifier rejected the proof");
    //         println!("Verifier accepted the proof");
    //     }
}
