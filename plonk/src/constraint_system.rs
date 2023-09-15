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
    pub c_a: Vec<CopyOf>,
    pub c_b: Vec<CopyOf>,
    pub c_c: Vec<CopyOf>,
}

impl<F: PrimeField> ConstraintSystem<F> {
    pub fn add_constraint(&mut self, a: &F, op: CircuitOps, b: &F, equals_c: &F) {
        // can we split this from there. Which means make config and synthysis as different function.

        // add to constraint_system.
        // self.a.push(a);
        // self.b.push(b);
        // self.c.push(equals_c);

        // add contraints.
        let (q_l, q_r, q_m, q_o, q_c) = match op {
            CircuitOps::Add => (F::ONE, F::ONE, F::ZERO, F::ONE.neg(), F::ZERO),
            CircuitOps::Mul => (F::ZERO, F::ZERO, F::ONE, F::ONE.neg(), F::ZERO),
            CircuitOps::Const => (F::ZERO, F::ONE, F::ZERO, F::ZERO, F::ONE.neg() * equals_c),
            CircuitOps::PublicInput => {
                // self.pub_gate_position.push(self.q_r.len());
                // self.pub_gate_value.push(F::from(b.parse::<i32>().unwrap()));

                (F::ZERO, F::ONE, F::ZERO, F::ZERO, F::ZERO)
            }
            _ => panic!(),
        };
        self.q_l.push(q_l);
        self.q_r.push(q_r);
        self.q_m.push(q_m);
        self.q_o.push(q_o);
        self.q_c.push(q_c);
    }
}
