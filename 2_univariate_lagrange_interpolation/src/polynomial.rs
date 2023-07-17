use bls12_381::Scalar;
use ff::BatchInvert;

// p(x) = = a_0 + a_1 * X + ... + a_n * X^(n-1)
//
// coeffs: [a_0, a_1, ..., a_n]
// basis: X^[n-1]
pub struct Polynomial {
    coeffs: Vec<Scalar>,
}

impl Polynomial {
    // p(x) = a_0 + a_1 * X + ... + a_n * X^(n-1)
    // coeffs: [a_0, a_1, ..., a_n]
    // basis: X^[n-1]
    //
    // domain: {0, 1, . . . , n − 1}
    // evals: {p(0), p(1), ..., p(n-1)
    fn encode(vector: Vec<Scalar>) -> Self {
        Self { coeffs: vector }
    }

    // p(x)=∑y_j⋅L_j(X), where
    // y_j: [a_0, a_1, ..., a_n].
    // basis: L_j(X)=∏(X−x_k)/(x_j−x_k)
    //
    // domain: x, most case is that{0, 1, . . . , n − 1}
    // evals: [a_0, a_1, ..., a_n]
    //
    // we can use encode points as (domain, eval) to polynomials
    // the poly
    pub fn lagrange_interpolate(domains: Vec<Scalar>, evals: Vec<Scalar>) -> Self {
        assert_eq!(domains.len(), evals.len());

        if evals.len() == 1 {
            // Constant polynomial
            Self {
                coeffs: vec![evals[0]],
            }
        } else {
            let domain_len = domains.len();

            // 1. divisors = vec(x_j - x_k). prepare for L_j(X)=∏(X−x_k)/(x_j−x_k)
            let mut divisors = Vec::with_capacity(domain_len);
            for (j, x_j) in domains.iter().enumerate() {
                let mut divisor = Vec::with_capacity(domain_len - 1);
                // obtain domain for x_k
                for x_k in domains
                    .iter()
                    .enumerate()
                    .filter(|&(k, _)| k != j)
                    .map(|(_, x)| x)
                {
                    divisor.push(*x_j - x_k);
                }
                divisors.push(divisor);
            }
            // Inverse (x_j - x_k)^(-1) for each j != k to compute L_j(X)=∏(X−x_k)/(x_j−x_k)
            divisors
                .iter_mut()
                .flat_map(|v| v.iter_mut())
                .batch_invert();

            // p(x)=∑y_j⋅L_j(X) in coefficients
            let mut final_poly = vec![Scalar::zero(); domain_len];

            for (j, (divisor_j, y_j)) in divisors.into_iter().zip(evals.iter()).enumerate() {
                let mut tmp: Vec<F> = Vec::with_capacity(domain_len);
                let mut product = Vec::with_capacity(domain_len - 1);
                tmp.push(Scalar::one());

                // obtain domain for x_k
                //
                for (x_k, divisor) in domains
                    .iter()
                    .enumerate()
                    .filter(|&(k, _)| k != j)
                    .map(|(_, x)| x)
                    .zip(divisor_j.into_iter())
                {
                    product.resize(tmp.len() + 1, Scalar::zero());

                    // loop (domain_len + 1) round
                    for ((a, b), product) in tmp
                        .iter()
                        .chain(std::iter::once(&Scalar::zero()))
                        .zip(std::iter::once(&Scalar::zero()).chain(tmp.iter()))
                        .zip(product.iter_mut())
                    {
                        *product = *a * (-divisor * x_k) + *b * divisor;
                    }
                    std::mem::swap(&mut tmp, &mut product);
                }

                assert_eq!(tmp.len(), domain_len);
                assert_eq!(product.len(), domain_len - 1);

                // p(x)=∑y_j⋅L_j(X)
                for (final_coeff, interpolation_coeff) in final_poly.iter_mut().zip(tmp.into_iter())
                {
                    *final_coeff += interpolation_coeff * y_j;
                }
            }
            Self { coeffs: final_poly }
        }
    }

    /// This evaluates a polynomial (in coefficient form) at `x`.
    pub fn evaluate(poly: &Polynomial, x: Scalar) -> Scalar {
        let coeffs = poly.coeffs.clone();

        fn eval(poly: &Vec<Scalar>, point: F) -> F {
            poly.iter()
                .fold(Scalar::zero(), |acc, coeff| acc * point + coeff)
        }
        let n = poly.coeffs.len();
        let num_threads = multicore::current_num_threads();
        if n * 2 < num_threads {
            eval(&coeffs, x)
        } else {
            let chunk_size = (n + num_threads - 1) / num_threads;
            let mut parts = vec![Scalar::zero(); num_threads];
            multicore::scope(|scope| {
                for (chunk_idx, (out, coeffs)) in
                    parts.chunks_mut(1).zip(poly.chunks(chunk_size)).enumerate()
                {
                    scope.spawn(move |_| {
                        let start = chunk_idx * chunk_size;
                        out[0] = eval(&coeffs, x) * point.pow_vartime(&[start as u64, 0, 0, 0]);
                    });
                }
            });
            parts.iter().fold(Scalar::zero(), |acc, coeff| acc + coeff)
        }
    }
}
// canonical set of inputs
