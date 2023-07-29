use bls12_381::Scalar;

// convert a num into its binary form
// eg: 8 -> 1000, will output [1, 0, 0, 0]
pub fn convert_to_binary(bit_len: &usize, num: usize) -> Vec<usize> {
    let a: usize = 1 << bit_len;
    assert!(a >= num);
    (0..*bit_len)
        .map(|n| (num >> n) & 1)
        .rev()
        .collect::<Vec<usize>>()
}

// convert a num into its binary form
// eg: 8 -> 1000, will output [1, 0, 0, 0]
pub fn convert_from_binary(num: &Vec<usize>) -> usize {
    num.iter().rev().enumerate().map(|(i, n)| n << i).sum()
}

// try to expand factorization form to coeffs form for `uni-variable poly`
// For now, we'll only support two factorizations to a coeffs.
// eg: (4x^2 + 1)(x + 4) = 4x^3 + 4x^2 + x + 4
fn expand_factor_for_upoly(lhs: Vec<Scalar>, rhs: Vec<Scalar>) -> Vec<Scalar> {
    let target_len = (lhs.len() - 1) * (rhs.len() - 1) + 1;

    let mut product = vec![Scalar::zero(); target_len];

    for n in 0..lhs.len() {
        for m in 0..rhs.len() {
            product[n + m] += lhs[n] * rhs[m];
        }
    }
    product
}

// try to expand factorization form to coeffs form for multi-variables poly
// For now, we'll only support two factorizations to a coeffs.
// eg: f(x1,x2) = (1+x1) * (1−x2) = 1 + x1 - x2 - x1x2
//
// NOTE!!! Only support var_num=2, which mul form such as: (a+bx1) * (c+dx2) = ac + bc*x1 + ad*x2 + bd*x1*x2
pub fn expand_factor_for_mpoly(var_num: usize, lhs: Vec<Scalar>, rhs: Vec<Scalar>) -> Vec<Scalar> {
    let target_len = 1 << var_num;
    assert_eq!(target_len, lhs.len());
    assert_eq!(target_len, rhs.len());

    let mut product = vec![Scalar::zero(); target_len];

    for (n, l) in lhs.iter().enumerate() {
        for (m, r) in rhs.iter().enumerate() {
            product[n | m] += l * r;
        }
    }
    product
}

#[cfg(test)]
mod test {
    use crate::utils::{
        convert_from_binary, convert_to_binary, expand_factor_for_mpoly, expand_factor_for_upoly,
    };
    use bls12_381::Scalar;
    use ff::PrimeField;
    use rayon::scope;

    #[test]
    fn test_expand_factor_for_upoly() {
        // f1 = 1 + x
        let poly_one = vec![Scalar::one(), Scalar::one()];
        // f2 = -1 + x
        let poly_two = vec![Scalar::one().neg(), Scalar::one()];
        // product = f1 * f2 = -1 + x^2
        let target = vec![Scalar::one().neg(), Scalar::zero(), Scalar::one()];

        let actual = expand_factor_for_upoly(poly_one, poly_two);
        assert_eq!(target, actual);
        println!("{:?}", actual);
    }

    #[test]
    fn test_expand_factor_for_mpoly() {
        // NOTE!!! Only support var_num=2, which mul form such as: (a+bx1) * (c+dx2) = ac + bc*x1 + ad*x2 + bd*x1*x2
        let var_num: usize = 2;

        // f1 = (1+x1)
        //         terms: (0,0) = 1, (0,1)=0, (1,0)=1,  (1,1)=0
        let poly_one = vec![Scalar::one(), Scalar::zero(), Scalar::one(), Scalar::zero()];
        // f2 = (2−x2)
        //         terms: (0,0) = 2, (0,1)=-1, (1,0)=0, (1,1)=0
        let poly_two = vec![
            Scalar::from_u128(2),
            Scalar::one().neg(),
            Scalar::zero(),
            Scalar::zero(),
        ];
        // product: F(x1,x2) = f1 * f2 = 2 - x2 + 2x1 - x1x2
        //         terms: (0,0) = 2, (0,1)=-1, (1,0)=2, (1,1)=-1
        let target = vec![
            Scalar::from_u128(2),
            Scalar::one().neg(),
            Scalar::from_u128(2),
            Scalar::one().neg(),
        ];

        let actual = expand_factor_for_mpoly(var_num, poly_one, poly_two);
        assert_eq!(target, actual);
        println!("{:?}", actual);
    }

    #[test]
    fn test_convert() {
        let raw: Vec<usize> = vec![1, 2, 3, 4, 8, 17];

        let binaries = raw
            .iter()
            .map(|i| convert_to_binary(&5, i.clone()))
            .collect::<Vec<_>>();

        let new_nums = binaries
            .iter()
            .map(|b| convert_from_binary(b))
            .collect::<Vec<_>>();

        assert_eq!(new_nums, raw);
    }

    #[test]
    fn test_convert_into_binary() {
        for i in [1, 2, 3, 4, 8, 17] {
            let binary = convert_to_binary(&4, i);
            println!("{:?} ->  {:?}", i, binary);
        }
    }

    #[test]
    fn test_convert_from_binary() {
        let mut binary: Vec<usize> = vec![1, 0, 0, 0, 1];
        let raw = convert_from_binary(&binary);
        println!("{:?} ->  {:?}", raw, binary);
    }
}
