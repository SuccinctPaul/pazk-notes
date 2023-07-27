/// For matrix A and B, C = A · B.
///
/// How can one verify that two matrices were multiplied correctly.
/// First,choose a random `r∈Fp`,and let x=(1,r,r2,...,rn−1).
/// Then compute `y=Cx` and `z=A·Bx`,outputting YES if y = z and NO otherwise.
mod matrix;
mod prover;
mod utils;
mod verifier;

use crate::matrix::Matrix;
use crate::prover::Prover;
use crate::utils::gen_x;
use rand_core::OsRng;

#[test]
fn completeness() {
    let n: usize = std::env::var("n")
        .unwrap_or_else(|_| "4".to_string())
        .parse()
        .expect("Cannot parse DEGREE env var as u32");

    // prover
    let alice = Prover::random(n);
    // C = A · B
    let c = alice.matrix_multiplication();

    let x = gen_x(OsRng, n);
    // z=A·Bx
    let z = alice.hash(&x);

    // verify
    // y=Cx
    let y = c.matrix_mul_vec(&x);

    assert_eq!(z, y);
}
