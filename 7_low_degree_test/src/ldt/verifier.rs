use crate::ldt::LDTProof;
use crate::merkle_tree::proof::MerkleProof;
use crate::merkle_tree::MerkleTree;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use ark_std::log2;
use bls12_381::Scalar;
use ff::PrimeField;

pub struct Verifier {
    pub target_deg: usize, // target degree
    z: Scalar,             // The origin value for evaluate.
    merkle_c: Scalar,      // the commit challenge.
}

impl Verifier {
    pub fn init(target_deg: usize, z: Scalar, merkle_c: Scalar) -> Self {
        Self {
            target_deg,
            z,
            merkle_c,
        }
    }

    pub fn verify(&self, proof: LDTProof) {
        let mut transcript = Keccak256Transcript::default();
        let d = log2(self.target_deg) as usize;

        assert_eq!(proof.commits.len(), d - 1);
        assert_eq!(proof.evals.len(), d - 1);
        let commits = &proof.commits;
        let evals = &proof.evals;
        let mut z_i = self.z; // z^1 = z^(2^0)

        let mut merkle_c_i = self.merkle_c;

        let two_inv = Scalar::from_u128(2).invert().unwrap();
        for i in 0..(d - 1) {
            println!("");
            println!("round: i: {:?}", i);
            println!("evals.len: {:?}", evals.len());
            // 1. obtain fi_L(z^2) ,fi_R (z^2) with: fi(z) = fi_L(z^2) + z fi_R (z^2)
            let (f_i_z, f_i_neg_z): (Scalar, Scalar) = *evals.get(i).unwrap();
            let f_i_L = (f_i_z.add(&f_i_neg_z)).mul(&two_inv);
            // calc fiR
            let double_z_inv = z_i.double().invert().unwrap();
            let f_i_R = (f_i_z.sub(&f_i_neg_z)).mul(&double_z_inv);

            if d == 1 || (d - 2) == i {
                // 2. last round check
                assert_eq!(
                    proof.last_const,
                    (f_i_L, f_i_R),
                    "Verifier: Last round check failed."
                );
            } else {
                // 2. check fi+1(z^2) = fi_L(z^2) + αi*fi_R(z^2)
                let alpha = transcript.challenge();
                let f_i_plus_1 = f_i_L + alpha * f_i_R;
                let (target_f_i_plus_1, _): (Scalar, Scalar) = *evals.get(i + 1).unwrap();
                assert_eq!(
                    f_i_plus_1, target_f_i_plus_1,
                    "Verifier: round-{i} check failed."
                );

                // 3. verify the cm todo
                MerkleTree::verify(&merkle_c_i, commits.get(i).unwrap());

                // prepare for next round
                z_i = z_i.mul(&z_i); // z^(2^i), Important !!!
                merkle_c_i.double();
            }
        }
    }
}
