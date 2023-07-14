use crate::utils::{dump_field_data, read_from_file};
use ff::Field;
use rand_core::{OsRng, RngCore};

/// The Reed-Solomon Fingerprinting case(2.1) in chapter 2
mod prover;
mod utils;
mod verify;

#[derive(Default, Eq, PartialEq)]
pub(crate) struct Person<F: Field> {
    data: Vec<F>,
}

impl<F: Field> Person<F> {
    pub(crate) fn new(file_name: &String) -> Self<F> {
        let data = read_from_file(file_name);
        Self { data }
    }
}

const file_A: String = String::from("file_A.bin");
const file_B: String = String::from("file_B.bin");

fn prepare_data() {
    let length = 10 + OsRng.next_u64() % 90;
    dump_field_data(&file_A, length);
    dump_field_data(&file_B, 2 * length);
}

#[test]
fn completeness() {
    prepare_data();

    let Alice = Person::new(&file_A);
    let Bob = Person::new(&file_A);

    // use the prover trait by alice

    assert!();
}

#[test]
fn soundness() {
    prepare_data();

    let Alice = Person::new(&file_A);
    let Bob = Person::new(&file_B);

    // use the prover trait by alice

    assert!();
}
