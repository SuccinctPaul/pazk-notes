use crate::transcript::Transcript;
use bls12_381::Scalar;
use ff::{Field, PrimeField};
use sha3::{Digest, Keccak256};
use std::marker::PhantomData;
use std::net::UdpSocket;

pub struct Keccak256Transcript<F: PrimeField> {
    hasher: Keccak256,
    _marker: PhantomData<F>,
}

impl<F: PrimeField> Transcript<F> for Keccak256Transcript<F> {
    fn append(&mut self, new_data: &[u8]) {
        self.hasher.update(&mut new_data.to_owned());
    }

    // auto append and gen challenge
    fn challenge(&mut self) -> F {
        self.append(&[1]);

        let mut result_hash = [0_u8; 32];
        result_hash.copy_from_slice(&self.hasher.finalize_reset());
        result_hash.reverse();
        self.hasher.update(result_hash);
        let sum = result_hash.to_vec().iter().map(|&b| b as u128).sum();
        F::from_u128(sum)
    }
}

impl<F: PrimeField> Default for Keccak256Transcript<F> {
    fn default() -> Self {
        Self {
            hasher: Keccak256::new(),
            _marker: Default::default(),
        }
    }
}
