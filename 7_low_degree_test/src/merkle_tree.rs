pub mod hasher;
pub mod node;
pub mod proof;

use crate::merkle_tree::hasher::{Keccak256Hash, ScalarHash};
use crate::merkle_tree::node::TreeNode;
use crate::merkle_tree::proof::MerkleProof;
use crate::utils::convert_to_binary;
use ark_std::log2;
use bls12_381::Scalar;
use std::cmp::Ordering;
use std::fs::metadata;

// A Merkle tree is a binary tree, with values of type `T` at the leafs,
// and where every internal node holds the hash of the concatenation of the hashes of its children nodes.
// Note: For convinence, we suppose Merkle tree is a ![complete binary tree](https://www.geeksforgeeks.org/types-of-binary-tree/?ref=lbp)
//      Degree: 2
//      Leaf nodes: if tree height is h, so the number of leaf nodes will be `2^(h-1)`
//      Total nodes: A tree of height h has total nodes = 2^h–1
//      Height of tree: If tree has N nodes, the hight `h=log(N+1)–1=Θ(ln(n))`. From root to leaf: [1,h].
#[derive(Clone, Debug)]
pub struct MerkleTree {
    root: TreeNode, // The root of the inner binary tree
    height: usize,  // The height of the tree
}

impl MerkleTree {
    // init and commit
    // Constructs a Merkle Tree from a vector of data.
    // Root = hash_util(left.hash + right.hash)
    pub fn commit(values: Vec<Scalar>) -> Self {
        assert!(
            !values.is_empty(),
            "Can't initial MerkleTree from empty vector"
        );
        let leaves_num = values.len();
        let height: usize = 1 + log2(leaves_num) as usize;
        assert_eq!(1 << (height - 1), leaves_num, "It's not a perfect tree");

        // lowest level
        let leaves_nodes = values
            .iter()
            .map(|v| TreeNode::new_leaf(*v))
            .collect::<Vec<TreeNode>>();

        // construct tree by leaves.
        let mut cur = leaves_nodes;
        for i in 0..(height - 1) {
            let cur_len = cur.len();
            let parant = (0..(cur_len / 2))
                .map(|j| {
                    let left = cur.get(2 * j).unwrap();
                    let right = cur.get(2 * j + 1).unwrap();
                    let parent_hash = Keccak256Hash::hash(&left.get_hash().add(&right.get_hash()));

                    TreeNode::Node {
                        hash: parent_hash,
                        left: Box::new(left.clone()),
                        right: Box::new(right.clone()),
                    }
                })
                .collect::<Vec<TreeNode>>();
            cur = parant;
        }
        assert_eq!(cur.len(), 1);

        let root = cur.remove(0);

        MerkleTree { root, height }
    }

    // equal the commit, by open it by index of values.
    pub fn open_by_index(&self, index: usize) -> MerkleProof {
        // index belong [0, leaves_num).
        assert!(index >= 0 && index < self.leaves_num(), "Wrong leaf index");

        let path_len = (self.height - 1);
        // get leaf-root path,
        // Suppose the left child is 0, the right child is 1, so the path can be indexed as binary form with (height-1) bits.
        // eg: tree height is 3, which has total 2^2 leaves, the leave can ben indexed as (00, 01, 10, 11).
        // a. turn the index into binary form with (height-1) bits.
        let mut path = convert_to_binary(&path_len, index);
        path.reverse(); // make path from root to left.

        // b. according the path, we can found out the MerkleProof of the indexed leaf, which just need to collect the bro-node.
        //    We'll collect the bro-node by the path. Collect the left child is 1, the right child is 0.

        let mut values = Vec::with_capacity(self.height);
        let root_hash = self.root.get_hash();

        let mut cur_node = &self.root;

        println!("{:?}", path);
        // for now the hash values are collected from root to leaf.
        for p in path {
            // let p = path.get(path_len - i).unwrap();

            match cur_node {
                TreeNode::Leaf { hash, value } => panic!("Never reach leaf"),
                TreeNode::Node { hash, left, right } => {
                    // collect the right as bro-node.
                    if p == 0 {
                        values.push(right.get_hash());
                        cur_node = left.as_ref();
                    } else {
                        println!("value:{:?}", left.get_hash());
                        values.push(left.get_hash());
                        cur_node = right.as_ref();
                    }
                }
            }
        }

        // reverse the hash values to make sure it's from leaf to root
        values.reverse();

        MerkleProof {
            root: root_hash,
            children: values,
        }
    }

    // open.
    // The challenge maybe not in values, so return empty children.
    pub fn open(&self, challenge: &Scalar) -> MerkleProof {
        let mut values = Vec::with_capacity(self.height - 1);
        let root_hash = self.root.get_hash();
        Self::dfs(&self.root, &challenge, &mut values);
        if values.is_empty() {
            // log! todo
        }
        MerkleProof {
            root: root_hash,
            children: values,
        }
    }

    fn dfs(root: &TreeNode, target: &Scalar, res: &mut Vec<Scalar>) -> bool {
        match root {
            TreeNode::Leaf { hash, value } => {
                if value == target {
                    true
                } else {
                    false
                }
            }
            TreeNode::Node { hash, left, right } => {
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

    // Returns the root hash of Merkle tree
    pub fn root_hash(&self) -> Scalar {
        self.root.get_hash()
    }

    // Returns the height of Merkle tree
    pub fn height(&self) -> usize {
        self.height
    }

    // Leaf nodes: if tree height is h, so the number of leaf nodes will be `2^h`
    pub fn leaves_num(&self) -> usize {
        1 << (self.height - 1)
    }

    // Total nodes: A tree of height h has total nodes = 2^(h+1)–1
    pub fn nodes_num(&self) -> usize {
        2 ^ self.height - 1
    }

    pub fn verify(challenge: &Scalar, proof: &MerkleProof) {
        let target = proof.root;
        let actual = if proof.children.is_empty() {
            // The challenge maybe not in values, just verify the root.
            proof.root
        } else {
            let leaf_hash = Keccak256Hash::hash(&challenge);
            proof
                .children
                .iter()
                .fold(leaf_hash, |acc, eval| Keccak256Hash::hash(&acc.add(&eval)))
        };
        assert_eq!(target, actual, "Verifier: verify failed!")
    }

    // equal the commit, by open it by index of values.
    pub fn verify_by_index(&self, index: usize, proof: &MerkleProof) {
        // index belong [0, leaves_num).
        assert!(index >= 0 && index < self.leaves_num(), "Wrong leaf index");

        let path_len = (self.height - 1);
        // 1. get leaf-root path,
        // Suppose the left child is 0, the right child is 1, so the path can be indexed as binary form with (height-1) bits.
        // eg: tree height is 3, which has total 2^2 leaves, the leave can ben indexed as (00, 01, 10, 11).
        // a. turn the index into binary form with (height-1) bits.
        let mut path = convert_to_binary(&path_len, index);
        // make path from root to left.
        path.reverse();
        // to make sure iter can reach the leaf layer.
        path.push(2);

        // b. found out the target left.
        let mut cur_node = &self.root;
        let mut challenge = Scalar::zero();
        for p in path {
            match cur_node {
                TreeNode::Leaf { hash, value } => challenge = value.clone(),
                TreeNode::Node { hash, left, right } => {
                    assert!(p != 2);
                    // collect the right as bro-node.
                    if p == 0 {
                        cur_node = left.as_ref();
                    } else {
                        cur_node = right.as_ref();
                    }
                }
            }
        }
        println!("target: {:?}", challenge);
        Self::verify(&challenge, proof);
    }
}

#[cfg(test)]
mod test {
    use crate::merkle_tree::proof::MerkleProof;
    use crate::merkle_tree::MerkleTree;
    use crate::poly::random_poly;
    use crate::utils::{random_chars, random_scalars};
    use bls12_381::Scalar;
    use ff::PrimeField;
    use rand_core::{OsRng, RngCore};
    use std::fmt::Debug;

    #[test]
    fn test_init_merkle_tree() {
        let poly = random_poly(3);
        println!("chars:{:?}", poly);
        let merkle_tree = MerkleTree::commit(poly.coeffs());
        println!("merkle tree: {:?}", merkle_tree);
    }

    #[test]
    fn test_commit_and_verify() {
        let coeffs = vec![
            Scalar::one(),
            Scalar::from_u128(12),
            Scalar::one(),
            Scalar::from_u128(13),
        ];
        let merkle_tree = MerkleTree::commit(coeffs);
        // println!("merkle tree: {:?}", merkle_tree);

        // MerkleTree {
        //     root: Node {
        //         hash: 0x4053ef94c1db0c3a6159b84891f03ee40b5aaca60091f6e438b7b653cf1b6f20,
        //         left: Node {
        //             hash: 0x5d3b8160daf88b74a74b4a5b91ce4eaea2f64628d6c8f4717330d7734eb0f2f0,
        //             left: Leaf {
        //                 hash: 0x38a2f65eb883578ccc8a27acd26c6646d22fbbaa09e533726b84bd7d9ff94c87,
        //                 value: 0x0000000000000000000000000000000000000000000000000000000000000001
        //             },
        //             right: Leaf {
        //                 hash: 0x33feef36be1c5c0384ecaba81a839c2126444a9dec203df90fa6b8ec2fdeaa87,
        //                 value: 0x000000000000000000000000000000000000000000000000000000000000000c
        //             }
        //         },
        //         right: Node {
        //             hash: 0x56108a065ccd17f0706ef2fa4aa8b80620d7490c9cab818b25b48b39c58594fa,
        //             left: Leaf {
        //                 hash: 0x38a2f65eb883578ccc8a27acd26c6646d22fbbaa09e533726b84bd7d9ff94c87,
        //                 value: 0x0000000000000000000000000000000000000000000000000000000000000001
        //             },
        //             right: Leaf {
        //                 hash: 0x0e2c9965653910c8765b9b7f6eb348643c6da2e58d76a165cd14dfe960e1d418,
        //                 value: 0x000000000000000000000000000000000000000000000000000000000000000d
        //             }
        //         }
        //     },
        //     height: 2
        // }
        let challenge = Scalar::one();
        // MerkleProof {
        // 	children: [
        //      0x33feef36be1c5c0384ecaba81a839c2126444a9dec203df90fa6b8ec2fdeaa87,
        //      0x56108a065ccd17f0706ef2fa4aa8b80620d7490c9cab818b25b48b39c58594fa
        // ],
        // 	root: 0x4053ef94c1db0c3a6159b84891f03ee40b5aaca60091f6e438b7b653cf1b6f20
        // }
        let proof = merkle_tree.open(&challenge);
        println!("{:?}", proof);
        // correct

        MerkleTree::verify(&challenge, &proof);
    }

    #[test]
    fn test_commit_and_verify_by_index() {
        let coeffs = vec![
            Scalar::one(),
            Scalar::from_u128(12),
            Scalar::zero(),
            Scalar::from_u128(13),
        ];
        let merkle_tree = MerkleTree::commit(coeffs);
        println!("merkle tree: {:?}", merkle_tree);

        // merkle tree: MerkleTree {
        //     root: Node {
        //         hash: 0x5e7042679f5529053b7fff8b5578fd6d332070daf6595fba220475075e3f6ca6,
        //         left: Node {
        //             hash: 0x1c40f88c081d9412450a3b26f15202a9cb65b87f28a1a7247c3825d22de9fd16,
        //             left: Leaf {
        //                 hash: 0x38a2f65eb883578ccc8a27acd26c6646d22fbbaa09e533726b84bd7d9ff94c87,
        //                 value: 0x0000000000000000000000000000000000000000000000000000000000000001
        //             },
        //             right: Leaf {
        //                 hash: 0x33feef36be1c5c0384ecaba81a839c2126444a9dec203df90fa6b8ec2fdeaa87,
        //                 value: 0x000000000000000000000000000000000000000000000000000000000000000c
        //             }
        //         },
        //         right: Node {
        //             hash: 0x144cd89c19f04739ec8b8f39cfeb65b16c7a5fbcf1e3043baf2a8a0ef9a2de0e,
        //             left: Leaf {
        //                 hash: 0x64fdc03ab367550778acd60eef4b079fde19310f5db8ddd6700b2bf9ce4a9211,
        //                 value: 0x0000000000000000000000000000000000000000000000000000000000000000
        //             },
        //             right: Leaf {
        //                 hash: 0x0e2c9965653910c8765b9b7f6eb348643c6da2e58d76a165cd14dfe960e1d418,
        //                 value: 0x000000000000000000000000000000000000000000000000000000000000000d
        //             }
        //         }
        //     },
        //     height: 3
        // }
        // let challenge = (OsRng.next_u32() % 4) as usize;
        let challenge = 3;
        println!("{challenge}");
        // [1, 1]
        // value: 0x1c40f88c081d9412450a3b26f15202a9cb65b87f28a1a7247c3825d22de9fd16
        // value: 0x64fdc03ab367550778acd60eef4b079fde19310f5db8ddd6700b2bf9ce4a9211
        // MerkleProof {
        //     children: [
        //          0x64fdc03ab367550778acd60eef4b079fde19310f5db8ddd6700b2bf9ce4a9211,
        //          0x1c40f88c081d9412450a3b26f15202a9cb65b87f28a1a7247c3825d22de9fd16
        //      ],
        //     root: 0x5e7042679f5529053b7fff8b5578fd6d332070daf6595fba220475075e3f6ca6
        // }
        let proof = merkle_tree.open_by_index(challenge);
        println!("{:?}", proof);
        // correct
        merkle_tree.verify_by_index(challenge, &proof);

        // MerkleTree::verify(&challenge, &proof);
    }
}
//
