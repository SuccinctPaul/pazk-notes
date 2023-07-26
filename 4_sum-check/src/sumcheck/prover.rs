use crate::poly::multivar_poly::MPolynomial;
use crate::poly::univar_poly::Polynomial;
use crate::utils::convert_to_binary;
use bls12_381::Scalar;

pub struct Prover {
    g: MPolynomial,
}

impl Prover {
    pub fn new(mpoly: MPolynomial) -> Self {
        Self { g: mpoly }
    }

    // sum all the evaluations on hypercube of a mpoly
    // obtain C1, which claimed equal H.
    pub fn proof(&self) -> Scalar {
        let n = 1 << self.g.var_num;
        (0..n)
            .map(|i| {
                let domain = convert_to_binary(&self.g.var_num, i);
                self.g.evaluate(&domain)
            })
            .sum()
    }

    // Return g1(X) = sum g(X, x_2, ..., x_v)
    pub fn round_1(&self) -> Polynomial {
        self.g.partial_evaluate(&vec![])
    }

    // 1 < j < v, total v-2 rounds
    // Return g_j = (r1, ..., r_j-1, X, x_j+1, ..., x_v)
    pub fn recursive_round_j(&self, challenges: &Vec<usize>) -> Polynomial {
        self.g.partial_evaluate(challenges)
    }

    // Return g_v = (r1, r2, ..., r_v-1, X_v)
    pub fn round_v(&self, challenges: &Vec<usize>) -> Polynomial {
        self.g.partial_evaluate(challenges)
    }

    pub fn evaluate(&self, challenges: &Vec<usize>) -> Scalar {
        self.g.evaluate(challenges)
    }
}
