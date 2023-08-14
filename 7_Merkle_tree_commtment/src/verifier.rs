use crate::merkle_tree::hasher::calculate_hash;
use crate::merkle_tree::proof::Proof;
use crate::prover::Prover;

#[derive(Default)]
pub struct Verifier {
    challenge: char,
}

impl Verifier {
    // random char
    pub fn gen_challenge(&self) -> char {
        // self.challenge =
        todo!()
    }

    pub fn verify(&self, proof: &Proof) {
        let target = proof.root;

        let leaf_hash = calculate_hash(&self.challenge);
        let actual = proof.children.iter().fold(leaf_hash, |acc, eval| {
            let sum = acc + eval;
            calculate_hash(&sum)
        });
        assert!(target, actual, "Verifier: verify failed!")
    }
}
