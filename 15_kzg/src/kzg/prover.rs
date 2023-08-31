use crate::kzg::param::ParamKzg;
use crate::kzg::KZGProof;
use crate::poly::Polynomial;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use ff::{BitViewSized, Field, PrimeField};
use pairing::Engine;
use std::ops::{MulAssign, SubAssign};

pub struct Prover<E: Engine> {
    param: ParamKzg<E>,
}

impl<E: Engine> Prover<E> {
    pub fn init(param: ParamKzg<E>) -> Self {
        Self { param }
    }

    pub fn prover(&self, poly: &Polynomial<E::Fr>) -> KZGProof<E> {
        // 1. commit
        let cm = self.commit(poly);

        // 2. challenge z.
        let mut transcript_1 = Keccak256Transcript::<E::Fr>::default();
        let z = transcript_1.challenge();
        // 3. eval z.
        let eval = poly.evaluate(z.clone());

        // 4. open
        let pi = self.open(poly, &z);

        KZGProof::new(cm, eval, pi)
    }

    // return the commit of p
    fn commit(&self, poly: &Polynomial<E::Fr>) -> E::G1 {
        self.param.eval_at_tau_g1(poly)
    }

    // return the commit of q, aka.pi, the proof.
    fn open(&self, poly: &Polynomial<E::Fr>, z: &E::Fr) -> E::G1 {
        // q = ( p(x) - p(z) ) / x-z
        let q_coeff = Self::kate_division(&poly.coeffs(), z.clone());
        let q = Polynomial::from_coeffs(q_coeff);
        // the proof is evaluating the Q at tau in G1
        self.commit(&q)
    }

    // Divides polynomial `a` in `X` by `X - b` with no remainder.
    //      q(x) = f(x)-f(z)/x-z
    fn kate_division(a: &Vec<E::Fr>, z: E::Fr) -> Vec<E::Fr> {
        let b = -z;
        let a = a.into_iter();

        let mut q = vec![E::Fr::ZERO; a.len() - 1];

        let mut tmp: E::Fr = E::Fr::ZERO;
        for (q, r) in q.iter_mut().rev().zip(a.rev()) {
            let mut lead_coeff = *r;
            lead_coeff.sub_assign(&tmp);
            *q = lead_coeff;
            tmp = lead_coeff;
            tmp.mul_assign(&b);
        }
        q
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::kzg::param::ParamKzg;
    use crate::kzg::prover::Prover;
    use crate::kzg::verifier::Verifier;
    use crate::poly::Polynomial;
    use bls12_381::{Bls12, Scalar};
    use ff::{Field, PrimeField};

    #[test]
    fn test_div() {
        // division: -2+x+x^2 = (x-1)(x+2)
        let division = vec![Scalar::from_u128(2).neg(), Scalar::ONE, Scalar::ONE];

        // dividor: 2+x
        // dividor: -1+x
        let coeffs = vec![Scalar::ONE.neg(), Scalar::ONE];
        let dividor = Polynomial::from_coeffs(coeffs);

        // target:
        //      quotient poly: 2+x
        //      remainder poly: 0
        let target_qoutient = vec![Scalar::from_u128(2), Scalar::ONE];

        // q(x) = f(x)-f(z)/x-z
        let z = Scalar::ONE;
        let actual_qoutient = Prover::<Bls12>::kate_division(&division, z);

        assert_eq!(actual_qoutient, target_qoutient);
    }
}
