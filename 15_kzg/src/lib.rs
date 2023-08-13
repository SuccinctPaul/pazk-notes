//! this module contains an implementation of Kate-Zaverucha-Goldberg polynomial commitments

mod poly;

use crate::poly::*;
use bls12_381::Scalar as Fr;
use ff::Field;
use group::prime::PrimeCurveAffine;
use group::{Curve, Group};
use pairing::Engine;
use rand::{Rng, RngCore};
use std::fmt::Debug;
use std::iter;
use std::ops::Neg;

/// KZG polinomial commitments on Bls12-381. This structure contains the trusted setup.
pub struct Kzg<E: Engine> {
    pub pow_tau_g1: Vec<E::G1>,
    pub pow_tau_g2: Vec<E::G2>,
}

impl<E: Engine + Debug> Kzg<E> {
    fn eval_at_tau_g1(&self, poly: &Poly<E::Fr>) -> E::G1 {
        poly.0
            .iter()
            .enumerate()
            .fold(E::G1::identity(), |acc, (n, k)| {
                acc + self.pow_tau_g1[n] * k
            })
    }

    fn eval_at_tau_g2(&self, poly: &Poly<E::Fr>) -> E::G2 {
        poly.0
            .iter()
            .enumerate()
            .fold(E::G2::identity(), |acc, (n, k)| {
                acc + self.pow_tau_g2[n] * k
            })
    }

    fn z_poly_of(points: &[(E::Fr, E::Fr)]) -> Poly<E::Fr> {
        points.iter().fold(Poly::one(), |acc, (z, _y)| {
            &acc * &Poly::new(vec![z.neg(), E::Fr::ONE])
        })
    }

    /// Generate the trusted setup. Is expected that this function is called
    ///   in a safe environment what will be destroyed after its execution
    /// The `n` parameter is the maximum number of points that can be proved
    pub fn trusted_setup<R: RngCore>(n: usize, rng: R) -> Self {
        let tau = E::Fr::random(rng);

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

        let pow_tau_g2: Vec<E::G2> = powers_of_tau
            .iter()
            .map(|tau_pow| E::G2Affine::generator() * tau_pow)
            .collect();

        Self {
            pow_tau_g1,
            pow_tau_g2,
        }
    }

    /// Returns the maximum degree of the polinomial commitment
    pub fn max_degree(&self) -> usize {
        self.pow_tau_g1.len() - 1
    }

    /// Generate a polinomial and its commitment from a `set` of points
    #[allow(non_snake_case)]
    pub fn poly_commitment_from_set(&self, set: &[(E::Fr, E::Fr)]) -> (Poly<E::Fr>, E::G1) {
        let poly = Poly::lagrange(set);
        let commitment = self.eval_at_tau_g1(&poly);

        (poly, commitment)
    }

    /// Generates a proof that `points` exists in `set`
    #[allow(non_snake_case)]
    pub fn prove(&self, poly: &Poly<E::Fr>, points: &[(E::Fr, E::Fr)]) -> E::G1 {
        // compute a lagrange poliomial I that have all the points to proof that are in the set
        // compute the polinomial Z that has roots (y=0) in all x's of I,
        //   so this is I=(x-x0)(x-x1)...(x-xn)
        let I = Poly::lagrange(points);
        let Z = Self::z_poly_of(points);

        // now compute that Q = ( P - I(x) ) / Z(x)
        // also check that the division does not have remainder
        let mut poly = poly.clone();
        poly -= &I;
        let (Q, remainder) = poly / Z;
        assert!(remainder.is_zero());

        // the proof is evaluating the Q at tau in G1
        self.eval_at_tau_g1(&Q)
    }

    /// Verifies that `points` exists in `proof`
    #[allow(non_snake_case)]
    pub fn verify(&self, commitment: &E::G1, points: &[(E::Fr, E::Fr)], proof: &E::G1) -> bool {
        let I = Poly::lagrange(points);
        let Z = Self::z_poly_of(points);

        let e1 = E::pairing(&proof.to_affine(), &self.eval_at_tau_g2(&Z).to_affine());

        let e2 = E::pairing(
            &(*commitment - self.eval_at_tau_g1(&I)).to_affine(),
            &E::G2Affine::generator(),
        );
        e1 == e2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bls12_381::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_kzg() {
        // Create a trustd setup that allows maximum 4 points (degree+1)
        let kzg = Kzg::<Bls12>::trusted_setup(5, OsRng);

        // define the set of points (the "population"), and create a polinomial
        // for them, as well its polinomial commitment, see the polinomial commitment
        // like the "hash" of the polinomial
        let set = vec![
            (Fr::from(1), Fr::from(2)),
            (Fr::from(2), Fr::from(3)),
            (Fr::from(3), Fr::from(4)),
            (Fr::from(4), Fr::from(57)),
        ];
        let (p, c) = kzg.poly_commitment_from_set(&set);

        // generate a proof that (1,2) and (2,3) are in the set
        let proof01 = kzg.prove(&p, &vec![set[0].clone(), set[1].clone()]);

        // prove that (1,2) and (2,3) are in the set
        assert!(kzg.verify(&c, &vec![set[0].clone(), set[1].clone()], &proof01));
        // other proofs will fail since the proof only works for exactly (1,2) AND (2,3)
        assert!(!kzg.verify(&c, &vec![set[0].clone()], &proof01));
        assert!(!kzg.verify(&c, &vec![set[0].clone(), set[2].clone()], &proof01));

        // prove and verify that the whole set exists in the whole set
        let proof0123 = kzg.prove(&p, &set);
        assert!(kzg.verify(&c, &set, &proof0123));
    }
}
