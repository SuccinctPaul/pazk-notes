pub mod hasher;
pub mod node;
pub mod proof;

use crate::merkle_tree::hasher::{calculate_hash, calculate_parent_hash};
use crate::merkle_tree::node::TreeNode;
use crate::merkle_tree::proof::Proof;
use ark_std::log2;
use std::cmp::Ordering;

// A Merkle tree is a binary tree, with values of type `T` at the leafs,
// and where every internal node holds the hash of the concatenation of the hashes of its children nodes.
// Note: For convinence, we suppose Merkle tree is a ![complete binary tree](https://www.geeksforgeeks.org/types-of-binary-tree/?ref=lbp)
//      Degree: 2
//      Leaf nodes: if tree height is h, so the number of leaf nodes will be `2^h`
//      Total nodes: A tree of height h has total nodes = 2^(h+1)–1
//      Height of tree: If tree has N nodes, the hight `h=log(N+1)–1=Θ(ln(n))`. From root to leaf: [1,h].
#[derive(Clone, Debug)]
pub struct MerkleTree {
    root: TreeNode, // The root of the inner binary tree
    height: usize,  // The height of the tree
}

impl MerkleTree {
    // Constructs a Merkle Tree from a vector of data.
    // Root = hash_util(left.hash + right.hash)
    pub fn init(values: Vec<char>) -> Self {
        assert!(
            !values.is_empty(),
            "Can't initial MerkleTree from empty vector"
        );
        let leaves_num = values.len();
        let height: usize = log2(leaves_num) as usize;
        assert_eq!(1 << height, leaves_num, "It's not a perfect tree");

        let leaves_nodes = values
            .iter()
            .map(|v| TreeNode::new_leaf(*v))
            .collect::<Vec<TreeNode>>();

        // construct tree by leaves.
        let mut cur = leaves_nodes;
        for i in 0..height {
            let cur_len = cur.len();
            let parant = (0..(cur_len / 2))
                .map(|j| {
                    let left = cur.get(2 * j).unwrap();
                    let right = cur.get(2 * j + 1).unwrap();
                    let parent_hash = calculate_parent_hash(left.get_hash(), right.get_hash());

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

        // while cur.len() > 1 {
        //     let mut parent = Vec::new();
        //     while !cur.is_empty() {
        //         let left = cur.remove(0);
        //         let right = cur.remove(0);
        //
        //         let sum = left.get_hash() + right.get_hash();
        //         let parent_hash = calculate_hash(&sum);
        //
        //         let node = TreeNode::Node {
        //             hash: parent_hash,
        //             left: Box::new(left),
        //             right: Box::new(right),
        //         };
        //
        //         parent.push(node);
        //     }
        //
        //     height += 1;
        //
        //     cur = parent;
        // }

        let root = cur.remove(0);

        MerkleTree { root, height }
    }

    pub fn commit(&self, x: &char) -> Proof {
        let mut values = Vec::with_capacity(self.height - 1);
        let root_hash = self.root.get_hash();
        Self::dfs(&self.root, &x, &mut values);
        Proof {
            root: root_hash,
            children: values,
        }
    }

    fn dfs(root: &TreeNode, target: &char, res: &mut Vec<u64>) -> bool {
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
    pub fn root_hash(&self) -> u64 {
        self.root.get_hash()
    }

    // Returns the height of Merkle tree
    pub fn height(&self) -> usize {
        self.height
    }

    // Leaf nodes: if tree height is h, so the number of leaf nodes will be `2^h`
    pub fn leaves_num(&self) -> usize {
        2 ^ self.height
    }

    // Total nodes: A tree of height h has total nodes = 2^(h+1)–1
    pub fn nodes_num(&self) -> usize {
        2 ^ (self.height + 1) - 1
    }
}

#[cfg(test)]
mod test {
    use crate::merkle_tree::proof::Proof;
    use crate::merkle_tree::MerkleTree;
    use crate::prover::Prover;
    use crate::utils::random_chars;
    use crate::verifier::Verifier;

    #[test]
    fn test_init_merkle_tree() {
        let chars = random_chars(3);
        println!("chars:{:?}", chars);
        let merkle = MerkleTree::init(chars);
        println!("merkle tree: {:?}", merkle);
    }

    #[test]
    fn test_commit() {
        let chars = vec!['W', '8', 'E', 'X', 'D', '8', 'R', '3'];
        let challenge = 'W';
        let merkle_tree = MerkleTree::init(chars.clone());
        {
            // MerkleTree {
            //     root: Node {
            //         hash: 2997809638824881102,
            //         left: Node {
            //             hash: 13957922012229917015,
            //             left: Node {
            //                 hash: 10153464161223545464,
            //                 left: Leaf {
            //                     hash: 5949921715258702887,
            //                     value: 'W',
            //                 },
            //                 right: Leaf {
            //                     hash: 3209422213365730399,
            //                     value: '8',
            //                 },
            //             },
            //             right: Node {
            //                 hash: 10895954492970826136,
            //                 left: Leaf {
            //                     hash: 15042720617947887434,
            //                     value: 'E',
            //                 },
            //                 right: Leaf {
            //                     hash: 15818208776807171099,
            //                     value: 'X',
            //                 },
            //             },
            //         },
            //         right: Node {
            //             hash: 14010322267561343302,
            //             left: Node {
            //                 hash: 2385074442875957999,
            //                 left: Leaf {
            //                     hash: 6796667025961897532,
            //                     value: 'D',
            //                 },
            //                 right: Leaf {
            //                     hash: 3209422213365730399,
            //                     value: '8',
            //                 },
            //             },
            //             right: Node {
            //                 hash: 168356471189691628,
            //                 left: Leaf {
            //                     hash: 5573041882718737857,
            //                     value: 'R',
            //                 },
            //                 right: Leaf {
            //                     hash: 15080230035883566959,
            //                     value: '3',
            //                 },
            //             },
            //         },
            //     },
            //     height: 3,
            // }
            // println!("merkle_tree:{:#?}", merkle_tree);
        }
        let target = Proof {
            root: 2997809638824881102,
            children: vec![
                3209422213365730399,  // '8'
                10895954492970826136, // right
                14010322267561343302, // right
            ],
        };

        let prover = Prover {
            values: chars,
            merkle_tree: merkle_tree,
        };
        let (has, actual) = prover.has_x(&challenge);
        if has {
            let actual = actual.unwrap();
            assert_eq!(target, actual);
        }
    }
}
