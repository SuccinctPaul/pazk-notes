// Description of the GKR protocol,
// when applied to a layered arithmetic circuit C of depth d and fan-in two on input x ∈ Fn.
// Throughout, ki denotes log2(Si) where Si is the number of gates at layer i of C.

use crate::arithmetic::layered_circuit::CircuitConfig;
use crate::gkr::prover::Prover;
use crate::gkr::verifier::Verifier;
use bls12_381::Scalar;

mod prover;
mod verifier;

pub struct GKR {
    prover: Prover,
    verifier: Verifier,
    d: usize,
}

impl GKR {
    // Init with layer-circuit
    pub fn init(config: CircuitConfig) -> Self {
        let mut prover = Prover::init(config);

        Self {
            prover,
            verifier: Verifier::default(),
            d: config.depth,
        }
    }

    // $f_{r_i}^{i}(b,c):=\widetilde{add_{i}}(r_{i},b,c)(\widetilde{W_{i+1}}(b)+\widetilde{W_{i+1}}(c))+\widetilde{mult_i}(r_i,b,c)(\widetilde{W_{i+1}}(b)\cdot \widetilde{W_{i+1}}(c))$
    fn run_protocol(&mut self, inputs: &Vec<Scalar>) {
        // 1. Prepare at the start of the protocol,
        //    The remainder of the protocol is devoted to confirming that $m0 =\widetilde{W^0}(r0)$ .
        self.prover.synthesize(inputs);
        //  1.1 P sends a function $D: {0,1}^k_0 → F$ claimed to equal W_0 (the function mapping output gate labels to output values).
        let D_poly = self.prover.output();
        //  1.2 V pick a challenge r_0( $r0∈Fk0$ ) and let $m_{0}=\widetilde{D}(r_0)$
        let r_0 = self.verifier.init(D_poly);

        // 2. start the d rounds gkr-gkr_sumcheck
        let mut r_i = r_0;
        for i in 0..self.d {

            // 1. P send $m_i = \sum_{b,c\in{0,1}^{i+1}} f_{r_i}(b,c)$
            // sumcheck with a proof

            // with sumcheck

            // 2. So that V may check this claim, by using the gkr_sumcheck for $f_{r_i}^{i}(b,c)$

            // gkr_sumcheck total 2*k_{i+1} round(var).

            // suncheck: round 1

            // loop {
            // 1.
            // }

            // gkr_sumcheck: round v
        }

        // 3. V checks d􏰌irectly that md = W􏰌d (rd ) using Lemma 3.8.
    }
}
