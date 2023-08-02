use bls12_381::Scalar;
use rayon::{current_num_threads, scope};

// p(x) = = a_0 + a_1 * X + ... + a_n * X^(n-1)
//
// coeffs: [a_0, a_1, ..., a_n]
// basis: X^[n-1]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Polynomial {
    pub(crate) coeffs: Vec<Scalar>,
}

impl Polynomial {
    // This evaluates a polynomial (in coefficient form) at `x`.
    pub fn evaluate(&self, x: Scalar) -> Scalar {
        let coeffs = self.coeffs.clone();
        let poly_size = self.coeffs.len();

        // p(x) = = a_0 + a_1 * X + ... + a_n * X^(n-1), revert it and fold sum it
        fn eval(poly: &[Scalar], point: Scalar) -> Scalar {
            poly.iter()
                .rev()
                .fold(Scalar::zero(), |acc, coeff| acc * point + coeff)
        }

        let num_threads = current_num_threads();
        if poly_size * 2 < num_threads {
            eval(&coeffs, x)
        } else {
            let chunk_size = (poly_size + num_threads - 1) / num_threads;
            let mut parts = vec![Scalar::zero(); num_threads];
            scope(|scope| {
                for (chunk_idx, (out, c)) in parts
                    .chunks_mut(1)
                    .zip(coeffs.chunks(chunk_size))
                    .enumerate()
                {
                    scope.spawn(move |_| {
                        let start = chunk_idx * chunk_size;
                        out[0] = eval(c, x) * x.pow_vartime(&[start as u64, 0, 0, 0]);
                    });
                }
            });
            parts.iter().fold(Scalar::zero(), |acc, coeff| acc + coeff)
        }
    }
}
