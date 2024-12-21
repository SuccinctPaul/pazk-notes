use ff::PrimeField;
use plonk::circuit::gate::{CopyTag, Gate};
use plonk::circuit::{Circuit, CircuitConfig};
use plonk::constraint_system::ConstraintSystem;

// Prove {x1,x2,...,xn}(n>2) are fib array.
// Need n-1 rows.
struct FibCircuit<F: PrimeField> {
    fib_array: Vec<u64>,
    row_num: u64,
}

impl<F: PrimeField> Circuit<F> for FibCircuit<F> {
    fn configure(k: usize) -> CircuitConfig<F> {
        let n = 1 << k - 2;

        // column:  | a  | b  | c  |
        // witness  | x1 | x2 | x3 |
        // witness  | x2 | x3 | x4 |
        // witness  | x3 | x5 | x5 |
        // witness  | .. | .. | .. |
        // witness  | x_n-2 | x_n-1 | xn |

        // So we need n-2 rows Add gate.
        let gates: Vec<Gate<F>> = vec![Gate::Add; n];
        // So we need n-3 rows Add gate.
        let mut copy_constraints = vec![];
        for i in 1..n {
            copy_constraints.push((CopyTag::a(i), CopyTag::b(i - 1)));
            copy_constraints.push((CopyTag::b(i), CopyTag::c(i - 1)));
        }
        CircuitConfig {
            gates,
            copy_constraints,
        }
    }

    fn synthesize(&self, meta: &mut ConstraintSystem<F>) -> plonk::circuit::witness::Assigments<F> {
        todo!()
    }
}

fn main() {
    // gen param

    // gen circuit

    // prove

    // verify
}
