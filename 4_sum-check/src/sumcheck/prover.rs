use crate::poly::multivar_poly::MPolynomial;
use crate::poly::univar_poly::Polynomial;
use crate::utils::convert_to_binary;
use bls12_381::Scalar;

pub struct Prover {
    // cache_g_j: Vec<Polynomial>
}

impl Prover {
    // sum all the evaluations on hypercube of a mpoly
    // obtain C1, which claimed equal H.
    pub fn proof(mpoly: &MPolynomial) -> Scalar {
        let n = 1 << mpoly.var_num;
        (0..n)
            .map(|i| {
                let domain = convert_to_binary(&mpoly.var_num, i);
                mpoly.evaluate(&domain)
            })
            .sum()
    }

    pub fn round_1() -> Polynomial {
        // how to convert a mpoly (r1, r2, .., X, x_i, ..., x_v)
        todo!()
    }

    // 1 < j < v, total v-2 rounds
    pub fn recursive_round_j(challengers: Vec<Scalar>) -> Polynomial {
        // send g_j = (r1, ..., r_j-1, X, x_j+1, ..., x_v)

        // challengers

        // xj+1, ..., xv

        todo!()
    }

    pub fn round_v() {
        // send g_v = (r1, r2, ..., r_v-1, X_v)
    }
}
