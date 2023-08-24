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
}

#[derive(Clone, Copy, Debug)]
pub struct Keccak256Hash<F: PrimeField> {
    _marker: PhantomData<F>,
}

impl<F: PrimeField> ScalarHash<F> for Keccak256Hash<F> {
    fn hash(input: &F) -> F {
        // fn get_scalar() -> C::Scalar {
        let get_scalar = |r: &[u8; 32]| {
            let mut repr = F::Repr::default();
            println!("{:?}", r);
            repr.as_mut().copy_from_slice(r);
            println!("{:?}", repr.as_ref());

            F::from_repr(repr).unwrap()
        };

        // hash
        let mut h = Keccak256::new();
        h.update(input.to_repr().as_ref());

        // let r = h.finalize().as_slice();
        let mut slice: [u8; 32] = h.finalize().as_slice().try_into().unwrap();
        for i in (0..16) {
            slice[i] = 0;
        }
        // todo sometime will meet bug.
        // get_scalar
        get_scalar(&slice)
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
        // [64, 26, 193, 249, 18, 134, 13, 197, 176, 206, 213, 209, 169, 26, 96, 197, 249, 170, 154, 113, 215, 44, 217, 239, 125, 205, 16, 2, 212, 55, 219, 102]
        // [70, 73, 151, 28, 96, 138, 140, 117, 223, 67, 12, 121, 226, 41, 105, 81, 193, 252, 27, 241, 167, 41, 63, 4, 111, 125, 34, 135, 96, 144, 190, 133]
        // 0x66db37d40210cd7defd92cd7719aaaf9c5601aa9d1d5ceb0c50d8612f9c11a40

        let left = Scalar::random(&mut OsRng);
        let right = Scalar::random(&mut OsRng);

        let parent = Keccak256Hash::hash(&left.add(&right));
        println!("{:?}", parent);

        let left = Scalar::from_u128(10);
        let right = Scalar::from_u128(12);

        let parent = Keccak256Hash::hash(&left.add(&right));
        println!("{:?}", parent);
    }
}
