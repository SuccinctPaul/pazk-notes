use crate::utils::{dump_field_data, read_from_file};
use bls12_381::Scalar;
use ff::Field;
use rand_core::{OsRng, RngCore};

/// The Reed-Solomon Fingerprinting case(2.1) in chapter 2
mod prover;
mod utils;
mod verify;

#[derive(Default, Eq, PartialEq)]
pub(crate) struct Person {
    data: Vec<Scalar>,
}

impl Person {
    pub(crate) fn new(file_name: &str) -> Self {
        let data = read_from_file(file_name);
        Self { data }
    }
}

const file_A: &str = "file_A.bin";
const file_B: &str = "file_B.bin";

fn prepare_data() {
    let length = 10 + OsRng.next_u64() % 90;
    dump_field_data(file_A, 10 + length);
    dump_field_data(file_B, 10 + 2 * length);
}

#[test]
fn completeness() {
    prepare_data();

    let Alice = Person::new(file_A);
    let Bob = Person::new(file_A);

    // use the prover trait by alice

    // assert!();
}

#[test]
fn soundness() {
    prepare_data();

    let Alice = Person::new(file_A);
    let Bob = Person::new(file_B);

    // use the prover trait by alice

    // assert!();
}
