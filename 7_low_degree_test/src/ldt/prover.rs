use crate::poly::split_poly;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use ark_std::log2;
use bls12_381::Scalar;
use rayon::iter::split;
use std::ops::Mul;
use sumcheck::poly::univar_poly::Polynomial;

#[derive(Default)]
pub struct Proof {
    pub polys: Vec<Polynomial>, // i ∈ {0,log(d)}, fi
    pub last_const: Vec<usize>,
    pub commits: Vec<Scalar>, // todo
}

pub struct Prover {
    poly: Polynomial,
}

impl Prover {
    pub fn init(poly: Polynomial) -> Self {
        Self { poly }
    }

    pub fn prove(&self) {
        let mut transcript = Keccak256Transcript::default();
        let proof = Proof::default();

        // commit phase
        let mut d = log2(poly.degree());
        // P starts from f(x), and for i = 0 sets f0(x) = f(x).
        let p_0 = self.poly;
        let mut p_i = p_0;
        while d >= 0 {
            d = Self::commit(proof, &mut p_i, &mut transcript);
        }
    }

    pub fn split_and_fold(
        proof: &mut Proof,
        p_i: &mut Polynomial,
        transcript: &mut Keccak256Transcript,
    ) -> Polynomial {
        // assert!()
        let (p_L, p_R) = split_poly(&p_i.clone());
        // last iteration
        if p_L.degree() == 0 && p_R.degree() == 0 {
            proof.last_const.push(p_L.coeffs()[&0]);
            proof.last_const.push(p_R.coeffs()[&0]);
            p_i // return itself for end.
        }
        let d = log2(poly.degree());

        // gen challenge
        let challenge = transcript.challenge();
        // compute new poly fi+1, which is the random linear combination of p_L,p_R,
        // f_i_1 = f_L + c*f_R
        let p_i_plus_1 = p_L + p_L.mul(&challenge);
        // todo commits new poly fi+1 ?

        p_i_plus_1
    }

    fn query() {}
}
