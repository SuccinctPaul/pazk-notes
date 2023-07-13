use crate::utils::{calculate_hash, ComputeType};

#[derive(Default)]
pub struct Verify {
    data: Vec<u64>,
}

impl Verify {
    pub(crate) fn receive(&mut self, data: Vec<u64>) {
        self.data = data;
    }

    pub(crate) fn summary(&self) -> u64 {
        calculate_hash(&self.data)
    }

    pub(crate) fn compute(&self, compute: ComputeType) -> u64 {
        match compute {
            ComputeType::Sum => {
                self.data.iter().sum()
            }
        }
    }
}
