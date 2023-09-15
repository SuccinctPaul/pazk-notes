use crate::constraints::ConstraintSystem;
use crate::pcs::kzg::param::ParamKzg;
use crate::transcript::default::Keccak256Transcript;
use ff::PrimeField;
use pairing::Engine;

pub struct Prover<E: Engine> {
    param: ParamKzg<E>,          // setup param.
    cs: ConstraintSystem<E::Fr>, // constraint system
    winess: Vec<E::Fr>,          // private inputs
    constants: Vec<E::Fr>,       // public inputs
    roots: Vec<E::Fr>,
    ConcreteCircuit: Circuit<E::Fr>,
}

impl<E: Engine> Prover<E> {
    pub fn prove(&self) {
        let mut transcript = Keccak256Transcript::default();


        self.ConcreteCircuit: Circuit<Scheme::Scalar>;

    }

    // Round 1:
    // 1. Compute and commit wire witness polynomials.
    // 2. Compute public input polynomial.
    // Return the wire witness polynomials and their commitments,
    // also return the public input polynomial.
    fn round_1(&self) {
        self.cs.a;
    }

    // Round 2: Compute and commit the permutation grand product polynomial.
    // Return the grand product polynomial and its commitment.
    fn round_2(&self) {}
    // Round 3: Return the splitted quotient polynomials and their commitments.
    // Note that the first `num_wire_types`-1 splitted quotient polynomials
    // have degree `domain_size`+1.
    fn round_3(&self) {}
    // Round 4: Compute linearization polynomial and evaluate polynomials to be
    // opened.
    //
    // Compute the polynomial evaluations for TurboPlonk.
    // Return evaluations of the Plonk proof.
    fn round_4(&self) {}
    // Round 5
    // Compute (aggregated) polynomial opening proofs at point `zeta` and
    // `zeta * domain_generator`.
    fn round_5(&self) {}
}
