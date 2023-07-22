use crate::utils::{convert_from_binary, convert_to_binary};
use bls12_381::Scalar;
use ff::Field;
/// Generally speaking, there are two type of implements:
/// # Impl-1
/// ## Struct
///
/// pub struct MSparsePolynomial<F: Field, T: Term> {
///     /// The number of variables the polynomial supports
///     #[derivative(PartialEq = "ignore")]
///     pub num_vars: usize,
///     /// List of each term along with its coefficient
///     /// term: (coeff, T)
///     /// T: [(var_index, exp)]
///     pub terms: Vec<(F, T)>,
/// }
///
/// ## Examples
/// `2*x_0^3 + x_0*x_2 + x_1*x_2 + 5`:
/// ```
/// let poly = MSparsePolynomial::from_coefficients_vec(
///     3,
///     vec![
///         (Fq::from(2), SparseTerm::new(vec![(0, 3)])),
///         (Fq::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
///         (Fq::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
///         (Fq::from(5), SparseTerm::new(vec![])),
///     ],
/// );
/// ```
/// # Impl-2
/// Multivariate polynomials are represented as hash maps with exponent vectors
/// as keys and coefficients as values. E.g.:
/// ## struct
/// pub struct MPolynomial<T: FiniteField> {
///
///     pub variable_count: usize,
///     // Notice that the exponent values may not exceed 0xFF = 255 = u8::MAX.
///     pub coefficients: HashMap<Vec<u8>, T>,
/// }
///
/// ## Examples
/// f(x,y,z) = 17 + 2xy + 42z - 19x^6*y^3*z^12 is represented as:
/// var_num = 3,
///     {
///         [0,0,0] => 17,
///         [1,1,0] => 2,
///         [0,0,1] => 42,
///         [6,3,12] => -19,
///     }
use std::collections::HashMap;
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
    // w: {0,1}^v
    // F(x_1,...,x_v) = ∑f(w)·X_w(x_1,...,x_v),
    // X_w(x1,...,xv) := ∏(xiwi +(1−xi)(1−wi)).
    fn lagrange(v: usize, domain: Vec<Scalar>, evals: Vec<Scalar>) {
        assert_eq!(domain.len().pow(v as u32), evals.len());

        // compute f_i = f_x * X_w

        for (i, f_w) in evals.iter().enumerate() {

            // decode i into 二进制

            // compute X_w
        }

        // compute F = sum(f_i)
    }

    fn evaluate(&self, domain: &Vec<usize>) -> Scalar {
        let var_num = &self.var_num;

        assert_eq!(domain.len(), *var_num, "Domain is less than it");

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
                let exps = convert_to_binary(var_num, index);

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

#[cfg(test)]
mod test {
    use crate::utils::*;
    use crate::MPolynomial::MPolynomial;
    use bls12_381::Scalar;
    use ff::PrimeField;

    #[test]
    fn test_langrange() {
        // g(x1,x2) = x1*x2 + 4*x1 + 3*x2
        // var_num = 2
        // domain = {0, 1}^2, it's on hypercube.
        //      As for each var x in [0,1], so (x1, x2) in [(0,0), (0,1), (1,0), (1,1)]
        // evals = [0, 3, 4, 8]
        //      As g(0,0)=0, g(0,1)=3, g(1,0)=4, g(1,1)=8.

        let domain = [[0, 0], [0, 1], [1, 0], [1, 1]];
        let evals = [0, 3, 4, 8];
    }

    #[test]
    fn test_evaluate() {
        // let g(x1, x2, x3) = 5 + 2*x3 + 3*x2 +  x1 * x2 * x3
        // term0: exp: (0,0,0) = 5
        // term1: exp: (0,0,1) = 2*x3
        // term2: exp: (0,1,0) = 3*x2
        // term3-6: exp: (0,1,0) = 0.
        // term7: exp: (1,1,1) = x1 * x2 * x3

        let var_num = 3;

        let poly = MPolynomial {
            var_num,
            coeffs: vec![
                Scalar::from_u128(5),
                Scalar::from_u128(2),
                Scalar::from_u128(3),
                Scalar::zero(),
                Scalar::zero(),
                Scalar::zero(),
                Scalar::zero(),
                Scalar::one(),
            ],
        };

        // domain: (0,1,1)
        let domain = convert_to_binary(&var_num, 3);
        let target = Scalar::from_u128(10);

        let actual = poly.evaluate(&domain);
        assert_eq!(target, actual);
    }

    #[test]
    fn test_domain() {
        // g(x1,...,xv) = x1*x2 + 4*x1 + 3*x2 + ... + xv
        // var_num: v
        // domain: [[0; v], ..., [0, 1,..., 0], ..., [1; v]]
        let var_num: Vec<usize> = vec![2, 3, 4];

        for num in var_num.iter() {
            let max_num: usize = 1 << num;
            let domain = (0..max_num)
                .into_iter()
                .map(|n| convert_to_binary(&num, n))
                .collect::<Vec<_>>();
            assert_eq!(domain.len(), max_num);
            println!("num: {:?}", num);
            println!("domain: size:{:?},  {:?}", domain.len(), domain);
        }
    }
}
