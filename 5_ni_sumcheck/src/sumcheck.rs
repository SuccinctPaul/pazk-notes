use crate::poly::multivar_poly::MPolynomial;
use crate::poly::univar_poly::Polynomial;
use crate::sumcheck::prover::Prover;
use crate::sumcheck::verifier::Verifier;
use bls12_381::Scalar;
use std::env::var;
use std::iter::Sum;
use crate::transcript::default::Keccak256Transcript;
use crate::transcript::{poly_to_bytes, Transcript};

mod prover;
mod verifier;

// todo convert to nizk
struct SumCheck {
    v: usize,
    prover: Prover,
    verifier: Verifier,
}


impl SumCheck {
    fn new(g: MPolynomial) -> Self {
        let var_num = g.var_num;

        let prover = Prover::new(g);
        let proof = prover.proof();
        let verifier = Verifier::new(var_num, proof);

        Self {
            v: var_num,
            prover,
            verifier,
        }
    }



}

#[derive(Default)]
struct Proofs {
    target: Scalar,
    g_i_vec: Vec<Polynomial>,
}


// #[cfg(test)]
// mod test {
//     use crate::poly::multivar_poly::MPolynomial;
//     use crate::sumcheck::SumCheck;
//     use bls12_381::Scalar;
//     use ff::PrimeField;
//
//     fn gen_mpoly() -> MPolynomial {
//         // let g(x1, x2, x3) = 9 + 2*x3 + 3*x2 + 2 * x1 * x2 + 4* x1 * x2 * x3
//         // term0: exp: (0,0,0) = 9
//         // term1: exp: (0,0,1) = 2*x3
//         // term2: exp: (0,1,0) = 3*x2
//         // term3-6: exp: (0,1,0) = 0.
//         // term6: exp: (1,1,0) = 2 * x1 * x2
//         // term7: exp: (1,1,1) = 4 * x1 * x2 * x3
//
//         let var_num = 3;
//
//         MPolynomial {
//             var_num,
//             coeffs: vec![
//                 Scalar::from_u128(9),
//                 Scalar::from_u128(2),
//                 Scalar::from_u128(3),
//                 Scalar::zero(),
//                 Scalar::zero(),
//                 Scalar::zero(),
//                 Scalar::from_u128(2),
//                 Scalar::from_u128(4),
//             ],
//         }
//     }
//
//     #[test]
//     fn test_sumcheck() {
//         let mpoly = gen_mpoly();
//
//         let mut sumcheck = SumCheck::new(mpoly);
//
//         // todo! meet error
//         sumcheck.run_protocol();
//     }
// }
