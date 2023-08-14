use ring::digest::{Algorithm, Digest};

use crate::hashutils::{HashUtils, Hashable};
use crate::merkle_tree::hasher::calculate_hash;

pub use crate::proof::{Lemma, Positioned, Proof};

/// Node of a Binary Tree.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TreeNode<T> {
    Leaf {
        hash: u64, // Hash of the node
        value: T,  // Value of the leaf node
    },
    Node {
        hash: u64,               // Hash of the node
        left: Box<TreeNode<T>>,  // Left child of the node
        right: Box<TreeNode<T>>, // Right chiild of the node
    },
}

impl<T> TreeNode<T> {
    /// Create a new Node
    pub fn new(hash: u64, value: T) -> Self {
        Self::Leaf { hash, value }
    }

    /// Create a new leaf
    pub fn new_leaf(value: T) -> Node<T>
    where
        T: Hashable,
    {
        let hash = calculate_hash(&value);
        Self::new(hash, value)
    }

    /// Returns a hash from the Node.
    pub fn hash(&self) -> u64 {
        match self {
            &Self::Leaf { hash, .. } => hash,
            &Self::Node { hash, .. } => hash,
        }
    }

    /// Returns a borrowing iterator over the leaves of the Node.
    pub fn iter(&self) -> LeavesIterator<T> {
        LeavesIterator::new(self)
    }
}

/// An borrowing iterator over the leaves of a `Node`.
/// Adapted from http://codereview.stackexchange.com/q/110283.
#[allow(missing_debug_implementations)]
pub struct LeavesIterator<'a, T>
where
    T: 'a,
{
    current_value: Option<&'a T>,
    right_nodes: Vec<&'a Node<T>>,
}

impl<'a, T> LeavesIterator<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
        let mut iter = LeavesIterator {
            current_value: None,
            right_nodes: Vec::new(),
        };

        iter.add_left(root);

        iter
    }

    fn add_left(&mut self, mut Node: &'a Node<T>) {
        loop {
            match *Node {
                TreeNode::Node {
                    ref left,
                    ref right,
                    ..
                } => {
                    self.right_nodes.push(right);
                    Node = left;
                }

                TreeNode::Leaf { ref value, .. } => {
                    self.current_value = Some(value);
                    break;
                }
            }
        }
    }
}

impl<'a, T> Iterator for LeavesIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let result = self.current_value.take();

        if let Some(rest) = self.right_nodes.pop() {
            self.add_left(rest);
        }

        result
    }
}

/// An iterator over the leaves of a `Node`.
#[allow(missing_debug_implementations)]
pub struct LeavesIntoIterator<T> {
    current_value: Option<T>,
    right_nodes: Vec<Node<T>>,
}

impl<T> LeavesIntoIterator<T> {
    fn new(root: Node<T>) -> Self {
        let mut iter = LeavesIntoIterator {
            current_value: None,
            right_nodes: Vec::new(),
        };

        iter.add_left(root);

        iter
    }

    fn add_left(&mut self, mut Node: Node<T>) {
        loop {
            match Node {
                TreeNode::Node { left, right, .. } => {
                    self.right_nodes.push(*right);
                    Node = *left;
                }

                TreeNode::Leaf { value, .. } => {
                    self.current_value = Some(value);
                    break;
                }
            }
        }
    }
}

impl<T> Iterator for LeavesIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let result = self.current_value.take();

        if let Some(rest) = self.right_nodes.pop() {
            self.add_left(rest);
        }

        result
    }
}

impl<T> IntoIterator for Node<T> {
    type Item = T;
    type IntoIter = LeavesIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LeavesIntoIterator::new(self)
    }
}
