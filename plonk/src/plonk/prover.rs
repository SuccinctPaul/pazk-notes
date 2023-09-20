use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::constraints::ConstraintSystem;
use crate::math::poly::Polynomial;
use crate::pcs::kzg::param::ParamKzg;
use crate::plonk::PlonkProof;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use ff::{Field, PrimeField};
use pairing::Engine;

pub struct Prover<E, C, T>
where
    E: Engine,
    C: Circuit<E::Fr>,
    T: Transcript<F>,
{
    param: ParamKzg<E>, // setup param.
    winess: Vec<E::Fr>, // private inputs
    roots: Vec<E::Fr>,
    circuit: C,
    domain: Vec<E::Fr>,
    Z_H: Polynomial<E::Fr>,
}

impl<E, C, T> Prover<E, C, T>
where
    E: Engine,
    C: Circuit<E::Fr>,
    T: Transcript<F>,
{
    pub fn prove(&self) {
        let n = 1 << self.cs.k;

        // initialize constraint system
        let circuit_config = C::<E::Fr>::configure(self.cs.k);
        let mut meta = ConstraintSystem::<E::Fr>::new(self.param.k, &circuit_config); // constraint system

        self.circuit.synthesize(&mut meta);

        let mut transcript = T::new();
    }

    // Round 1:
    // 1. Compute wire witness polynomials.
    // 2. commit a(x),b(x),c(x) to proof
    fn round_1(
        &self,
        meta: &mut ConstraintSystem<E::Fr>,
        transcript: &mut T,
        proof: &mut PlonkProof<E::Fr>,
    ) -> (Polynomial<E::Fr>, Polynomial<E::Fr>, Polynomial<E::Fr>) {
        // Generate random blinding scalars
        let (b1, b2, b3, b4, b5, b6) = (
            transcript.challenge(),
            transcript.challenge(),
            transcript.challenge(),
            transcript.challenge(),
            transcript.challenge(),
            transcript.challenge(),
        );

        // 1. Compute wire polynomials a(X), b(X), c(X)
        let poly_a = Polynomial::from_coeffs(vec![b2, b1]) * self.Z_H
            + Polynomial::lagrange_interpolate(self.domain.clone(), meta.a.clone());
        let poly_b = Polynomial::from_coeffs(vec![b4, b3]) * self.Z_H
            + Polynomial::lagrange_interpolate(self.domain.clone(), meta.b.clone());
        let poly_c = Polynomial::from_coeffs(vec![b6, b5]) * self.Z_H
            + Polynomial::lagrange_interpolate(self.domain.clone(), meta.c.clone());

        // 2. Compute public input polynomial.
        proof.cm_a = self.param.eval_at_tau_g1(&poly_a);
        proof.cm_b = self.param.eval_at_tau_g1(&poly_b);
        proof.cm_c = self.param.eval_at_tau_g1(&poly_c);

        (poly_a, poly_b, poly_c)
    }

    // Round 2:
    // 1. Compute ermutation grand product polynomial z(X)
    // 2. commit z(X) to proof
    fn round_2(
        &self,
        meta: &mut ConstraintSystem<E::Fr>,
        transcript: &mut T,
        proof: &mut PlonkProof<E::Fr>,
    ) -> Polynomial<E::Fr> {
        // Generate random blinding scalars
        let (b7, b8, b9) = (
            transcript.challenge(),
            transcript.challenge(),
            transcript.challenge(),
        );
        // Compute permutation challenges
        let (beta, gamma) = (transcript.challenge(), transcript.challenge());

        // 1. Compute permutation polynomial z(X)
        // todo more
        let poly_z = Polynomial::from_coeffs(vec![b9, b8, b7]);

        // 2. Compute public input polynomial.
        proof.cm_z = self.param.eval_at_tau_g1(&poly_z);

        poly_z
    }

    // Round 3:
    // 1. Compute and quotient polynomial t(X)
    // 2. spilit quotient polynomial into `t_lo, t_mid, t_hi`.
    // 2. commit `t_lo, t_mid, t_hi` to proof
    //
    // TODO: Does the qoutient poly make sure the constraints ?
    fn round_3(&self, meta: &mut ConstraintSystem<E::Fr>, transcript: &mut T) {
        // Compute quotient challenges
        let (alpha) = (transcript.challenge());
    }
    // Round 4:
    // 1. Evaluate polynomials
    fn round_4(
        &self,
        (poly_a, poly_b, poly_c, poly_s_sigma_1, poly_s_sigma_2, poly_z): (
            &Polynomial<E::Fr>,
            &Polynomial<E::Fr>,
            &Polynomial<E::Fr>,
            &Polynomial<E::Fr>,
            &Polynomial<E::Fr>,
            &Polynomial<E::Fr>,
            &Polynomial<E::Fr>,
        ),
        proof: &mut PlonkProof<E::Fr>,
    ) {
        // Compute evaluate challenges
        let sigma = transcript.challenge();

        proof.a_z = poly_a.evaluate(sigmac.clone());
        proof.b_z = poly_b.evaluate(sigmac.clone());
        proof.c_z = poly_c.evaluate(sigmac.clone());
        proof.s_sigma_1_z = poly_s_sigma_1.evaluate(sigmac.clone());
        proof.s_sigma_2_z = poly_s_sigma_2.evaluate(sigmac.clone());
        proof.z_omega_z = poly_z.evaluate(sigmac.clone());
    }
    // Round 5
    // Compute (aggregated) polynomial opening proofs at point `zeta` and
    // `zeta * domain_generator`.
    fn round_5(&self, meta: &mut ConstraintSystem<E::Fr>, transcript: &mut T) {}
}
