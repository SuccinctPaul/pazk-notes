use crate::ldt::LDTProof;
use crate::merkle_tree::{proof::MerkleProof, MerkleTree};
use crate::poly::{split_poly, Polynomial};
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use ark_std::log2;
use bls12_381::Scalar;
use rayon::iter::split;
use std::iter::Scan;
use std::ops::{Add, Mul};

pub struct Prover {
    poly: Polynomial,
    z: Scalar,
    merkle_c: Scalar,
}

impl Prover {
    pub fn init(poly: Polynomial, z: Scalar, merkle_c: Scalar) -> Self {
        Self { poly, z, merkle_c }
    }

    pub fn prove(&self) {
        let mut transcript = Keccak256Transcript::default();
        let mut proof = LDTProof::default();

        // iter for d rounds.
        let mut d = log2(self.poly.degree());

        // P starts from f(x), and for i = 0 sets f0(x) = f(x).
        let p_0 = self.poly;
        let mut p_i = p_0;
        let mut z_i = self.z; // z^1 = z^(2^0)
        for i in 0..d {
            p_i = Self::split_and_fold(&mut transcript, &mut proof, &p_i, z_i.clone());
            z_i = z_i.mul(&z_i); // z^(2^i)
        }
    }

    pub fn split_and_fold(
        &self,
        transcript: &mut Keccak256Transcript,
        proof: &mut LDTProof,
        p_i: &Polynomial,
        z_i: Scalar,
    ) -> Polynomial {
        assert_eq!(p_i.degree(), 0, "poly.degree=0, can't split_and_fold");
        // 1. split
        let (p_L, p_R) = split_poly(&p_i);
        // last iteration
        if p_L.degree() == 0 && p_R.degree() == 0 {
            proof.last_const = (*p_L.coeffs().get(0).unwrap(), *p_R.coeffs().get(0).unwrap());
            return p_L; // return itself for end.
        }

        // 2. fold
        //  gen challenge: alpha
        let alpha_i = transcript.challenge();
        // compute new poly fi+1, which is the random linear combination of p_L,p_R,
        //      f_i_1 = f_L + c*f_R
        let p_i_plus_1 = p_L.add(&p_L.mul(&alpha_i));

        // 3. commit phase
        //  merkle tree commit the poly fi+1
        let merkle_tree = MerkleTree::commit(p_i_plus_1.coeffs().clone());
        // 4. query phase
        let cm_i = merkle_tree.open(&self.merkle_c);

        // 5. evaluate
        let f_z = p_i_plus_1.evaluate(z_i.clone());
        let f_neg_z = p_i_plus_1.evaluate(z_i.neg());

        // cache in script
        proof.commits.push(cm_i);
        proof.evals.push((f_z, f_neg_z));

        p_i_plus_1
    }
}
