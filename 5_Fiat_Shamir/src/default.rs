use crate::Transcript;
use sha3::{Digest, Keccak256};

pub struct Keccak256Transcript {
    hasher: Keccak256,
}

impl Transcript for Keccak256Transcript {
    fn append(&mut self, new_data: &[u8]) {
        self.hasher.update(&mut new_data.to_owned());
    }

    fn challenge(&mut self) -> [u8; 32] {
        let mut result_hash = [0_u8; 32];
        result_hash.copy_from_slice(&self.hasher.finalize_reset());
        result_hash.reverse();
        self.hasher.update(result_hash);
        result_hash
    }
}

impl Default for Keccak256Transcript {
    fn default() -> Self {
        Self {
            hasher: Keccak256::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bls12_381::Scalar;
    use ff::Field;
    use rand_core::OsRng;

    #[test]
    fn test() {
        let mut rng = OsRng;
        let s_one = Scalar::random(rng);
        let s_two = Scalar::random(rng);

        let mut transcript1 = Keccak256Transcript::default();
        transcript1.append(&s_one.to_bytes());
        transcript1.append(&s_two.to_bytes());

        let challenge_1 = transcript1.challenge();

        let mut transcript2 = Keccak256Transcript::default();
        transcript2.append(&s_one.to_bytes());
        transcript2.append(&s_two.to_bytes());

        let challenge_2 = transcript2.challenge();

        assert_eq!(challenge_1, challenge_2);
    }
}
