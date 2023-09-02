//! Math theory is the ecc Discrete Logarithm problem.
//!
//! The protocol prove :
//!     P holds (h, w) such that h = g*w in G, while V knows h and g.
//!
//! The the curves in Bls12 is additive group. So it's little different with the protocl in book(#184).

use pairing::Engine;

mod prover;
mod verifier;

pub struct Proof<E: Engine> {
    a: E::G1, // commitment
    e: E::Fr, // random scalar, a challenger from verifier
    z: E::Fr, // proof: z = we + r
}

#[cfg(test)]
mod test {
    use crate::sigma::prover::Prover;
    use crate::sigma::verifier::Verifier;
    use bls12_381::Bls12;

    #[test]
    fn test_sigma_protocol() {
        let (prover, h) = Prover::<Bls12>::init();
        let verifier = Verifier::init(h);

        let proof = prover.prove();
        verifier.verify(&proof);
    }
}
