use crate::matrix::Matrix;
use bls12_381::Scalar;
use ff::Field;
use rand_core::RngCore;
use std::ops::MulAssign;

/// x=(1,r,r2,...,rn−1)
pub fn gen_x(rng: impl RngCore, n: usize) -> Vec<Scalar> {
    let r: Scalar = Scalar::random(rng);

    let mut cur_r = Scalar::one();
    (0..n)
        .map(|_| {
            let res = cur_r;
            cur_r.mul_assign(r);
            res
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use bls12_381::Scalar;
    use ff::Field;
    use rand_core::OsRng;
    use std::ops::MulAssign;

    #[test]
    fn test_gen_x() {
        let n = 4;

        let r: Scalar = Scalar::random(OsRng);

        let target = vec![Scalar::one(), r, r.mul(&r), r.mul(&r).mul(&r)];

        // Param::gen_x
        let mut cur_r = Scalar::one();
        let x = (0..n)
            .map(|_| {
                let res = cur_r;
                cur_r.mul_assign(r);
                res
            })
            .collect::<Vec<_>>();

        assert_eq!(target, x);
        println!("{:?}", x);
    }
}
