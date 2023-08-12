use crate::gkr_sumcheck::prover::Prover;
use crate::gkr_sumcheck::verifier::Verifier;
use crate::poly::{MPolynomial, Polynomial};
use bls12_381::Scalar;
use std::env::var;
use std::iter::Sum;

pub mod prover;
pub mod verifier;

// The key difference between standard sumcheck and gkr one is :
// 1. The summing evaluations is on different:
//      The standard sumcheck summing up all the evaluations of a multi-polynomial over all Boolean inputs
//
//      The variables of gkr one is split with constants_part and variables_part(eg: f(3,2,1,x1,x2)),
//      so it only sums up  the evaluations of a multi-polynomial over partial(the variable part) Boolean inputs
// 2. The evaluated poly:
//      The standard one evaluated only on one poly.
//      The gkr one evaluted on three polys-add, mult, W_i+1
pub struct GkrSumCheck<'a> {
    v_l: usize, // the constants_part var_num.  v_l + v_r = ki + 2*k_i_plus_1
    v_r: usize, // the variable_part var_num. equals to `v` in standard sumcheck.
    r_i: &'a Vec<usize>,
    // layer_i: usize, // the gkr layer index. [0,d)
    prover: Prover<'a>,
    verifier: Verifier,
}

//  (add, mult, w_i_plus_1)
type F_r_Poly<'a> = (&'a MPolynomial, &'a MPolynomial, &'a MPolynomial);

impl GkrSumCheck<'_> {
    pub fn init((v_l, v_r): (usize, usize), g: F_r_Poly, r_i: &Vec<usize>) -> Self {
        assert_eq!(g.0.var_num, g.1.var_num);
        assert_eq!(g.0.var_num, v_l + v_r);
        assert_eq!(r_i.len(), v_l);
        assert_eq!(g.2.var_num, v_r / 2); // w_i_plus_1

        let prover = Prover::new(g, r_i);
        let proof = prover.proof();
        let verifier = Verifier::new(v_r, proof);

        Self {
            v_l,
            v_r,
            r_i,
            prover,
            verifier,
        }
    }

    // pub fn run_protocol(&mut self) {
    //     // round 1
    //     let g1 = self.prover.round_1();
    //     self.verifier.round_1(g1);
    //
    //     // round 2 - (v-1)
    //     for j in 2..self.v_r {
    //         let challenges = self.verifier.challenges();
    //         let g_j = self.prover.recursive_round_j(&challenges);
    //         self.verifier.recursive_round_j(j, g_j);
    //         drop(challenges);
    //     }
    //
    //     // round v
    //     let challenges = self.verifier.challenges();
    //     let g_v = self.prover.round_v(&challenges);
    //     self.verifier.round_v(g_v);
    //     // drop(challenges);
    //
    //     // finally check
    //     let challenges = self.verifier.challenges();
    //     let target = self.prover.evaluate(&challenges);
    //     self.verifier.check(target);
    // }
}

// #[cfg(test)]
// mod test {
//     use crate::poly::multivar_poly::MPolynomial;
//     use crate::gkr_sumcheck::SumCheck;
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
//         let mut gkr_sumcheck = SumCheck::new(mpoly);
//
//         gkr_sumcheck.run_protocol();
//     }
// }
