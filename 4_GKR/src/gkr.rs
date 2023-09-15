// Description of the GKR protocol,
// when applied to a layered math constraint_system C of depth d and fan-in two on input x ∈ Fn.
// Throughout, ki denotes log2(Si) where Si is the number of gates at layer i of C.

use crate::arithmetic::layered_circuit::CircuitConfig;
use crate::gkr::prover::Prover;
use crate::gkr::verifier::Verifier;
use crate::gkr_sumcheck::GkrSumCheck;
use bls12_381::Scalar;

mod prover;
mod verifier;

pub struct GKR {
    prover: Prover,
    verifier: Verifier,
    d: usize,
    input_var_num: usize,
}

impl GKR {
    // Init with layer-constraint_system
    pub fn init(config: CircuitConfig) -> Self {
        let d = config.depth;
        let input_var_num = config.input_var_num;
        let mut prover = Prover::init(config);

        Self {
            prover,
            verifier: Verifier::default(),
            d,
            input_var_num,
        }
    }

    // $f_{r_i}^{i}(b,c):=\widetilde{add_{i}}(r_{i},b,c)(\widetilde{W_{i+1}}(b)+\widetilde{W_{i+1}}(c))+\widetilde{mult_i}(r_i,b,c)(\widetilde{W_{i+1}}(b)\cdot \widetilde{W_{i+1}}(c))$
    fn run_protocol(&mut self, inputs: &Vec<Scalar>) {
        // 1. Prepare at the start of the protocol,
        //    The remainder of the protocol is devoted to confirming that $m0 =\widetilde{W^0}(r0)$ .
        //    <==> check $m0 = \sum_{b,c\in{0,1}^{i+1}} f_{r_i}(b,c)$
        self.prover.synthesize(inputs);
        //  1.1 P sends a function $D: {0,1}^k_0 → F$ claimed to equal W_0 (the function mapping output gate labels to output values).
        let D_poly = self.prover.D_poly();
        //  1.2 V pick a challenge r_0( $r0∈Fk0$ ) and let $m_{0}=\widetilde{D}(r_0)$
        let (r_0, m_0) = self.verifier.init(D_poly, inputs, self.input_var_num);

        // 2. start the d rounds gkr_sumcheck
        //    check $m_i = \sum_{b,c\in{0,1}^{i+1}} f_{r_i}(b,c)$
        let mut r_i = r_0;
        let mut m_i = m_0;
        for i in 0..self.d {
            // the ops and witness used in current layer.
            let (add_i, mult_i) = self.prover.ops.get(i).unwrap();
            let w_i_plus_1 = self.prover.witness.get(i + 1).unwrap();
            let g = (add_i.clone(), mult_i.clone(), w_i_plus_1.clone());

            let mut sumcheck = GkrSumCheck::init(g, r_i.clone(), m_i.clone());

            // we support the sumcheck prover is from GKR::prover! So does verifier.
            let (r_i_plus_1, m_i_plus_1) = sumcheck.run_protocol();

            r_i = r_i_plus_1;
            m_i = m_i_plus_1;
        }

        // 3. V checks directly that md = W_d(rd ) using Lemma 3.8.
        let r_d = r_i;
        let m_d = m_i;
        self.verifier.check(&r_d, m_d);

        // print the output
        let outputs = self.prover.outputs();
        println!("Output: {:?}", outputs);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::arithmetic::layered_circuit::Layer;
    use crate::arithmetic::layered_circuit::Ops::MUL;
    use crate::utils::convert_from_binary;
    use ff::PrimeField;

    // sample from Figure 4.12.
    fn simple_circuit() -> CircuitConfig {
        let input_var_num: usize = 2;

        let layer_1 = Layer {
            gates: vec![MUL(0, 0), MUL(1, 1), MUL(1, 2), MUL(3, 3)],
            var_num: 2,
        };

        let output_layer = Layer {
            gates: vec![MUL(0, 1), MUL(2, 3)],
            var_num: 1,
        };

        // Return layer
        CircuitConfig {
            layers: vec![output_layer, layer_1],
            input_var_num,
            depth: 3,
        }
    }

    #[test]
    fn test_GKR() {
        // todo-bugfix later.
        let inputs = vec![
            Scalar::one(),
            Scalar::from_u128(2),
            Scalar::one(),
            Scalar::from_u128(4),
        ];

        let config = simple_circuit();
        // evaluate the constraint_system with input
        let mut gkr = GKR::init(config);
        gkr.run_protocol(&inputs);
    }
}
