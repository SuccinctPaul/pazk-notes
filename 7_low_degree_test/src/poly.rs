use bls12_381::Scalar;
use ff::Field;
use rand_core::OsRng;
pub use sumcheck::poly::univar_poly::*;

// fi(x) = fi^L (x2) + x fi^R (x2)
pub fn split_poly(p: &Polynomial) -> (Polynomial, Polynomial) {
    assert!(p.degree() != 0, "poly.degree=0, can't split_and_fold");
    // let d = p.degree() + 1;
    let coeffs = p.coeffs();
    let odd: Vec<Scalar> = coeffs.iter().step_by(2).cloned().collect();
    let even: Vec<Scalar> = coeffs.iter().skip(1).step_by(2).cloned().collect();
    // return the fi_L and fi_R
    (Polynomial::from_coeffs(odd), Polynomial::from_coeffs(even))
}

// random a poly with a degree
pub fn random_poly(degree: usize) -> Polynomial {
    assert!(degree >= 0);
    let coeffs = (0..=degree)
        .into_iter()
        .map(|_| Scalar::random(OsRng))
        .collect::<Vec<Scalar>>();
    let poly = Polynomial::from_coeffs(coeffs);
    assert_eq!(poly.degree(), degree);
    poly
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_split() {
        let deg = 5;
        let poly = random_poly(deg);

        let (pL, pR) = split_poly(&poly);

        // check that f(z) == fL(x^2) + x * fR(x^2), for a rand z
        let z = Scalar::random(OsRng);
        assert_eq!(
            poly.evaluate(z.clone()),
            pL.evaluate(z.square()) + z * pR.evaluate(z.square())
        );
    }

    #[test]
    fn test_split_more() {
        // let deg = 5;
        for deg in 1..5 {
            let poly = random_poly(deg);

            let (pL, pR) = split_poly(&poly);

            // check that f(z) == fL(x^2) + x * fR(x^2), for a rand z
            let z = Scalar::random(OsRng);
            assert_eq!(
                poly.evaluate(z.clone()),
                pL.evaluate(z.square()) + z * pR.evaluate(z.square())
            );
        }
    }
}
