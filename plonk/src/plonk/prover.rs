use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::constraints::ConstraintSystem;
use crate::pcs::kzg::param::ParamKzg;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use ff::PrimeField;
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
    // 1. Compute and commit wire witness polynomials.
    // 2. Compute public input polynomial.
    // Return the wire witness polynomials and their commitments,
    // also return the public input polynomial.
    fn round_1(&self, meta: &mut ConstraintSystem<E::Fr>, transcript: &mut T) {
        // self.cs.a;
    }

    // Round 2: Compute and commit the permutation grand product polynomial.
    // Return the grand product polynomial and its commitment.
    fn round_2(&self, meta: &mut ConstraintSystem<E::Fr>, transcript: &mut T) {}
    // Round 3: Return the splitted quotient polynomials and their commitments.
    // Note that the first `num_wire_types`-1 splitted quotient polynomials
    // have degree `domain_size`+1.
    fn round_3(&self, meta: &mut ConstraintSystem<E::Fr>, transcript: &mut T) {}
    // Round 4: Compute linearization polynomial and evaluate polynomials to be
    // opened.
    //
    // Compute the polynomial evaluations for TurboPlonk.
    // Return evaluations of the Plonk proof.
    fn round_4(&self, meta: &mut ConstraintSystem<E::Fr>, transcript: &mut T) {}
    // Round 5
    // Compute (aggregated) polynomial opening proofs at point `zeta` and
    // `zeta * domain_generator`.
    fn round_5(&self, meta: &mut ConstraintSystem<E::Fr>, transcript: &mut T) {}
}
