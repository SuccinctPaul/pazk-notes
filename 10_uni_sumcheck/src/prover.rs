pub struct Prover {}

impl Prover {
    // prove that a specified univariate polynomial p of degree D sums to 0 over a multiplicative subgroup H with |H| = n,
    pub fn prove(&self, poly: Poly) {
        // 0. obtain subdomain H.

        // 1. confirm sum of p on H is 0

        // 2. obtain h,f with the equation: p(X) = h(X)*Z_H(X) + X*f(X)
    }

    pub fn evaluate() {}
}
