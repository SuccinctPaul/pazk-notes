use bls12_381::Scalar;
use ff::BatchInvert;
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
            let poly_size = domains.len();
            let lag_basis_poly_size = poly_size - 1;

            // 1. divisors = vec(x_j - x_k). prepare for L_j(X)=∏(X−x_k)/(x_j−x_k)
            let mut divisors = Vec::with_capacity(poly_size);
            for (j, x_j) in domains.iter().enumerate() {
                // divisor_j
                let mut divisor = Vec::with_capacity(lag_basis_poly_size);
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

            // 2. Calculate  L_j(X) : L_j(X)=∏(X−x_k) divisors_j
            let mut L_j_vec: Vec<Vec<Scalar>> = Vec::with_capacity(poly_size);

            for (j, divisor_j) in divisors.into_iter().enumerate() {
                let mut L_j: Vec<Scalar> = Vec::with_capacity(poly_size);
                L_j.push(Scalar::one());

                // (X−x_k) * divisors_j
                let mut product = Vec::with_capacity(lag_basis_poly_size);

                // obtain domain for x_k
                for (x_k, divisor) in domains
                    .iter()
                    .enumerate()
                    .filter(|&(k, _)| k != j)
                    .map(|(_, x)| x)
                    .zip(divisor_j.into_iter())
                {
                    product.resize(L_j.len() + 1, Scalar::zero());

                    // loop (poly_size + 1) round
                    // calculate L_j(X)=∏(X−x_k) divisors_j with coefficient form.
                    for ((a, b), product) in L_j
                        .iter()
                        .chain(std::iter::once(&Scalar::zero()))
                        .zip(std::iter::once(&Scalar::zero()).chain(L_j.iter()))
                        .zip(product.iter_mut())
                    {
                        *product = *a * (-divisor * x_k) + *b * divisor;
                    }
                    std::mem::swap(&mut L_j, &mut product);
                }

                assert_eq!(L_j.len(), poly_size);
                assert_eq!(product.len(), poly_size - 1);

                L_j_vec.push(L_j);
            }

            // p(x)=∑y_j⋅L_j(X) in coefficients
            let mut final_poly = vec![Scalar::zero(); poly_size];
            // 3. p(x)=∑y_j⋅L_j(X)
            for (L_j, y_j) in L_j_vec.iter().zip(evals) {
                for (final_coeff, L_j_coeff) in final_poly.iter_mut().zip(L_j.into_iter()) {
                    *final_coeff += L_j_coeff * y_j;
                }
            }
            Self { coeffs: final_poly }
        }
    }

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

impl std::ops::Mul<&Polynomial> for &Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: &Polynomial) -> Self::Output {
        let mut coeffs: Vec<Scalar> =
            vec![Scalar::zero(); self.coeffs.len() + rhs.coeffs.len() - 1];
        for n in 0..self.coeffs.len() {
            for m in 0..rhs.coeffs.len() {
                coeffs[n + m] += self.coeffs[n] * rhs.coeffs[m];
            }
        }
        Self::Output { coeffs }
    }
}

impl std::ops::Mul<&Scalar> for &Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: &Scalar) -> Self::Output {
        let coeffs = if rhs == &Scalar::zero() {
            vec![Scalar::zero()]
        } else {
            self.coeffs.iter().map(|c| c * rhs).collect::<Vec<Scalar>>()
        };
        Self::Output { coeffs }
    }
}

impl std::ops::Add<&Polynomial> for &Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: &Polynomial) -> Self::Output {
        let max_len = std::cmp::max(self.coeffs.len(), rhs.coeffs.len());
        let coeffs = (0..max_len)
            .into_iter()
            .map(|n| {
                if n >= self.coeffs.len() {
                    rhs.coeffs[n]
                } else if n >= rhs.coeffs.len() {
                    self.coeffs[n]
                } else {
                    // n < self.0.len() && n < rhs.0.len()
                    self.coeffs[n] + rhs.coeffs[n]
                }
            })
            .collect::<Vec<Scalar>>();
        Self::Output { coeffs }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ff::PrimeField;
    use std::ops::{Add, Mul};

    #[test]
    fn test_mul_poly() {
        // p = 1 - x
        let p = Polynomial {
            coeffs: vec![Scalar::one(), Scalar::one().neg()],
        };
        // q = 1 + x
        let q = Polynomial {
            coeffs: vec![Scalar::one(), Scalar::one()],
        };

        assert_eq!(
            p.mul(&q).coeffs,
            vec![Scalar::one(), Scalar::zero(), Scalar::one().neg()]
        );

        // add
        assert_eq!(p.add(&q).coeffs, vec![Scalar::from_u128(2), Scalar::zero()]);

        // poly.mul(scalar)
        assert_eq!(
            p.mul(&Scalar::from_u128(5)).coeffs,
            vec![Scalar::from_u128(5), Scalar::from_u128(5).neg()]
        );
    }
}
