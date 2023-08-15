use crate::merkle_tree::hasher::{calculate_hash, calculate_parent_hash};
use crate::merkle_tree::proof::Proof;
use crate::utils::random_chars;

#[derive(Default)]
pub struct Verifier {
    challenge: char,
}

impl Verifier {
    // random char
    pub fn gen_challenge(&mut self) -> char {
        let challenge = random_chars(0).get(0).unwrap().clone();
        self.challenge = challenge.clone();
        challenge
    }

    pub fn verify(&self, proof: &Proof) {
        let target = proof.root;

        let leaf_hash = calculate_hash(&self.challenge);
        let actual = proof.children.iter().fold(leaf_hash, |acc, eval| {
            calculate_parent_hash(acc, eval.clone())
        });
        assert_eq!(target, actual, "Verifier: verify failed!")
    }
}

#[cfg(test)]
mod test {

    use crate::merkle_tree::proof::Proof;
    use crate::merkle_tree::MerkleTree;
    use crate::verifier::Verifier;

    #[test]
    fn test_verify() {
        let chars = vec!['W', '8', 'E', 'X', 'D', '8', 'R', '3'];
        let challenge = 'W';
        let merkle_tree = MerkleTree::init(chars);
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
        let proofs = Proof {
            root: 2997809638824881102,
            children: vec![
                3209422213365730399,  // '8'
                10895954492970826136, // right
                14010322267561343302, // right
            ],
        };

        let verifiy = Verifier { challenge };
        verifiy.verify(&proofs);
    }
}
