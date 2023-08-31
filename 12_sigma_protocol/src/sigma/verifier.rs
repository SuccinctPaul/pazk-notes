use crate::sigma::Proof;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use ff::Field;
use group::prime::PrimeCurveAffine;
use group::{Curve, Group};
use pairing::Engine;
use std::ops::Mul;

#[derive(Default)]
pub struct Verifier<E: Engine> {
    h: E::G1,
}

impl<E: Engine> Verifier<E> {
    pub fn init(h: E::G1) -> Self {
        Self { h }
    }

    // check the proof:
    //  a*h^e = g^z  ==>
    //      g^r * (g^w)^e = g^(we+r)  ==>
    //      g^r * g^(we) = g^(we+r)  ==>
    //      g^(we+r) = g^(we+r)
    //
    // NOTE:
    //      a = g^r
    //      z = we+r
    //      h = g^w
    pub fn verify(&self, proof: &Proof<E>) {
        // todo meet bug.
        let lhs = proof.a * (self.h * proof.e);
        let rhs = E::G1Affine::generator() * proof.z;
        assert_eq!(lhs, rhs, "Verifier: verify failed.")
    }
}
