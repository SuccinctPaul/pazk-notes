pub mod hasher;
pub mod node;
pub mod proof;

use crate::hashutils::{HashUtils, Hashable};
use crate::merkle_tree::hasher::calculate_hash;
use crate::merkle_tree::node::TreeNode;
use crate::merkle_tree::proof::Proof;
use crate::tree::{LeavesIntoIterator, LeavesIterator, Tree};
use numeric::math::log2;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::mpsc::channel;

// A Merkle tree is a binary tree, with values of type `T` at the leafs,
// and where every internal node holds the hash of the concatenation of the hashes of its children nodes.
// Note: For convinence, we suppose Merkle tree is a ![complete binary tree](https://www.geeksforgeeks.org/types-of-binary-tree/?ref=lbp)
//      Degree: 2
//      Leaf nodes: if tree height is h, so the number of leaf nodes will be `2^h`
//      Total nodes: A tree of height h has total nodes = 2^(h+1)–1
//      Height of tree: If tree has N nodes, the hight `h=log(N+1)–1=Θ(ln(n))`. From root to leaf: [1,h].
#[derive(Clone, Debug)]
pub struct MerkleTree<T> {
    root: TreeNode<T>, // The root of the inner binary tree
    height: usize,     // The height of the tree
}

impl<T> MerkleTree<T> {
    // Leaf nodes: if tree height is h, so the number of leaf nodes will be `2^h`
    pub fn leaves_num(&self) -> usize {
        2 ^ self.height
    }

    // Total nodes: A tree of height h has total nodes = 2^(h+1)–1
    pub fn nodes_num(&self) -> usize {
        2 ^ (self.height + 1) - 1
    }

    // Calc Hight by leaf number.
    fn hight(leaves_num: usize) -> usize {
        // leaves_num.ilog2()
        todo!()
    }

    pub fn commit(&self, x: &T) -> Proof {
        let mut values = Vec::with_capacity(self.height - 1);
        let root_hash = self.root.get_hash();
        Self::dfs(&self.root, &x, &mut values);
        Proof {
            root: root_hash,
            children: values,
        }
    }

    fn dfs(root: &TreeNode<T>, target: &T, res: &mut Vec<u64>) -> bool {
        match root {
            TreeNode::Leaf(_, value) => {
                if value == target {
                    true
                } else {
                    false
                }
            }
            TreeNode::Node(_, left, right) => {
                let l = Self::dfs(left, target, res);
                // if left meet target.
                if l {
                    res.push(right.get_hash());
                    return true;
                }

                // if right meet target.
                let r = Self::dfs(right, target, res);
                if r {
                    res.push(left.get_hash());
                }
                r
            }
        }
    }

    // Constructs a Merkle Tree from a vector of data.
    // Root = hash_util(left.hash + right.hash)
    pub fn init(values: Vec<T>) -> Self {
        assert!(
            values.is_empty(),
            "Can't initial MerkleTree from empty vector"
        );
        let count = values.len();
        // assert!(, "len of vector must be 2^h")

        let mut height = 0;

        let leaves_nodes = values
            .iter()
            .map(|v| TreeNode::new_leaf(v))
            .collect::<Vec<TreeNode<T>>>();

        // construct tree by leaves.
        let mut cur = leaves_nodes;
        while cur.len() > 1 {
            let mut parent = Vec::new();
            while !cur.is_empty() {
                assert!(cur.len() == 1, "It's not a perfect tree");
                let left = cur.remove(0);
                let right = cur.remove(0);

                let sum = left.get_hash() + right.get_hash();
                let parent_hash = calculate_hash(&sum);

                let node = TreeNode::Node {
                    hash: parent_hash,
                    left,
                    right,
                };

                next.push(node);
            }

            height += 1;

            cur = next;
        }

        let root = cur.remove(0);

        MerkleTree { root, height }
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

impl<T: PartialEq> PartialEq for MerkleTree<T> {
    #[allow(trivial_casts)]
    fn eq(&self, other: &MerkleTree<T>) -> bool {
        self.root == other.root && self.height == other.height && self.count == other.count
    }
}

impl<T: Eq> Eq for MerkleTree<T> {}

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
