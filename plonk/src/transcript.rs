#![allow(clippy::map_flatten)]
#![allow(clippy::ptr_arg)]
use bls12_381::Scalar;
use ff::{Field, PrimeField};

use crate::math::poly::Polynomial;
pub mod default;
mod poseidon;

pub trait Transcript<F: PrimeField> {
    fn append(&mut self, new_data: &[u8]);

    fn challenge(&mut self) -> F;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::transcript::default::Keccak256Transcript;
    use bls12_381::Scalar;
    use ff::Field;

    #[test]
    fn test_challenge() {
        let mut transcript_1 = Keccak256Transcript::<Scalar>::default();
        let challenge_1 = transcript_1.challenge();

        let mut transcript_2 = Keccak256Transcript::<Scalar>::default();
        let challenge_2 = transcript_2.challenge();

        assert_eq!(challenge_2, challenge_1);
    }
}
