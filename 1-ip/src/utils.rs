use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// calculate the hash of the data
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub enum ComputeType {
    Sum,
}
