use crate::transcript::Transcript;
use bls12_381::Scalar;
use ff::PrimeField;
use sha3::{Digest, Keccak256};
use std::net::UdpSocket;

pub struct Keccak256Transcript {
    hasher: Keccak256,
}

impl Transcript for Keccak256Transcript {
    fn append(&mut self, new_data: &[u8]) {
        self.hasher.update(&mut new_data.to_owned());
    }

    fn challenge(&mut self) -> Scalar {
        let mut result_hash = [0_u8; 32];
        result_hash.copy_from_slice(&self.hasher.finalize_reset());
        result_hash.reverse();
        self.hasher.update(result_hash);
        let sum = result_hash.to_vec().iter().map(|&b| b as u128).sum();
        Scalar::from_u128(sum)
    }
}

impl Default for Keccak256Transcript {
    fn default() -> Self {
        Self {
            hasher: Keccak256::new(),
        }
    }
}
