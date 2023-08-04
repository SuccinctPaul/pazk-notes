#![allow(clippy::map_flatten)]
#![allow(clippy::ptr_arg)]
use bls12_381::Scalar;

use crate::poly::univar_poly::Polynomial;
pub mod default;

pub trait Transcript {
    fn append(&mut self, new_data: &[u8]);

    // generate r1, r2, ..., rv
    fn challenge(&mut self) -> usize;
}

pub(crate) fn poly_to_bytes(poly: &Polynomial) -> Vec<u8> {
    coeffs_to_bytes(&poly.coeffs)
}

fn coeffs_to_bytes(coeffs: &Vec<Scalar>) -> Vec<u8> {
    coeffs
        .iter()
        .map(|c| c.to_bytes())
        .flatten()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::transcript::coeffs_to_bytes;
    use crate::transcript::default::Keccak256Transcript;
    use bls12_381::Scalar;
    use ff::Field;
    use rand_core::OsRng;

    #[test]
    fn test_coeff_to_transcript() {
        let mut rng = OsRng;

        let coeffs = (0..4).map(|_| Scalar::random(rng)).collect::<Vec<_>>();

        // from scalar vector
        let mut transcript_1 = Keccak256Transcript::default();
        for x in coeffs.clone() {
            transcript_1.append(&x.to_bytes());
        }
        let challenge_1 = transcript_1.challenge();

        // from coeffs, as mock of poly
        let mut transcript_2 = Keccak256Transcript::default();
        let bytes = coeffs_to_bytes(&coeffs);
        transcript_2.append(&bytes);
        let challenge_2 = transcript_2.challenge();

        assert_eq!(challenge_2, challenge_1);
    }
}
