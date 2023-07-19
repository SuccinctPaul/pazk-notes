use crate::polynomial::Polynomial;
use bls12_381::Scalar;
use ff::PrimeField;

mod polynomial;
mod utils;

/// Encode vector into polynomial.

#[test]
fn encode() {
    let two = Scalar::one().add(&Scalar::one());

    // p = 1 + 2x + x^2
    let a = vec![Scalar::one(), two, Scalar::one()];

    let poly = Polynomial::encode(a);

    let z = poly.evaluate(Scalar::one());

    assert_eq!(Scalar::from_u128(4), z);

    let z = poly.evaluate(two.double());
    assert_eq!(Scalar::from_u128(25), z);

    for i in 1..10 {
        println!("{:?}", poly.evaluate(Scalar::from_u128(i)));
    }
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

    let z = poly.evaluate(Scalar::from_u128(3));
    println!("{:?}", z);

    // meet errors
    for (x, y) in domain.iter().zip(evals) {
        assert_eq!(poly.evaluate(*x), y);
    }
}
