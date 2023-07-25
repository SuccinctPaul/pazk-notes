use crate::poly::univar_poly::Polynomial;
use bls12_381::Scalar;
use ff::Field;
use rand_core::OsRng;

pub struct Verifier {
    // C1
    pub(crate) proof: Scalar,
    // // r1, r2, ..., rv.
    // challengers: Vec<Scalar>,
}

impl Verifier {
    // generate r1, r2, ..., rv
    pub fn generate_challenger() -> Scalar {
        Scalar::random(OsRng)
    }

    pub fn round_1(self, g1: Polynomial) {
        let actual = g1.evaluate(Scalar::one()) + g1.evaluate(Scalar::zero());

        assert_eq!(actual, self.proof, "No-equal in round_1");
    }

    // 1 < j < v, total v-2 rounds
    pub fn recursive_round_j() {
        // g_j-1(r_j-1) = g_j(0) + g_j(1)
    }

    //
    pub fn round_v() -> bool {
        // 1. gv−1 (rv−1 ) = gv (0) + gv (1).

        // 2. gv (rv ) = g(r1 , . . . , rv )

        true
    }
}
