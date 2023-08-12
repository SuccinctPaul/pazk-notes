use crate::arithmetic::layered_circuit::CircuitConfig;
use crate::poly::MPolynomial;
use bls12_381::Scalar;

pub struct Prover {
    witness: Vec<MPolynomial>, // witness, start from 0 to d (include the input layer(layer_d)
    ops: Vec<(MPolynomial, MPolynomial)>, // (add,mult) gate mpoly of each layer, start from 0 to d-1
    depth: usize,
    config: CircuitConfig,
}

impl Prover {
    // actual, this is the config.
    pub fn init(config: CircuitConfig) -> Self {
        let ops = config.ops_to_mpoly();
        Self {
            witness: vec![],
            ops,
            depth: config.depth,
            config,
        }
    }

    // synthesize with inputs to gen witness/advices.
    pub(crate) fn synthesize(&mut self, input: &Vec<Scalar>) {
        let witness = self.config.witness_to_poly(&input); //todo opti
        self.witness = witness;
    }

    //  P sends a function $D: {0,1}^k_0 → F$ claimed to equal W_0 (the function mapping output gate labels to output values).
    pub fn output(&self) -> MPolynomial {
        self.witness[0].clone()
    }

    // GKR-gkr_sumcheck-Round 1
    //  P send $m_i = \sum_{b,c\in{0,1}^{i+1}} f_{r_i}(b,c)$
    // pub fn claim_mi(&self, layer_i: usize, r_i: &Vec<usize>) -> Scalar {}

    // At the start of the protocol, P sends a function D: {0,1}^{k_0} → F claimed to equal W0
    // (the function mapping output gate labels to output values).
    // each layer has its claims, which means it's witness of the layer.
    pub fn claims(&self, depth: usize) -> Vec<Scalar> {
        // output, the layer 0.
        todo!()
    }

    pub fn round_1(&self) -> () {
        todo!()
    }

    // total d round: i=0,1,...,d−1
    pub fn round_i(&self) -> () {
        todo!()
    }

    // Define the (2ki+1)-variate polynomial
    pub fn gen_f_ri() {}
}
