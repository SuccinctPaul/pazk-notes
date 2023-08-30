use crate::msm::small_multiexp;
use crate::poly::Polynomial;
use ff::{Field, PrimeField};
use group::prime::PrimeCurveAffine;
use pairing::Engine;
use rand_core::OsRng;
use std::fmt::Debug;

// The SRS
#[derive(Clone)]
pub struct ParamKzg<E: Engine> {
    pub(crate) k: usize,
    pub(crate) n: usize,
    pub pow_tau_g1: Vec<E::G1>,
    pub pow_tau_g2: Vec<E::G2>,
}

impl<E: Engine + Debug> ParamKzg<E>
where
    E::Fr: PrimeField,
{
    fn new(k: usize) -> Self {
        Self::setup(k)
    }

    // Generate the SRS
    pub fn setup(k: usize) -> Self {
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

        // obtain [s]1
        let pow_tau_g1: Vec<E::G1> = powers_of_tau
            .iter()
            .map(|tau_pow| E::G1Affine::generator() * tau_pow)
            .collect();

        // obtain [s]2
        let pow_tau_g2: Vec<E::G2> = powers_of_tau
            .iter()
            .map(|tau_pow| E::G2Affine::generator() * tau_pow)
            .collect();

        Self {
            k,
            n,
            pow_tau_g1,
            pow_tau_g2,
        }
    }

    // unify ti with commit_lagrange
    pub fn eval_at_tau_g1(&self, poly: &Polynomial<E::Fr>) -> E::G1 {
        let mut scalars = Vec::with_capacity(poly.len());
        scalars.extend(poly.coeffs().iter());
        let bases = &self.pow_tau_g1;
        let size = scalars.len();
        assert!(bases.len() >= size);
        small_multiexp(&scalars, &bases[0..size])
    }

    pub fn eval_at_tau_g2(&self, poly: &Polynomial<E::Fr>) -> E::G2 {
        let mut scalars = Vec::with_capacity(poly.len());
        scalars.extend(poly.coeffs().iter());
        let bases = &self.pow_tau_g2;
        let size = scalars.len();
        assert!(bases.len() >= size);
        small_multiexp(&scalars, &bases[0..size])
    }
}
