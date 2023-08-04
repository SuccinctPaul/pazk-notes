use bls12_381::Scalar;

pub struct Prover {}

impl Prover {
    // At the start of the protocol, P sends a function D: {0,1}k0 → F claimed to equal W0
    // (the function mapping output gate labels to output values).
    pub fn proof(&self) -> () {}

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
