use crate::circuit::copy_constraint::{CopyConstraint, CopyTag};
use crate::circuit::gate::{CopyTag, Gate};
use crate::circuit::witness::Assigments;
use crate::constraint_system::ConstraintSystem;
use ff::PrimeField;

pub mod gate;
pub mod witness;

// The circuit trait.
pub trait Circuit<F: PrimeField> {
    // config the circuit gate.
    // Note: As the q_c is the constant column, so that, the constant(public input) is a part of config.
    fn configure(k: usize) -> CircuitConfig<F>;

    // synthesize the witness(intput and its generated advice) values.
    fn synthesize(&self, meta: &mut ConstraintSystem<F>) -> Assigments<F>;
}

pub struct CircuitConfig<F: PrimeField> {
    pub gates: Vec<Gate<F>>,
    pub copy_constraints: Vec<(CopyTag, CopyTag)>,
}
