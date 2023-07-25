use crate::utils::convert_to_binary;
use bls12_381::Scalar;
use std::ops::AddAssign;

// A multivariate polynomial g is multilinear if the degree of the polynomial in each variable is at most one.
// For example, the polynomial g(x1,x2) = x_1*x_2 +4x_1 +3x_2 is multilinear, but the polynomial
// h(x1,x2) = x2 + 4x1 + 3x2 is not.
pub struct MPolynomial {
    pub var_num: usize,
    // The index (with binary form) is the exponent values.
    pub coeffs: Vec<Scalar>,
}

impl MPolynomial {
    pub fn evaluate(&self, domain: &Vec<usize>) -> Scalar {
        assert_eq!(domain.len(), self.var_num, "Domain is less than var_num");

        let mut sum_of_term = Scalar::zero();

        // compute each term_i: coeff * product_x
        for (index, coeff) in self.coeffs.iter().enumerate() {
            // if the coeff is 0, then skip it.
            if coeff.eq(&Scalar::zero()) {
                continue;
            }

            // if index is 0, then term = coeff.
            if index == 0 {
                sum_of_term += coeff;
            } else {
                // x_0^exps[0] * x_1^exps[1] * x_2^exps[2]+ ...
                let exps = convert_to_binary(&self.var_num, index);

                // compute product of x , eg: product_x = (x_1^exp1) * (x_2^exp2)
                let mut product = 1;
                for (x_i, exp_i) in domain.into_iter().zip(exps) {
                    let x = x_i.clone();

                    // Note, as the definition, the exp is in [0, 1]
                    // if exp != 0 && x != 0 {
                    product *= x.pow(exp_i as u32);

                    // once product, the computation of product is over. As zero multiple anything is zero.
                    if 0 == product {
                        break;
                    }
                }

                match product {
                    0 => continue,
                    1 => sum_of_term += coeff,
                    _ => {
                        let term_i = coeff.mul(&Scalar::from(product as u64));
                        sum_of_term.add_assign(term_i);
                    }
                }
            }
        }
        sum_of_term
    }
}
