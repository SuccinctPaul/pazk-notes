/// For matrix A and B, C = A · B.
///
/// How can one verify that two matrices were multiplied correctly.
/// First,choose a random `r∈Fp`,and let x=(1,r,r2,...,rn−1).
/// Then compute `y=Cx` and `z=A·Bx`,outputting YES if y = z and NO otherwise.

mod prover;
mod verifier;
mod setup;
mod matrix;




fn prepare(){
    let n: u64 = std::env::var("n")
        .unwrap_or_else(|_| "4".to_string())
        .parse()
        .expect("Cannot parse DEGREE env var as u32");

}