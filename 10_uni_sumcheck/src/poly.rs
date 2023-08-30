use bls12_381::Scalar;
use ff::{BatchInvert, Field};
use rand_core::OsRng;
use rayon::{current_num_threads, scope};

// p(x) = = a_0 + a_1 * X + ... + a_n * X^(n-1)
//
// coeffs: [a_0, a_1, ..., a_n]
// basis: X^[n-1]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Polynomial<F: Field> {
    pub(crate) coeffs: Vec<F>,
}

impl<F: Field> Polynomial<F> {
    pub fn random(k: usize) -> Self {
        let n = 1 << k;
        let coeffs = (0..n).map(|_| F::random(OsRng)).collect::<Vec<_>>();
        Self::from_coeffs(coeffs)
    }

    pub fn from_coeffs(coeffs: Vec<F>) -> Self {
        Self { coeffs }
    }

    // used by div.
    pub fn zero() -> Self {
        Self {
            coeffs: vec![F::ZERO],
        }
    }

    // The degree of the polynomial
    pub fn degree(&self) -> usize {
        assert!(self.coeffs.len() > 0);
        self.coeffs.len() - 1
    }
    // The len of the polynomial coeffs
    pub fn len(&self) -> usize {
        self.coeffs.len()
    }

    pub fn coeffs(&self) -> Vec<F> {
        self.coeffs.clone()
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
    pub fn lagrange_interpolate(domains: Vec<F>, evals: Vec<F>) -> Self {
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
            let mut L_j_vec: Vec<Vec<F>> = Vec::with_capacity(poly_size);

            for (j, divisor_j) in divisors.into_iter().enumerate() {
                let mut L_j: Vec<F> = Vec::with_capacity(poly_size);
                L_j.push(F::ONE);

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
                    product.resize(L_j.len() + 1, F::ZERO);

                    // loop (poly_size + 1) round
                    // calculate L_j(X)=∏(X−x_k) divisors_j with coefficient form.
                    for ((a, b), product) in L_j
                        .iter()
                        .chain(std::iter::once(&F::ZERO))
                        .zip(std::iter::once(&F::ZERO).chain(L_j.iter()))
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
            let mut final_poly = vec![F::ZERO; poly_size];
            // 3. p(x)=∑y_j⋅L_j(X)
            for (L_j, y_j) in L_j_vec.iter().zip(evals) {
                for (final_coeff, L_j_coeff) in final_poly.iter_mut().zip(L_j.into_iter()) {
                    *final_coeff += L_j_coeff.mul(y_j);
                }
            }
            Self { coeffs: final_poly }
        }
    }

    // This evaluates a polynomial (in coefficient form) at `x`.
    pub fn evaluate(&self, x: F) -> F {
        let coeffs = self.coeffs.clone();
        let poly_size = self.coeffs.len();

        // p(x) = = a_0 + a_1 * X + ... + a_n * X^(n-1), revert it and fold sum it
        fn eval<F: Field>(poly: &[F], point: F) -> F {
            poly.iter()
                .rev()
                .fold(F::ZERO, |acc, coeff| acc * point + coeff)
        }

        let num_threads = current_num_threads();
        if poly_size * 2 < num_threads {
            eval(&coeffs, x)
        } else {
            let chunk_size = (poly_size + num_threads - 1) / num_threads;
            let mut parts = vec![F::ZERO; num_threads];
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
            parts.iter().fold(F::ZERO, |acc, coeff| acc + coeff)
        }
    }
}

impl<F: Field> std::ops::Mul<&Polynomial<F>> for &Polynomial<F> {
    type Output = Polynomial<F>;
    fn mul(self, rhs: &Polynomial<F>) -> Self::Output {
        let mut coeffs: Vec<F> = vec![F::ZERO; self.coeffs.len() + rhs.coeffs.len() - 1];
        for n in 0..self.coeffs.len() {
            for m in 0..rhs.coeffs.len() {
                coeffs[n + m] += self.coeffs[n] * rhs.coeffs[m];
            }
        }
        Self::Output { coeffs }
    }
}

impl<F: Field> std::ops::Mul<&F> for &Polynomial<F> {
    type Output = Polynomial<F>;
    fn mul(self, rhs: &F) -> Self::Output {
        let coeffs = if rhs == &F::ZERO {
            vec![F::ZERO]
        } else {
            self.coeffs.iter().map(|c| c.mul(rhs)).collect::<Vec<F>>()
        };
        Self::Output { coeffs }
    }
}

impl<F: Field> std::ops::Add<&Polynomial<F>> for &Polynomial<F> {
    type Output = Polynomial<F>;

    fn add(self, rhs: &Polynomial<F>) -> Self::Output {
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
            .collect::<Vec<F>>();
        Self::Output { coeffs }
    }
}
// impl<F: Field> std::ops::Div for &Polynomial<F> {
//     type Output = (Polynomial<F>, Polynomial<F>);
//
//     fn div(self, rhs: &Polynomial<F>) -> Self::Output {
//         // init the (quotient, remainder)
//         let (mut q, mut r) = (Polynomial::zero(), self);
//
//         // r is not zero poly, and division.degree > divisor.degree.
//         while *r != Polynomial::zero() && r.degree() >= rhs.degree() {
//             let r_coeff = r.coeffs();
//             let rhs_coeff = rhs.coeffs();
//
//             let lead_r = r_coeff[r_coeff.len() - 1];
//             let lead_d = rhs_coeff[rhs_coeff.len() - 1];
//             let mut t = Polynomial::zero();
//             t.set(
//                 r_coeff.len() - rhs_coeff.len(),
//                 lead_r * lead_d.invert().unwrap(),
//             );
//             q += &t;
//             r -= &(&rhs * &t);
//         }
//         (q, r)
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    use ff::PrimeField;
    use std::ops::{Add, Div, Mul};

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

    #[test]
    fn lagrange_interpolate() {
        // aim: p = 1 + 2x + x^2

        let domain = vec![
            Scalar::from_u128(1),
            Scalar::from_u128(2),
            Scalar::from_u128(3),
            Scalar::from_u128(4),
            Scalar::from_u128(5),
            Scalar::from_u128(6),
            Scalar::from_u128(7),
            Scalar::from_u128(8),
            Scalar::from_u128(9),
        ];
        let evals = vec![
            Scalar::from_u128(4),
            Scalar::from_u128(9),
            Scalar::from_u128(10),
            Scalar::from_u128(19),
            Scalar::from_u128(24),
            Scalar::from_u128(31),
            Scalar::from_u128(40),
            Scalar::from_u128(51),
            Scalar::from_u128(64),
        ];

        let poly = Polynomial::lagrange_interpolate(domain.clone(), evals.clone());

        for (x, y) in domain.iter().zip(evals) {
            assert_eq!(poly.evaluate(*x), y);
        }
        println!("pass");
    }

    // #[test]
    // fn test_div() {
    //     // division: 2+3x+x^2 = (x+1)(x+2)
    //     let coeffs = vec![Scalar::from_u128(2), Scalar::ONE, Scalar::ONE];
    //     let division = Polynomial::from_coeffs(coeffs);
    //
    //     // dividor: 2+x
    //     let coeffs = vec![Scalar::from_u128(2), Scalar::ONE];
    //     let dividor = Polynomial::from_coeffs(coeffs);
    //
    //     // target:
    //     //      quotient poly: 1+x
    //     //      remainder poly: 0
    //     let coeffs = vec![Scalar::from_u128(2), Scalar::ONE];
    //     let target_qoutient = Polynomial::from_coeffs(coeffs);
    //     let target_remainder = Polynomial::zero();
    //
    //     // division / dividor = quotient + remainder
    //     let (actual_qoutient, actual_remainder) = division.div(dividor);
    //
    //     assert_eq!(actual_qoutient, target_qoutient);
    //     assert_eq!(actual_remainder, target_remainder);
    // }
}
