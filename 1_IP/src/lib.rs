/// The interactive proofs case(1.2.1) in chapter 1
/// In this case, business store data in cloud_provider. Later, before business wants wanna do some computation on cloud_provider, the business check the `data` first.
mod prover;
mod utils;
mod verify;

use rand_core::{OsRng, RngCore};
use std::iter;
use std::iter::{once, Sum};

use crate::prover::Prover;
use crate::utils::{calculate_hash, ComputeType};
use crate::verify::Verify;

pub(crate) struct Data {
    name: String,
    index: u64,
    data: Vec<u64>,
}

// Completeness of the IP means that if the cloud correctly runs the program on the data and follows
// the prescribed protocol, then the user will be convinced to accept the answer as valid.
#[test]
fn completeness() {
    println!("storage data");

    // init business and clouder provider
    let mut business = Prover::default();
    let mut cloud_provider = Verify::default();

    // send & receive data
    let data = business.send();
    cloud_provider.receive(data.clone());
    println!("received data\n");

    // challenge and response about data
    let challenge = OsRng.next_u64() % 100;
    calculate_hash(&data);
    assert_eq!(
        business.summary + challenge,
        cloud_provider.summary(challenge)
    );
    println!("verified equal");

    fn sum(datas: &Vec<u64>) -> u64 {
        datas.iter().sum()
    }

    assert_eq!(sum(&data), cloud_provider.compute(sum));
    println!("completeness");
}

// Soundness of the IP means that if the cloud returns the wrong output, then the user will reject the answer
// as invalid with high probability no matter how hard the cloud works to trick the user into accepting the answer as valid.
#[test]
fn soundness() {
    println!("storage datas");

    // init business and clouder provider
    let mut business = Prover::default();
    let mut cloud_provider = Verify::default();

    // send & receive data
    let raw_data = business.send();

    let mut data = raw_data.clone();
    println!("Add the mess info");
    data.push(OsRng.next_u64());
    cloud_provider.receive(data.clone());
    println!("received datas\n");

    // challenge and response about data
    let challenge = OsRng.next_u64() % 100;
    calculate_hash(&data);
    assert_ne!(
        business.summary + challenge,
        cloud_provider.summary(challenge)
    );
    println!("verified non equal");

    fn sum(datas: &Vec<u64>) -> u64 {
        datas.iter().sum()
    }

    assert_ne!(sum(&raw_data), cloud_provider.compute(sum));
    println!("Soundness");
}
