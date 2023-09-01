use bls12_381::{Bls12, Scalar};
use ff::Field;
use group::Group;
use pairing::Engine;
use rand_core::OsRng;
use std::ops::Mul;

// cm = Com_{G,H}(m, z)= G*m + H*z
fn commit<E: Engine>(m: &E::Fr, r: &E::Fr, h: &E::G1) -> E::G1 {
    E::G1::generator().mul(m) + h.mul(r)
}

#[test]
fn test_perseden_commit() {
    // 1. mock setup
    let mut rng = OsRng;
    let H = <Bls12 as Engine>::G1::random(rng);

    let m = Scalar::random(rng);
    let r = Scalar::random(rng);

    // 2.prover commit
    let cm = commit::<Bls12>(&m, &r, &H);

    // 3. mock-open. Send v,r to verifier.

    // 4. verify verify by recompute cm
    let cm_1 = commit::<Bls12>(&m, &r, &H);
    assert_eq!(cm, cm_1);
}
