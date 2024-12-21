pub mod param;
pub mod prover;
pub mod verifier;

use bls12_381::Scalar;
use ff::Field;
use pairing::Engine;

pub struct KZGProof<E: Engine> {
    cm: E::G1,   // commit of p(x)
    eval: E::Fr, // eval for p(z)
    pi: E::G1,   // aka.π, commit of q(x), q = p(x)-p(z)/x-z
}

impl<E: Engine> KZGProof<E> {
    fn new(cm: E::G1, eval: E::Fr, pi: E::G1) -> Self {
        Self { cm, eval, pi }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::math::poly::Polynomial;
    use crate::pcs::kzg::param::ParamKzg;
    use crate::pcs::kzg::prover::Prover;
    use crate::pcs::kzg::verifier::Verifier;
    use bls12_381::Bls12;
    use ff::PrimeField;

    #[test]
    fn test_kzg_protocol() {
        let k = 4;
        let poly = Polynomial::random(3);

        // setup
        let param = ParamKzg::<Bls12>::setup(k);

        // prove
        let prover = Prover::init(param.clone());
        let proof = prover.prover(&poly);

        // verify
        let verifier = Verifier::init(param);
        verifier.verify(proof);
    }
}
