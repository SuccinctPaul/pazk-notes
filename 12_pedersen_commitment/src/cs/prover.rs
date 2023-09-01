// Commit is a
// randomized algorithm that takes as input the committing key ck and the message m to be committed and
// outputs the commitment c, as well as possibly extra “opening information” d that the committer may hold
// onto and only reveal during the verification procedure.

use ff::Field;
use pairing::Engine;
use rand_core::OsRng;
use std::ops::{Add, Mul};

pub struct Prover<E: Engine> {
    ck: E::Fr, // ck: commitment key
    g: E::G1,  // generator
    h: E::G1,  // generator
    m: Vec<E::Fr>,
}

impl<E: Engine> Prover<E> {
    pub fn init() {
        let mut rng = OsRng;
        let m = (0..3).map(|_| E::Fr::random(rng)).collect();
        Self { m }
    }

    // cm = Com_{g,h}(m, z)= g*m+h*z
    fn commit(&self, m: &E::Fr, z: &E::Fr) -> E::G1 {
        self.g * m + self.h * z
    }
    fn open(&self, e: E::Fr) -> Vec<E::Fr> {
        let mut rng = OsRng;
        let k = self.m.len(); // 3
                              // 1. commit.
        let mut cm = Vec::with_capacity(k);
        let mut r = Vec::with_capacity(k);
        for mi in self.m {
            let ri = E::Fr::random(rng);
            let ci = self.g * mi + self.h * ri;
            cm.push(ci);
            r.push(ri);
        }
        //     // return cm
        //     cm
        // }
        // fn open(&self, e: E::Fr) {
        let mut rng = OsRng;

        // compute challenge
        // alpha
        let b1 = E::Fr::random(rng);
        let b2 = E::Fr::random(rng);
        let alpha = self.commit(&b1, &b2);
        // beta
        let b3 = E::Fr::random(rng);
        let b4 = E::Fr::random(rng);
        let beta = self.commit(&b3, &b4);
        // gamma
        let b5 = E::Fr::random(rng);
        let gamma = self.commit(&b3, &b5);

        // compute z:
        //     z1 ← b1 + e · m1 ,
        //     z2 ← b2 + e · r1 ,
        //     z3 ← b3 + e · m2 ,
        //     z4 ← b4 + e · r2 ,
        //     z5 ← b5 + e · (r3 − r1 m2 ).
        let z1 = b1 + e * self.m[&0];
        let z2 = b2 + e * r[&0];
        let z3 = b3 + e * self.m[&1];
        let z4 = b4 + e * r[&1];
        let z5 = b5 + e * (r[&2] - r[&0] * self.m[&1]);
    }
}
