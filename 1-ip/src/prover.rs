use crate::utils::calculate_hash;
use rand_core::{OsRng, RngCore};

#[derive(Default)]
pub struct Prover {
    // hash key of data
    pub(crate) summary: u64,
}

impl Prover {
    pub(crate) fn send(&mut self) -> Vec<u64> {
        // generate data
        let data = Self::generate_data();
        println!("data.size:{:?}", data.len());
        // obtain key by hash the data
        self.summary = calculate_hash(&data);
        // send data
        data
    }

    fn generate_data() -> Vec<u64> {
        let mut rng = OsRng;
        let size = 5 + rng.next_u32() % 15;
        (0..size).map(|_| rng.next_u32() as u64).collect::<Vec<_>>()
    }
}
