//! The constrains system is consisted of:
//! 1. arithmetic constraints:
//!     (q_l * a) + (q_r * b) + (q_o * c) + (q_m * a * b) + q_c = 0
//! 2. permutation constraints

mod witness;

use ff::PrimeField;
use pairing::Engine;

// Represents the minimal parameters that determine a `ConstraintSystem`.
pub struct ConstraintSystem<F: PrimeField> {
    // witness columns
    pub a: Vec<F>,
    pub b: Vec<F>,
    pub c: Vec<F>,
    // math constraint columns
    pub q_l: Vec<F>,
    pub q_r: Vec<F>,
    pub q_m: Vec<F>,
    pub q_o: Vec<F>,
    pub q_c: Vec<F>,
    // copy constraint_system columns
    // pub c_a: Vec<CopyOf>,
    // pub c_b: Vec<CopyOf>,
    // pub c_c: Vec<CopyOf>,
}

impl<F: PrimeField> ConstraintSystem<F> {
    fn evaluate(&self) {
        for (((ai, bi), ci), ((((q_l, q_r), q_m), q_o), q_c)) in
            self.a.iter().zip(self.b.iter()).zip(self.c.iter()).zip(
                self.q_l
                    .iter()
                    .zip(self.q_r.iter())
                    .zip(self.q_m.into_iter())
                    .zip(self.q_o.iter())
                    .zip(self.q_c.iter()),
            )
        {
            // arithmetic constraint
            assert_eq!(
                *ai * q_l + *bi * q_r + *ci * q_o + *ai * bi * q_m + q_c,
                F::ZERO,
                "arithmetic constraint meet wrong"
            )
        }
    }
}
