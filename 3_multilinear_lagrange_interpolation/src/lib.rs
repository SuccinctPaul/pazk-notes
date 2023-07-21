
// Multivariable Polynomial Reference
// 1. https://github.com/int-e/twenty-first/blob/cbda032f2c7b3ba44aa5582a1057c6b8b32e4ab6/twenty-first/src/shared_math/mpolynomial.rs
// 2. https://github.com/benruijl/symbolica/blob/cfb96196dd6f8dcae813157f9f42ad9eaa64a4ab/src/poly/polynomial.rs
// 3. https://github.com/arkworks-rs/algebra/blob/master/poly/src/polynomial/multivariate/sparse.rs



/// # Impl-1
/// ## Struct
///
/// pub struct MSparsePolynomial<F: Field, T: Term> {
///     /// The number of variables the polynomial supports
///     #[derivative(PartialEq = "ignore")]
///     pub num_vars: usize,
///     /// List of each term along with its coefficient
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
///
///
/// ## Examples
/// f(x,y,z) = 17 + 2xy + 42z - 19x^6*y^3*z^12 is represented as:
///     {
///         [0,0,0] => 17,
///         [1,1,0] => 2,
///         [0,0,1] => 42,
///         [6,3,12] => -19,
///     }
///
///


pub struct MPolynomial<T: FiniteField> {
    // Multivariate polynomials are represented as hash maps with exponent vectors
    // as keys and coefficients as values. E.g.:
    // f(x,y,z) = 17 + 2xy + 42z - 19x^6*y^3*z^12 is represented as:
    // {
    //     [0,0,0] => 17,
    //     [1,1,0] => 2,
    //     [0,0,1] => 42,
    //     [6,3,12] => -19,
    // }
    pub variable_count: usize,
    // Notice that the exponent values may not exceed 0xFF = 255 = u8::MAX.
    pub coefficients: HashMap<Vec<u8>, T>,
}
