use crate::merkle_tree::hasher::calculate_hash;
use std::hash::Hash;

/// Node of a Binary Tree.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TreeNode {
    Leaf {
        hash: u64,   // Hash of the node
        value: char, // Value of the leaf node
    },
    Node {
        hash: u64,            // Hash of the node
        left: Box<TreeNode>,  // Left child of the node
        right: Box<TreeNode>, // Right chiild of the node
    },
}

impl TreeNode {
    /// Create a new Node
    pub fn new(hash: u64, value: char) -> Self {
        Self::Leaf { hash, value }
    }

    // Create a new leaf
    pub fn new_leaf(value: char) -> TreeNode {
        let hash = calculate_hash(&value);
        Self::new(hash, value)
    }

    // Returns a hash from the Node.
    pub fn get_hash(&self) -> u64 {
        match self {
            &Self::Leaf { hash, .. } => hash,
            &Self::Node { hash, .. } => hash,
        }
    }
}
