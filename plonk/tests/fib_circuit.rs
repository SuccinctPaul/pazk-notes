use ff::PrimeField;
use plonk::circuit::{Circuit, CircuitConfig};
use plonk::constraint_system::ConstraintSystem;

struct FibCircuitCofig {}
struct FibCircuit<F: PrimeField> {}

impl<F: PrimeField> Circuit<F> for FibCircuit<F> {
    fn configure(meta: &mut ConstraintSystem<F>) -> CircuitConfig<F> {
        // define the fib circuit
        todo!()
    }

    fn synthesize(&self, meta: &mut ConstraintSystem<F>) -> plonk::circuit::witness::Assigments<F> {
        todo!()
    }
}
