use crate::merkle_tree::proof::Proof;
use crate::prover::Prover;
use crate::verifier::Verifier;

pub mod merkle_tree;
pub mod prover;
pub mod verifier;

#[test]
fn test_merkle_tree_commit() {
    let prover = Prover::random_values();
    let verifier = Verifier::default();

    let challenge = verifier.gen_challenge();
    let (has_challenge, proof) = prover.has_x(&challenge);

    if has_challenge {
        verifier.verify(&proof.unwrap());
    } else {
        println!("Don't has the challenge");
    }
}
