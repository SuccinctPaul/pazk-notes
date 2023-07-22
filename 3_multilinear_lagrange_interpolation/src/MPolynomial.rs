use bls12_381::Scalar;
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

// A multivariate polynomial g is multilinear if the degree of the polynomial in each variable is at most one.
// For example, the polynomial g(x1,x2) = x_1*x_2 +4x_1 +3x_2 is multilinear, but the polynomial
// h(x1,x2) = x2 + 4x1 + 3x2 is not.

pub struct MPolynomial {
    pub var_num: usize,
    // Notice that the exponent values may not exceed 0xFF = 255 = u8::MAX.
    pub coeffs: HashMap<Vec<u8>, Scalar>,
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
}

#[cfg(test)]
mod test {
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
}
