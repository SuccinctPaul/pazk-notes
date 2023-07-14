use crate::utils::read_from_file;
use crate::Person;
use ff::Field;
use std::fs::read;

trait Prover<F: Field> {
    fn hash(&self, f: F) -> F;
}

impl<F: Field> Prover<F> for Person<F> {
    fn hash(&self, r: F) -> F {
        self.data
            .iter()
            .enumerate()
            .map(|i, a| {
                let r_i = r.pow_vartime(i);
                r_i.mul(a)
            })
            .sum()
    }
}
