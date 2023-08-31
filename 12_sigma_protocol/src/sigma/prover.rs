use crate::sigma::Proof;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::Transcript;
use ff::Field;
use group::{Curve, Group};
use pairing::Engine;
use rand_core::OsRng;

#[derive(Default)]
pub struct Prover<E: Engine> {
    w: E::Fr,
    h: E::G1,
}

impl<E: Engine> Prover<E> {
    // init with the statement:
    //      prover know the w satisfies: h = g*w
    pub fn init() -> (Self, E::G1) {
        let w = E::Fr::random(OsRng);
        let h = Self::statement(Some(w.clone()));
        (Self { w, h }, h.clone())
    }

    fn statement(witness: Option<E::Fr>) -> E::G1 {
        let w = match witness {
            Some(w) => w,
            None => E::Fr::random(OsRng),
        };
        let g = E::G1::generator();
        // h = g*w
        let h = g * w;
        h
    }

    pub fn prove(&self) -> Proof<E> {
        let g = E::G1::generator();

        // commit phase
        let mut transcript_1 = Keccak256Transcript::<E::Fr>::default();
        let r = transcript_1.challenge();

        // a = g*r, aka commit
        let a = g * r;

        // open phase
        let e = transcript_1.challenge();
        // z = we + r
        let z = r + self.w * e;

        Proof { a, e, z }
    }
}
