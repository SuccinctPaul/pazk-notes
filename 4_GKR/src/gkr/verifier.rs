use crate::poly::MPolynomial;
use bls12_381::Scalar;
use rand_core::{OsRng, RngCore};
use std::env::var;
use std::os::unix::raw::mode_t;

#[derive(Clone, Debug, Default)]
pub struct Verifier {
    output: MPolynomial,
    m0: Scalar,
}

impl Verifier {
    pub fn init(&mut self, output: MPolynomial) -> Vec<usize> {
        //  V pick a challenge r_0( $r0∈Fk0$ ) and let $m_{0}=\widetilde{D}(r_0)$
        let r_0 = Self::gen_challenge(output.var_num);
        let m0 = output.evaluate(&r_0);

        self.output = output;
        self.m0 = m0;
        r_0
    }

    // generate r1, r2, ..., rv,  $r_i ∈ F^{k_i}$
    pub fn gen_challenge(var_num: usize) -> Vec<usize> {
        let randoms = (0..var_num)
            .map(|_| OsRng.next_u32() % 1000)
            .collect::<Vec<_>>();
        randoms.iter().map(|r| *r as usize).collect::<Vec<_>>()
    }

    // picks a random r0∈Fk0 and lets m0←D ̃(r0).
    pub fn round_i() {}

    // V checks  m_d = W_d (r_d )
    pub fn check() {}
}
