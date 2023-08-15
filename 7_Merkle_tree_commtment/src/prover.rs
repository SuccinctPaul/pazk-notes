use crate::merkle_tree::proof::Proof;
use crate::merkle_tree::MerkleTree;
use crate::utils::random_chars;

pub struct Prover {
    pub(crate) values: Vec<char>, // statement.
    pub(crate) merkle_tree: MerkleTree,
}

impl Prover {
    // k is the depth of tree, 2^k is the random values size.
    pub fn random_values(k: usize) -> Self {
        let values = random_chars(k);
        let merkle_tree = MerkleTree::init(values.clone());
        assert_eq!(merkle_tree.height(), k, "Unexpected Merkle tree height");
        Self {
            values,
            merkle_tree,
        }
    }

    // V send a challenge to P,
    pub fn has_x(&self, x: &char) -> (bool, Option<Proof>) {
        if self.values.contains(x) {
            let proof = self.merkle_tree.commit(x);

            (true, Some(proof))
        } else {
            (false, None)
        }
    }
}
