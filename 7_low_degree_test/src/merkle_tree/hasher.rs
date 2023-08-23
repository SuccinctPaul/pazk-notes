use bls12_381::Scalar;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Div;

/// calculate the hash of the data
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
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

pub fn calculate_parent_hash2(left: Scalar, right: Scalar) -> u64 {
    let mut sum = left + right;
    let mut s = DefaultHasher::new();
    sum.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod test {
    use crate::merkle_tree::hasher::{calculate_parent_hash, calculate_parent_hash2};
    use bls12_381::Scalar;
    use ff::Field;
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
        let rng = &mut OsRng;
        let left = Scalar::random(rng);
        let right = Scalar::random(rng);

        let parent = calculate_parent_hash2(left, right);
        println!("{:?}", parent);
    }
}
