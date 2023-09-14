use ark_std::iterable::Iterable;
use ff::Field;
use group::prime::PrimeCurveAffine;
use pairing::Engine;
use rand_core::OsRng;

pub fn gen_points<E: Engine>(k: usize) -> Vec<E::G1> {
    let n = 1 << k;

    let tau = E::Fr::random(OsRng);

    // obtain: s, ..., s^i,..., s^n
    let powers_of_tau: Vec<E::Fr> = (0..n)
        .into_iter()
        .scan(E::Fr::ONE, |acc, _| {
            let v = *acc;
            *acc *= tau;
            Some(v)
        })
        .collect();

    let pow_tau_g1: Vec<E::G1> = powers_of_tau
        .iter()
        .map(|tau_pow| E::G1Affine::generator() * tau_pow)
        .collect();
    pow_tau_g1
}

pub fn gen_scalars<E: Engine>(k: usize) -> Vec<E::Fr> {
    let n = 1 << k;

    (0..n).map(|_| E::Fr::random(OsRng)).collect()
}
