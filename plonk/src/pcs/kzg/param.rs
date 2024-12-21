use crate::math::msm::small_multiexp;
use crate::math::poly::Polynomial;
use ff::{Field, PrimeField};
use group::prime::PrimeCurveAffine;
use group::Group;
use pairing::Engine;
use rand_core::OsRng;
use std::fmt::Debug;

// The SRS
#[derive(Clone)]
pub struct ParamKzg<E: Engine> {
    pub(crate) k: usize,
    pub(crate) n: usize,
    pub pow_tau_g1: Vec<E::G1>,
    // pub pow_tau_g2: Vec<E::G2>,
    pub g1: E::G2,
    pub g2: E::G2,
    pub g2_s: E::G2,
    pub root_of_units: Vec<E::Fr>,
    // todo calcualte domain(H), poly z(x).
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
        let g1 = E::G1::generator();
        let pow_tau_g1: Vec<E::G1> = powers_of_tau.iter().map(|tau_pow| g1 * tau_pow).collect();

        // obtain [s]2
        let g2 = E::G2::generator();
        // let pow_tau_g2: Vec<E::G2> = powers_of_tau.iter().map(|tau_pow| g2 * tau_pow).collect();

        let g2_s = tau * g2;
        // todo add root of units

        Self {
            k,
            n,
            pow_tau_g1,
            // pow_tau_g2,
            g1,
            g2,
            g2_s,
            root_of_units: vec![],
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
