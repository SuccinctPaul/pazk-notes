use crate::utils::read_from_file;
use crate::Person;
use bls12_381::Scalar;
use ff::Field;
use rand_core::OsRng;
use std::fs::read;
use std::ops::MulAssign;

pub trait Prover<F: Field> {
    fn challenge() -> F {
        F::random(OsRng)
    }
    fn fs_hash(&self, f: F) -> F;
}

impl Prover<Scalar> for Person {
    /// FS fingerprint:
    /// 1. generate random r
    /// 2. hash(data) = data[0]*r^0 + data[1]*r^1 + ... + data[n-1]*r^(n-1)
    fn fs_hash(&self, r: Scalar) -> Scalar {
        let mut cur_r = Scalar::one();
        self.data
            .iter()
            .enumerate()
            .map(|(_, a)| {
                let res = a.mul(&cur_r);
                cur_r.mul_assign(r);
                res
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use bls12_381::Scalar;

    #[test]
    fn test_scalar_pow() {
        let one = Scalar::one();

        let two = one.add(&one);
        // println!("1+1 {:?}", one.add(&one));
        println!("2^2 {:?}", two.pow(&[2, 0, 0, 0]));
    }
}
