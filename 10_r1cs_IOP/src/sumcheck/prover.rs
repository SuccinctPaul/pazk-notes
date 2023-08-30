use crate::poly::Polynomial;
use ff::Field;

// todo
pub struct Prover<F: Field> {}

impl<F: Field> Prover<F> {
    /// Given a polynomial `f` to sum at domain `H`, return the actual sum, and
    /// polynomial `h` and `g` such that `p = h * Z_H + g * x + sum / |H|` where
    /// `Z_H` is the vanishing polynomial of `H`.
    ///
    /// # Returns
    /// * `h`: coefficient of polynomial `h` of degree `deg(f) - |H|`
    /// * `g`: coefficient of polynomial `g` of degree `|H| - 2`
    /// * `actual_sum`: actual sum of `f` over `H`

    // pub fn calculate_h_g_and_actual_sum(
    //     &self,
    //     f: &DensePolynomial<F>,
    // ) -> (DensePolynomial<F>, DensePolynomial<F>, F) {
    //     let vp = VanishingPoly::new(self.summation_domain);
    //     let (h, mut r) = f.divide_by_vp(vp);
    //     // we know r = x * g + sum / |H|. So g = (r - sum * |H|) / x
    //     let actual_sum = r.coeffs.remove(0) * F::from(self.summation_domain.size() as u64);
    //     let g = DensePolynomial::from_coefficients_vec(r.coeffs);
    //     (h, g, actual_sum)
    // }

    pub fn prove(&self, poly: Poly) {
        // 0. obtain subdomain H.

        // 1. confirm sum of p on H is 0

        // 2. obtain h,f with the equation: p(X) = h(X)*Z_H(X) + X*f(X)
    }

    pub fn evaluate() {}
}
