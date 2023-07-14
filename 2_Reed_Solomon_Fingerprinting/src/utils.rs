use bls12_381::Scalar;
use ff::Field;
use rand_core::OsRng;
use std::fs::File;

pub(crate) fn dump_field_data(file_name: &String, length: u64) {
    let rng = OsRng;
    let file = File::open(file_name);

    for x in (0..length) {
        let data = Scalar::random(rng);
    }
}

pub(crate) fn read_from_file<F: Field>(file_name: &String) -> Vec<F> {
    todo!()
}
