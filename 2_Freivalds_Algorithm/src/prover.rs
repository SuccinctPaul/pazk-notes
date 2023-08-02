use crate::matrix::Matrix;
use bls12_381::Scalar;
use ff::Field;

pub struct Prover {
    a: Matrix,
    b: Matrix,
}

impl Prover {
    pub fn random(n: usize) -> Self {
        Self {
            a: Matrix::random(n, n),
            b: Matrix::random(n, n),
        }
    }

    pub fn matrix_multiplication(&self) -> Matrix {
        Matrix::mul(&self.a, &self.b)
    }

    pub fn hash(&self, x: &Vec<Scalar>) -> Vec<Scalar> {
        // tmp = Bx
        let tmp = self.b.matrix_mul_vec(x);
        // z = A temp = A(Bx)
        self.a.matrix_mul_vec(&tmp)
    }
}
