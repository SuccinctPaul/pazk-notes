use bls12_381::Scalar as Fr;
use ff::{Field, PrimeField};

/// A polynomial field scalar
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly<F: Field>(pub Vec<F>);

// NOTE! only used for Bn256::Fr.
impl Poly<Fr> {
    /// Creates a Poly from u64 coeffs
    pub fn from(coeffs: &[u64]) -> Self {
        Poly::new(coeffs.iter().map(|n| Fr::from(*n)).collect::<Vec<Fr>>())
    }
}

impl<F: Field> Poly<F> {
    /// Creates a Poly from its `coefficients` with order of poly `coeff[0]*x^0, ..., coeff[m-1]*x^m`
    /// for safety, input value is normalized (trailing zeroes are removed)
    pub fn new(coeffs: Vec<F>) -> Self {
        let mut poly = Poly(coeffs);
        poly.normalize();
        poly
    }

    /// Returns p(x)=0
    pub fn zero() -> Self {
        Poly(vec![F::ZERO])
    }

    /// Returns p(x)=1
    pub fn one() -> Self {
        Poly(vec![F::ONE])
    }

    /// Creates a poly that satisfy a set of `p` points, by using [lagrange](https://alinush.github.io/2022/07/28/lagrange-interpolation.html)
    ///
    /// ϕ(X)=∑yi⋅Li(X), where Li(X)=∏(X−xj)/(xi−xj)
    ///
    /// ## Examples
    /// ```text
    ///    // f(x)=x is a polinomial that fits in (1,1), (2,2) points
    ///    assert_eq!(
    ///      Poly::lagrange(&vec![
    ///          (Scalar::from(1), Scalar::from(1)),
    ///          (Scalar::from(2), Scalar::from(2))
    ///      ]),
    ///      Poly::from(&[0, 1]) // f(x) = x
    ///    );
    /// ```
    pub fn lagrange(p: &[(F, F)]) -> Self {
        let k = p.len();
        let mut l = Poly::zero();
        for j in 0..k {
            let mut l_j = Poly::one();
            for i in 0..k {
                if i != j {
                    let c = (p[j].0 - p[i].0).invert().unwrap();
                    l_j = &l_j * &Poly::new(vec![-(c * p[i].0), c]);
                }
            }
            l += &(&l_j * &p[j].1);
        }
        l
    }

    /// Evals the polynomial at the A point
    /// # Examples
    /// ```text
    ///    // check that (x^2+2x+1)(2) = 9
    ///     let poly = Poly::from(&[1, 2, 1])
    ///     assert_eq!(
    ///         poly.eval(&Scalar::from(2)),
    ///         Scalar::from(9)
    ///     );
    /// ```
    pub fn eval(&self, x: &F) -> F {
        let mut x_pow = F::ONE;
        let mut y = self.0[0];
        for (i, _) in self.0.iter().enumerate().skip(1) {
            x_pow *= x;
            y += &(x_pow * self.0[i]);
        }
        y
    }

    /// Evals the polynomial supplying the `x_pows` [x^0, x^1, x^2, ..., x^m]
    pub fn eval_with_pows(&self, x_pow: &[F]) -> F {
        let mut y = self.0[0];
        for (i, _) in self.0.iter().enumerate() {
            y += &(x_pow[i] * self.0[i]);
        }
        y
    }

    /// Returns the degree of the polynomial
    ///
    /// poly.size = poly.degree + 1
    pub fn degree(&self) -> usize {
        self.0.len() - 1
    }

    /// Returns the coeffs size of the polynomial
    pub fn size(&self) -> usize {
        self.0.len()
    }

    /// Normalizes the coefficients, removing ending zeroes
    /// # Examples
    /// ```text
    ///    use a0kzg::Poly;
    ///    let mut p1 = Poly::from(&[1, 0, 0, 0]);
    ///    p1.normalize();
    ///    assert_eq!(p1, Poly::from(&[1]));
    /// ```
    pub fn normalize(&mut self) {
        if self.0.len() > 1 && self.0[self.0.len() - 1] == F::ZERO {
            let zero = F::ZERO;
            let first_non_zero = self.0.iter().rev().position(|p| p != &zero);
            if let Some(first_non_zero) = first_non_zero {
                self.0.resize(self.0.len() - first_non_zero, F::ZERO);
            } else {
                self.0.resize(1, F::ZERO);
            }
        }
    }

    /// Returns if p(x)=0
    pub fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    /// Set the `i`-th coefficient with new value
    /// # Examples
    /// ```text
    ///   let mut poly = Poly::zero();
    ///   poly.set(2, Scalar::from(7));
    ///   assert_eq!(poly, Poly::from(&[0, 0, 7]));
    ///  ```
    pub fn set(&mut self, index: usize, p: F) {
        let target_size = index + 1;
        if self.size() < target_size {
            self.0.resize(target_size, F::ZERO);
        }
        self.0[index] = p;
        self.normalize();
    }

    /// Returns the `i`-th coefficient
    /// # Examples
    /// ```text
    ///   let mut poly = Poly::zero();
    ///   poly.set(2, Scalar::from(7));
    ///   assert_eq!(poly.get(2), Some(&Scalar::from(7)));
    ///   assert_eq!(poly.get(3), None);
    ///  ```
    pub fn get(&mut self, index: usize) -> Option<&F> {
        self.0.get(index)
    }
}

impl<F: Field> std::ops::AddAssign<&Poly<F>> for Poly<F> {
    fn add_assign(&mut self, rhs: &Poly<F>) {
        for n in 0..std::cmp::max(self.0.len(), rhs.0.len()) {
            if n >= self.0.len() {
                self.0.push(rhs.0[n]);
            } else if n < self.0.len() && n < rhs.0.len() {
                self.0[n] += rhs.0[n];
            }
        }
        self.normalize();
    }
}

impl<F: Field> std::ops::AddAssign<&F> for Poly<F> {
    fn add_assign(&mut self, rhs: &F) {
        self.0[0] += rhs;
    }
}

impl<F: Field> std::ops::SubAssign<&Poly<F>> for Poly<F> {
    fn sub_assign(&mut self, rhs: &Poly<F>) {
        for n in 0..std::cmp::max(self.0.len(), rhs.0.len()) {
            if n >= self.0.len() {
                self.0.push(rhs.0[n]);
            } else if n < self.0.len() && n < rhs.0.len() {
                self.0[n] -= rhs.0[n];
            }
        }
        self.normalize();
    }
}

impl<F: Field> std::ops::Mul<&Poly<F>> for &Poly<F> {
    type Output = Poly<F>;
    fn mul(self, rhs: &Poly<F>) -> Self::Output {
        let mut mul: Vec<F> = std::iter::repeat(F::ZERO)
            .take(self.0.len() + rhs.0.len() - 1)
            .collect();
        for n in 0..self.0.len() {
            for m in 0..rhs.0.len() {
                mul[n + m] += self.0[n] * rhs.0[m];
            }
        }
        Poly(mul)
    }
}

impl<F: Field> std::ops::Mul<&F> for &Poly<F> {
    type Output = Poly<F>;
    fn mul(self, rhs: &F) -> Self::Output {
        if rhs == &F::ZERO {
            Poly::zero()
        } else {
            Poly(self.0.iter().map(|v| *v * *rhs).collect::<Vec<_>>())
        }
    }
}

impl<F: Field> std::ops::Div for Poly<F> {
    type Output = (Poly<F>, Poly<F>);

    fn div(self, rhs: Poly<F>) -> Self::Output {
        let (mut q, mut r) = (Poly::zero(), self);
        while !r.is_zero() && r.degree() >= rhs.degree() {
            let lead_r = r.0[r.0.len() - 1];
            let lead_d = rhs.0[rhs.0.len() - 1];
            let mut t = Poly::zero();
            t.set(r.0.len() - rhs.0.len(), lead_r * lead_d.invert().unwrap());
            q += &t;
            r -= &(&rhs * &t);
        }
        (q, r)
    }
}
impl<F: PrimeField> std::fmt::Display for Poly<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first: bool = true;
        for i in (0..=self.degree()).rev() {
            let bi_n =
                num_bigint::BigUint::from_bytes_le(&self.0[i].to_repr().as_ref()).to_str_radix(10);
            let bi_inv = num_bigint::BigUint::from_bytes_le(self.0[i].neg().to_repr().as_ref())
                .to_str_radix(10);

            if bi_n == "0" {
                continue;
            }

            if bi_inv.len() < 20 && bi_n.len() > 20 {
                if bi_inv == "1" && i != 0 {
                    write!(f, "-")?;
                } else {
                    write!(f, "-{}", bi_inv)?;
                }
            } else {
                if !first {
                    write!(f, "+")?;
                }
                if i == 0 || bi_n != "1" {
                    write!(f, "{}", bi_n)?;
                }
            }
            if i >= 1 {
                write!(f, "x")?;
            }
            if i >= 2 {
                write!(f, "^{}", i)?;
            }
            first = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bls12_381::Scalar as Fr;
    use bls12_381::*;

    #[test]
    fn test_poly_add() {
        let mut p246 = Poly::from(&[1, 2, 3]);
        p246 += &Poly::from(&[1, 2, 3]);
        assert_eq!(p246, Poly::from(&[2, 4, 6]));

        let mut p24645 = Poly::from(&[1, 2, 3]);
        p24645 += &Poly::from(&[1, 2, 3, 4, 5]);
        assert_eq!(p24645, Poly::from(&[2, 4, 6, 4, 5]));

        let mut p24646 = Poly::from(&[1, 2, 3, 4, 6]);
        p24646 += &Poly::from(&[1, 2, 3]);
        assert_eq!(p24646, Poly::from(&[2, 4, 6, 4, 6]));
    }

    #[test]
    fn test_poly_sub() {
        let mut p0 = Poly::from(&[1, 2, 3]);
        p0 -= &Poly::from(&[1, 2, 3]);
        assert_eq!(p0, Poly::from(&[0]));

        let mut p003 = Poly::from(&[1, 2, 3]);
        p003 -= &Poly::from(&[1, 2]);
        assert_eq!(p003, Poly::from(&[0, 0, 3]));
    }

    #[test]
    fn test_poly_mul() {
        assert_eq!(
            &Poly::from(&[5, 0, 10, 6]) * &Poly::from(&[1, 2, 4]),
            Poly::from(&[5, 10, 30, 26, 52, 24])
        );
    }

    #[test]
    fn test_div() {
        fn do_test<F: Field>(n: Poly<F>, d: Poly<F>) {
            let (q, r) = n.clone() / d.clone();
            let mut n2 = &q * &d;
            n2 += &r;
            assert_eq!(n, n2);
        }

        do_test(Poly::<Fr>::from(&[1]), Poly::<Fr>::from(&[1, 1]));
        do_test(Poly::<Fr>::from(&[1, 1]), Poly::<Fr>::from(&[1, 1]));
        do_test(Poly::<Fr>::from(&[1, 2, 1]), Poly::<Fr>::from(&[1, 1]));
        do_test(
            Poly::<Fr>::from(&[1, 2, 1, 2, 5, 8, 1, 9]),
            Poly::<Fr>::from(&[1, 1, 5, 4]),
        );
    }

    #[test]
    fn test_print() {
        assert_eq!("x^2+2x+1", format!("{}", Poly::from(&[1, 2, 1])));
        assert_eq!("x^2+1", format!("{}", Poly::from(&[1, 0, 1])));
        assert_eq!("x^2", format!("{}", Poly::from(&[0, 0, 1])));
        assert_eq!("2x^2", format!("{}", Poly::from(&[0, 0, 2])));
        assert_eq!("-4", format!("{}", Poly::new(vec![-Fr::from(4)])));
        assert_eq!(
            "-4x",
            format!("{}", Poly::new(vec![Fr::zero(), -Fr::from(4)]))
        );
        assert_eq!(
            "-x-2",
            format!("{}", Poly::new(vec![Fr::from(2).neg(), Fr::from(1).neg()]))
        );
        assert_eq!(
            "x-2",
            format!("{}", Poly::new(vec![-Fr::from(2), Fr::from(1)]))
        );
    }

    #[test]
    fn test_lagrange_multi() {
        let points = vec![
            (Fr::from(12342), Fr::from(22342)),
            (Fr::from(2234), Fr::from(22222)),
            (Fr::from(3982394), Fr::from(111114)),
            (Fr::from(483838), Fr::from(444444)),
        ];
        let l = Poly::lagrange(&points);
        points.iter().for_each(|p| assert_eq!(l.eval(&p.0), p.1));
    }
}
