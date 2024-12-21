// Verifies that `points` exists in `proof`

use crate::math::poly::Polynomial;
use crate::pcs::kzg::param::ParamKzg;
use crate::pcs::kzg::KZGProof;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use bls12_381::Scalar;
use ff::{Field, PrimeField};
use group::prime::PrimeCurveAffine;
use group::Curve;
use pairing::Engine;
use std::fmt::Debug;
use std::ops::Neg;

pub struct Verifier<E: Engine> {
    param: ParamKzg<E>,
}

impl<E: Engine> Verifier<E> {
    pub fn init(param: ParamKzg<E>) -> Self {
        Self { param }
    }

    // verify proof by pairing:
    //     check e(π, [x−z]_2 ) = e(cm−[p(z)]_1, g2)
    //          => e(g1, g2)^{q(x)*(x-z)} = e(g1, g2)^{p(x)-p(z)}
    //          => q(x)*(x-z) = p(x)-p(z)
    //          -> same as Prove::open.
    pub fn verify(&self, proof: KZGProof<E>) {
        let vanish_poly = |z: E::Fr| {
            let coeffs = vec![z.neg(), E::Fr::ONE];
            Polynomial::from_coeffs(coeffs)
        };

        // 1. challenge z.
        let mut transcript_1 = Keccak256Transcript::<E::Fr>::default();
        let z = transcript_1.challenge();

        // 2. prepare poly for pairing.
        //  compute: x-z
        let vanish_poly = vanish_poly(z);
        let eval_poly = Polynomial::from_coeffs(vec![proof.eval]);

        // 3.pairing
        // e(pi, [x-z]2)
        let e1 = E::pairing(
            &proof.pi.to_affine(),
            &self.param.eval_at_tau_g2(&vanish_poly).to_affine(),
        );
        // e(cm-[p(z)]1, g2)
        let e2 = E::pairing(
            &(proof.cm - self.param.eval_at_tau_g1(&eval_poly)).to_affine(),
            &E::G2Affine::generator(),
        );
        assert_eq!(e1, e2, "Verify: failed for pairing.");
    }
}
