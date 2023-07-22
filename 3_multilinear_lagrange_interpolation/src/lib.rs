mod MPolynomial;
mod utils;

// todo test
#[test]
fn factoration_to_coefficient_poly() {
    let factorization = vec![("x", 2), ("x", -1)];
    let mut coefficients: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    for (factor, exponent) in factorization {
        let coefficient = coefficients.entry(exponent).or_insert(0);
        *coefficient += factor.parse::<i32>().unwrap();
    }

    let mut coefficient_polynomial = String::new();
    for (exponent, coefficient) in &coefficients {
        coefficient_polynomial.push_str(&format!("{}x^{} + ", coefficient, exponent));
    }
    // Remove the trailing " + " from the last term
    coefficient_polynomial.pop();
    coefficient_polynomial.pop();

    println!("Coefficient Polynomial: {}", coefficient_polynomial);
}
