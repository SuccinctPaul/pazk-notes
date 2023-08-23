pub mod hasher;
pub mod node;
pub mod proof;

use crate::merkle_tree::hasher::{calculate_hash, calculate_parent_hash};
use crate::merkle_tree::node::TreeNode;
use crate::merkle_tree::proof::Proof;
use crate::utils::convert_to_binary;
use ark_std::log2;
use std::cmp::Ordering;

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

        let root = cur.remove(0);

        MerkleTree { root, height }
    }

    // equal the commit, by open it by index of values.
    pub fn open(&self, index: usize) -> Proof {
        // index belong [0, leaves_num).
        assert!(index >= 0 && index < self.leaves_num(), "Wrong leaf index");

        let path_len = (self.height - 1);
        // get leaf-root path,
        // Suppose the left child is 0, the right child is 1, so the path can be indexed as binary form with (height-1) bits.
        // eg: tree height is 3, which has total 2^2 leaves, the leave can ben indexed as (00, 01, 10, 11).
        // a. turn the index into binary form with (height-1) bits.
        let path = convert_to_binary(&path_len, index);

        // b. according the path, we can found out the proof of the indexed leaf, which just need to collect the bro-node.
        //    We'll collect the bro-node by the path. Collect the left child is 1, the right child is 0.

        let mut values = Vec::with_capacity(path_len);
        let root_hash = self.root.get_hash();

        let mut cur_node = &self.root;

        // for now the hash values are collected from root to leaf.
        for i in (0..path_len) {
            let p = path.get(path_len - i).unwrap();

            match cur_node {
                TreeNode::Leaf { hash, value } => panic!("Never reach leaf"),
                TreeNode::Node { hash, left, right } => {
                    // collect the right as bro-node.
                    if *p == 0 {
                        values.push(right.get_hash());
                        cur_node = left.as_ref();
                    } else {
                        values.push(left.get_hash());
                        cur_node = right.as_ref();
                    }
                }
            }
        }

        // reverse the hash values to make sure it's from leaf to root
        values.reverse();

        Proof {
            root: root_hash,
            children: values,
        }
    }

    // commit and open.
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
        2 ^ (self.height - 1)
    }

    // Total nodes: A tree of height h has total nodes = 2^(h+1)–1
    pub fn nodes_num(&self) -> usize {
        2 ^ self.height - 1
    }
}

#[cfg(test)]
mod test {
    use crate::merkle_tree::proof::Proof;
    use crate::merkle_tree::MerkleTree;
    use crate::utils::{random_chars, random_scalars};

    #[test]
    fn test_init_merkle_tree() {
        let chars = random_chars(3);
        println!("chars:{:?}", chars);
        let merkle = MerkleTree::init(chars);
        println!("merkle tree: {:?}", merkle);
    }

    #[test]
    fn test_init_merkle_tree_by_field() {
        let chars = random_scalars(3);
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

        let actual = merkle_tree.commit(&challenge);

        assert_eq!(target, actual);
    }
}
