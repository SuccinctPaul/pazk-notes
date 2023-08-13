use crate::gkr_sumcheck::prover::Prover;
use crate::gkr_sumcheck::verifier::Verifier;
use crate::poly::{MPolynomial, Polynomial};
use bls12_381::Scalar;
use ff::PrimeField;
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
pub struct GkrSumCheck {
    // v_l: usize, // the constants_part var_num.  v_l + v_r = ki + 2*k_i_plus_1
    v_r: usize, // the variable_part var_num. equals to `v` in standard sumcheck.
    // r_i: Vec<usize>,
    // layer_i: usize, // the gkr layer index. [0,d)
    prover: Prover,
    verifier: Verifier,
}

//  (add, mult, w_i_plus_1)
type F_r_Poly = (MPolynomial, MPolynomial, MPolynomial);

impl GkrSumCheck {
    pub fn init(g: F_r_Poly, r_i: Vec<usize>, m_i: Scalar) -> Self {
        assert_eq!(g.0.var_num, g.1.var_num);
        let (v_l, v_r) = (r_i.len(), 2 * g.2.var_num);
        assert_eq!(g.0.var_num, v_l + v_r);

        let prover = Prover::new(g, r_i);
        let verifier = Verifier::new(v_r, m_i);

        Self {
            v_r,
            prover,
            verifier,
        }
    }

    pub fn run_protocol(&mut self) -> (Vec<usize>, Scalar) {
        // round 1
        let g1 = self.prover.round_1();
        self.verifier.round_1(g1);

        // round 2 - (v-1)
        for j in 2..self.v_r {
            let challenges = self.verifier.challenges();
            let g_j = self.prover.recursive_round_j(&challenges);
            self.verifier.recursive_round_j(j, g_j);
            drop(challenges);
        }

        // round v
        let challenges = self.verifier.challenges();
        let g_v = self.prover.round_v(&challenges);
        self.verifier.round_v(g_v);
        // drop(challenges);

        // finally check
        let challenges = self.verifier.challenges();
        let (add_value, mult_value, l_polys, p_poly) = self.prover.evaluate(&challenges);
        self.verifier.check((add_value, mult_value, &p_poly));

        // Prepare for next sumcheck:
        //  V chooses random t and sets r_{i+1} = l(t) and mi+1 = q(r_{i+1})=q(l(t)).
        let t = Verifier::gen_challenge();
        let r_1_plus_1 = l_polys
            .iter()
            .map(|l_i| l_i.iter().rev().fold(0, |acc, coeff| acc * t + *coeff))
            .collect::<Vec<_>>();
        let m_i_plus_1 = p_poly.evaluate(Scalar::from_u128(t as u128));

        (r_1_plus_1, m_i_plus_1)
    }
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
