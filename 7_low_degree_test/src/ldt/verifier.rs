use crate::ldt::prover::Proof;
use bls12_381::Scalar;

pub struct Verifier {
    pub target_deg: usize, // target degree
    z: Scalar,
}

impl Verifier {
    pub fn verify(&self, d: usize, proof: &Proof) {
        let commits = proof.commits;
        let opens = proof.opens;
        let (const_L, const_R) = proof.last_const;

        for i in 0..d {
            // 1. check fi(z) = fi_L(z^2) + z fi_R (z^2)
            let (f_i_z, f_i_neg_z) = opens.get(i).unwrap();
            let f_i_L = (f_i_z + f_i_neg_z) / 2;
            let f_i_R = (f_i_z - f_i_neg_z) / 2;

            assert_eq!(
                f_i_z,
                f_i_L + z * f_i_R,
                "Fail to check fi(z) = fi_L(z^2) + z fi_R (z^2)"
            );

            // 2. verify the cm

            // 3. prepare for next round.
            // fi+1(z^2) = fi_L(z^2) + αi*fi_R(z^2)
            let c = challenge;
            let f_i_plus_1 = f_i_L + c * f_i_R;
        }
    }
}
