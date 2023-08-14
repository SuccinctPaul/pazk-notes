use crate::merkle_tree::MerkleTree;
use std::intrinsics::fabsf32;

pub struct Prover {
    values: Vec<char>, // statement.
    merkle_tree: MerkleTree<char>,
}

impl Prover {
    pub fn random_values() -> Self {
        let values = vec![seed];
        let merkle_tree = MerkleTree::init(values.clone());
        Self {
            values,
            merkle_tree,
        }
    }

    // V send a challenge to P,
    pub fn has_x(&self, x: &char) -> (bool, Option<Proof>) {
        if self.values.contains(x) {
            let proof = self.merkle_tree.commit();

            (true, Some(proof))
        } else {
            (false, None)
        }
    }
}
