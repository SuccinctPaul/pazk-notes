use crate::utils::calculate_hash;

#[derive(Default)]
pub struct Verify {
    data: Vec<u64>,
}

impl Verify {
    pub(crate) fn receive(&mut self, data: Vec<u64>) {
        self.data = data;
    }

    pub(crate) fn summary(&self, challenge: u64) -> u64 {
        calculate_hash(&self.data) + challenge
    }

    pub(crate) fn compute(&self, f: fn(&Vec<u64>) -> u64) -> u64 {
        f(&self.data)
    }
}
