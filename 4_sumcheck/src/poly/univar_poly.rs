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
