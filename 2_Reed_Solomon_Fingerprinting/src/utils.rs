use bls12_381::Scalar;
use ff::Field;
use rand_core::OsRng;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, Write};

pub(crate) fn dump_field_data(file_name: &str, length: u64) {
    let rng = OsRng;
    let mut file = File::create(file_name).unwrap();

    for x in (0..length) {
        let data = Scalar::random(rng);
        file.write(&data.to_bytes());
    }
}

// use to load Scalar([u64;4]). And it has 32 bytes(u8).
// We can name that limb: u8, LIMB_SIZE: 32;
pub(crate) fn read_from_file(file_name: &str) -> Vec<Scalar> {
    const LIMB_SIZE: u64 = 32;

    let mut file = File::open(file_name).unwrap();

    let file_bytes_num = fs::metadata(file_name).unwrap().len();
    assert!(file_bytes_num % LIMB_SIZE == 0, "File's broken");

    let result_size = file_bytes_num / 32;

    let mut bytes_buffer = [0_u8; LIMB_SIZE as usize];

    (0..result_size)
        .map(|_| {
            file.read(&mut bytes_buffer);
            Scalar::from_bytes(&bytes_buffer).expect("Convert bytes")
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    const file_name: &str = "test_dump.bin";
    const data_size: u64 = 10;

    #[test]
    fn test_dump_field_data() {
        dump_field_data(file_name, data_size);
    }

    #[test]
    fn test_load_field_data() {
        let data = read_from_file(file_name);
        assert_eq!(data.len() as u64, data_size);
        println!("{:?}", data);
    }

    #[test]
    fn test_file_size() {
        let mut file = File::open(file_name).unwrap();

        // file_len = number of bytes
        let file_len = fs::metadata(file_name).unwrap().len();
        println!("{:?}", file_len);
    }
}
