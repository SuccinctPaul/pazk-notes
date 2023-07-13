/// The interactive proofs case(1.2.1) in chapter 1
mod prover;
mod utils;
mod verify;

use rand_core::{OsRng, RngCore};
use std::iter;
use std::iter::Sum;

use crate::prover::Prover;
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
    println!("send data");
    let data = business.send();
    cloud_provider.receive(data.clone());
    println!("received data");

    // challenge and response about data
    let challenge = OsRng.next_u64();
    println!("challenge");
    assert_eq!(
        business.summary + challenge,
        cloud_provider.summary(challenge)
    );
    println!("verified data");

    // computation function: f

    fn sum(datas: &Vec<u64>) -> u64 {
        datas.iter().sum()
    }

    cloud_provider.compute(sum);

    // assert_eq!(sum_fn(data), cloud_provider.compute(sum_fn));
    // println!("completeness");
}
