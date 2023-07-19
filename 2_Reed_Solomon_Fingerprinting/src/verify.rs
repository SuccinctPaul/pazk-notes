use crate::utils::read_from_file;
use crate::Person;
use bls12_381::Scalar;
use ff::Field;
use std::ops::MulAssign;

pub trait Verifier<F: Field> {
    fn verify(&self, r: F, target_v: F) -> bool;
}

impl Verifier<Scalar> for Person {
    fn verify(&self, r: Scalar, fingerprint: Scalar) -> bool {
        // hash
        let mut cur_r = Scalar::one();
        let value = self
            .data
            .iter()
            .enumerate()
            .map(|(_, a)| {
                let res = a.mul(&cur_r);
                cur_r.mul_assign(r);
                res
            })
            .sum();

        // compare
        fingerprint == value
    }
}
