use crate::poly::multivar_poly::MPolynomial;
use crate::utils::convert_to_binary;
use bls12_381::Scalar;

struct Prover {}

impl Prover {
    // sum all the evaluations on hypercube of a mpoly
    // obtain C1, which claimed equal H.
    fn proof(mpoly: MPolynomial) -> Scalar {
        let n = 1 << mpoly.var_num;
        (0..n)
            .map(|i| {
                let domain = convert_to_binary(&mpoly.var_num, i);
                mpoly.evaluate(&domain)
            })
            .sum()
    }

    fn first_round() {
        // how to convert a mpoly (r1, r2, .., X, x_i, ..., x_v)
    }

    fn recursive_round() {
        // send g_j = (r1, ..., r_j-1, X, x_j+1, ..., x_v)
    }

    fn last_round() {
        // send g_v = (r1, r2, ..., r_v-1, X_v)
    }
}
