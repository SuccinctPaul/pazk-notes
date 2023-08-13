use crate::arithmetic::layered_circuit::CircuitConfig;
use crate::poly::MPolynomial;
use bls12_381::Scalar;

pub struct Prover {
    pub inputs: Vec<Scalar>,
    pub witness: Vec<MPolynomial>, // witness, start from 0 to d (include the input layer(layer_d). len = d+1
    outputs: Vec<Scalar>,
    pub ops: Vec<(MPolynomial, MPolynomial)>, // (add,mult) gate mpoly of each layer, start from 0 to d-1. len = d+1
    depth: usize,
    config: CircuitConfig,
}

impl Prover {
    // actual, this is the config.
    pub fn init(config: CircuitConfig) -> Self {
        let ops = config.ops_to_mpoly();
        Self {
            inputs: vec![],
            witness: vec![],
            outputs: vec![],
            ops,
            depth: config.depth,
            config,
        }
    }

    // synthesize with inputs to gen witness/advices.
    pub(crate) fn synthesize(&mut self, input: &Vec<Scalar>) {
        let (witness, outputs) = self.config.witness_to_poly(&input);
        self.witness = witness;
        self.outputs = outputs;
    }

    //  P sends a function $D: {0,1}^k_0 → F$ claimed to equal W_0 (the function mapping output gate labels to output values).
    pub fn D_poly(&self) -> MPolynomial {
        self.witness[0].clone()
    }

    pub fn outputs(&self) -> Vec<Scalar> {
        self.outputs.clone()
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
