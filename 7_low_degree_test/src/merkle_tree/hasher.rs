use bls12_381::Scalar;
use ff::PrimeField;
use sha3::{Digest, Keccak256};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::marker::PhantomData;
use std::ops::Div;

// abstraction to set the hash function used
pub trait ScalarHash<F: PrimeField>: Clone {
    fn hash(inputs: &F) -> F;
    fn hashes(inputs: &[F]) -> F;
}

#[derive(Clone, Copy, Debug)]
pub struct Keccak256Hash<F: PrimeField> {
    _marker: PhantomData<F>,
}

impl ScalarHash<Scalar> for Keccak256Hash<Scalar> {
    // same as calculate_hash, this is for Scalar
    fn hash(input: &Scalar) -> Scalar {
        // hash
        let mut h = Keccak256::new();
        h.update(input.to_repr().as_ref());

        // let r = h.finalize().as_slice();
        let slice: [u8; 32] = h.finalize().as_slice().try_into().unwrap();
        // get_scalar
        let bytes = [slice.clone(), slice]
            .concat()
            .as_slice()
            .try_into()
            .unwrap();
        Scalar::from_bytes_wide(&bytes)
    }

    // same as calculate_parent_hash, this is for Scalar
    fn hashes(inputs: &[Scalar]) -> Scalar {
        // hash
        let mut h = Keccak256::new();
        for x in inputs {
            h.update(x.to_repr().as_ref());
        }

        // let r = h.finalize().as_slice();
        let slice: [u8; 32] = h.finalize().as_slice().try_into().unwrap();
        // get_scalar
        let bytes = [slice.clone(), slice]
            .concat()
            .as_slice()
            .try_into()
            .unwrap();
        Scalar::from_bytes_wide(&bytes)
    }
}

/// calculate the hash of the data
pub fn calculate_hash<T: std::hash::Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
pub fn calculate_parent_hash(left: u64, right: u64) -> u64 {
    let mut sum: u128 = (left.div(2) + right / 2) as u128;
    let mut s = DefaultHasher::new();
    sum.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod test {
    use crate::merkle_tree::hasher::{calculate_parent_hash, Keccak256Hash, ScalarHash};
    use bls12_381::Scalar;
    use ff::{Field, PrimeField};
    use rand_core::{OsRng, RngCore};

    #[test]
    fn test_calculate_parent_hash() {
        let rng = &mut OsRng;
        let left = rng.next_u64();
        let right = rng.next_u64();

        let parent = calculate_parent_hash(left, right);
        println!("{:?}", parent);
    }

    #[test]
    fn test_calculate_parent_hash_with_scalar() {
        let left = Scalar::random(&mut OsRng);
        let right = Scalar::random(&mut OsRng);

        let parent = Keccak256Hash::hash(&left.add(&right));
        println!("{:?}", parent);

        let left = Scalar::from_u128(10);
        let right = Scalar::from_u128(12);

        let parent = Keccak256Hash::hashes(&[left, right]);
        println!("{:?}", parent);
    }
}
