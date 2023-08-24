use crate::merkle_tree::hasher::{Keccak256Hash, ScalarHash};
use crate::merkle_tree::MerkleTree;
use bls12_381::Scalar;
use std::hash::Hash;

/// Node of a Binary Tree.
#[derive(Clone, Debug, Eq)]
pub enum TreeNode {
    Leaf {
        hash: Scalar,  // Hash of the node
        value: Scalar, // Value of the leaf node
    },
    Node {
        hash: Scalar,         // Hash of the node
        left: Box<TreeNode>,  // Left child of the node
        right: Box<TreeNode>, // Right chiild of the node
    },
}

impl TreeNode {
    /// Create a new Node
    pub fn new(hash: Scalar, value: Scalar) -> Self {
        Self::Leaf { hash, value }
    }

    // Create a new leaf
    pub fn new_leaf(value: Scalar) -> TreeNode {
        let hash = Keccak256Hash::hash(&value);
        Self::new(hash, value)
    }

    // Returns a hash from the Node.
    pub fn get_hash(&self) -> Scalar {
        match self {
            &Self::Leaf { hash, .. } => hash,
            &Self::Node { hash, .. } => hash,
        }
    }
}

impl PartialEq<Self> for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TreeNode::Node { hash, left, right } => {
                let (hash1, left1, right1) = (hash, left, right);
                match other {
                    TreeNode::Node { hash, left, right } => {
                        if hash1 != hash || left1 != left || right1 != right || right1 != right {
                            false
                        } else {
                            true
                        }
                    }
                    _ => false,
                }
            }
            TreeNode::Leaf { hash, value } => {
                let (hash1, value1) = (hash, value);
                match other {
                    TreeNode::Leaf { hash, value } => {
                        if hash1 != hash || value1 != value {
                            false
                        } else {
                            true
                        }
                    }
                    _ => false,
                }
            }
        }
    }
}
