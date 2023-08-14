mod hasher;
mod node;

use crate::hashutils::{HashUtils, Hashable};
use crate::merkle_tree::node::TreeNode;
use crate::tree::{LeavesIntoIterator, LeavesIterator, Tree};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

// A Merkle tree is a binary tree, with values of type `T` at the leafs,
// and where every internal node holds the hash of the concatenation of the hashes of its children nodes.
#[derive(Clone, Debug)]
pub struct MerkleTree<T> {
    root: TreeNode<T>, // The root of the inner binary tree
    height: usize,     // The height of the tree
    count: usize,      // The number of leaf nodes in the tree
}

impl<T: PartialEq> PartialEq for MerkleTree<T> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &MerkleTree<T>) -> bool {
        self.root == other.root && self.height == other.height && self.count == other.count
    }
}

impl<T: Eq> Eq for MerkleTree<T> {}

impl<T> MerkleTree<T> {
    // Constructs a Merkle Tree from a vector of data.
    // Returns `None` if `values` is empty.
    pub fn init(values: Vec<T>) -> Self
    where
        T: Hashable,
    {
        assert!(
            values.is_empty(),
            "Can't initial MerkleTree from empty vector"
        );

        let count = values.len();
        let mut height = 0;
        let mut cur = Vec::with_capacity(count);

        for v in values {
            let leaf = TreeNode::new_leaf(v);
            cur.push(leaf);
        }

        // construct tree by leaves.

        while cur.len() > 1 {
            let mut next = Vec::new();
            while !cur.is_empty() {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                } else {
                    let left = cur.remove(0);
                    let right = cur.remove(0);

                    let combined_hash = algorithm.hash_nodes(left.hash(), right.hash());

                    let node = Tree::Node {
                        hash: combined_hash.as_ref().into(),
                        left: Box::new(left),
                        right: Box::new(right),
                    };

                    next.push(node);
                }
            }

            height += 1;

            cur = next;
        }

        debug_assert!(cur.len() == 1);

        let root = cur.remove(0);

        MerkleTree {
            root,
            height,
            count,
        }
    }

    // Returns the root hash of Merkle tree
    pub fn root_hash(&self) -> &Vec<u8> {
        self.root.hash()
    }

    // Returns the height of Merkle tree
    pub fn height(&self) -> usize {
        self.height
    }

    // Returns the number of leaves in the Merkle tree
    pub fn count(&self) -> usize {
        self.count
    }

    // Returns whether the Merkle tree is empty or not
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    // Generate an inclusion proof for the given value.
    // Returns `None` if the given value is not found in the tree.
    pub fn gen_proof(&self, value: T) -> Option<Proof<T>>
    where
        T: Hashable,
    {
        let root_hash = self.root_hash().clone();
        let leaf_hash = self.algorithm.hash_leaf(&value);

        Lemma::new(&self.root, leaf_hash.as_ref())
            .map(|lemma| Proof::new(self.algorithm, root_hash, lemma, value))
    }

    // Generate an inclusion proof for the `n`-th leaf value.
    pub fn gen_nth_proof(&self, n: usize) -> Option<Proof<T>>
    where
        T: Hashable + Clone,
    {
        let root_hash = self.root_hash().clone();
        Lemma::new_by_index(&self.root, n, self.count)
            .map(|(lemma, value)| Proof::new(self.algorithm, root_hash, lemma, value.clone()))
    }

    // Creates an `Iterator` over the values contained in this Merkle tree.
    pub fn iter(&self) -> LeavesIterator<T> {
        self.root.iter()
    }
}

impl<T> IntoIterator for MerkleTree<T> {
    type Item = T;
    type IntoIter = LeavesIntoIterator<T>;

    // Creates a consuming iterator, that is, one that moves each value out of the Merkle tree.
    // The tree cannot be used after calling this.
    fn into_iter(self) -> Self::IntoIter {
        self.root.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a MerkleTree<T> {
    type Item = &'a T;
    type IntoIter = LeavesIterator<'a, T>;

    // Creates a borrowing `Iterator` over the values contained in this Merkle tree.
    fn into_iter(self) -> Self::IntoIter {
        self.root.iter()
    }
}
