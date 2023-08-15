use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// calculate the hash of the data
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn calculate_parent_hash(left: u64, right: u64) -> u64 {
    let mut sum: u128 = (left / 2 + right / 2) as u128;
    let mut s = DefaultHasher::new();
    sum.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod test {
    use crate::merkle_tree::hasher::calculate_parent_hash;
    use rand_core::{OsRng, RngCore};

    #[test]
    fn test_calculate_parent_hash() {
        let rng = &mut OsRng;
        let left = rng.next_u64();
        let right = rng.next_u64();

        let parent = calculate_parent_hash(left, right);
        println!("{:?}", parent);
    }
}
