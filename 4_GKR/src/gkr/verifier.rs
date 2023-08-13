use crate::poly::MPolynomial;
use bls12_381::Scalar;
use rand_core::{OsRng, RngCore};
use std::env::var;
use std::os::unix::raw::mode_t;

#[derive(Clone, Debug, Default)]
pub struct Verifier {
    m0: Scalar,
    w_d: MPolynomial,
}

impl Verifier {
    pub fn init(
        &mut self,
        output: MPolynomial,
        inputs: &Vec<Scalar>,
        input_var_num: usize,
    ) -> (Vec<usize>, Scalar) {
        //  V pick a challenge r_0( $r0∈Fk0$ ) and let $m_{0}=\widetilde{D}(r_0)$
        let r_0 = Self::gen_challenge(output.var_num);
        let m0 = output.evaluate(&r_0);

        // Encode the inputs as Mpoly.
        let w_d = MPolynomial::lagrange(input_var_num, inputs);

        self.m0 = m0;
        self.w_d = w_d;
        (r_0, m0)
    }

    // generate r1, r2, ..., rv,  $r_i ∈ F^{k_i}$
    pub fn gen_challenge(var_num: usize) -> Vec<usize> {
        let randoms = (0..var_num)
            .map(|_| OsRng.next_u32() % 1000)
            .collect::<Vec<_>>();
        randoms.iter().map(|r| *r as usize).collect::<Vec<_>>()
    }

    // V checks  m_d = W_d (r_d )
    pub fn check(&self, r_d: &Vec<usize>, target: Scalar) {
        let actual = self.w_d.evaluate(r_d);
        assert_eq!(target, actual, "GKR verifier: final check failed");

        println!("GKR: V accepted the output from P");
    }
}
