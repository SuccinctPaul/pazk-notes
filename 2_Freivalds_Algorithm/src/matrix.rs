use std::ops::AddAssign;
use bls12_381::Scalar;
use ff::Field;
use rand_core::{OsRng, RngCore};

/// This define `matrix` (rows * cols) （m × n）
#[derive(Debug, Clone)]
pub struct Matrix {
    rows: usize,
    cols: usize, // columns
    values: Vec<Vec<Scalar>>,
}


impl Matrix {
    fn random(rows: usize, cols: usize) -> Self {
        let values = (0..rows).map(|_| {
            (0..cols).map(|_| {
                Scalar::random(OsRng)
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        Self {
            cols,
            rows,
            values,
        }
    }
    fn index(&self, row_index:usize, colm_index:usize) -> Scalar {
        assert!(0<= row_index || self.rows>row_index);
        assert!(0<= colm_index || self.cols>colm_index);
        *self.values.get(row_index).unwrap().get(colm_index).unwrap()

    }
    fn get_columns(&self, column_index:usize) -> Vec<Scalar> {
        assert!(0<= column_index || self.cols>column_index);

      self.values.iter().map(|v|{
          v.get(column_index).unwrap().clone()
      }).collect::<Vec<_>>()
    }

    fn vec_mul(a: &Vec<Scalar>, b: &Vec<Scalar>) -> Scalar {
        assert_eq!(a.len(), b.len());

        let mut res = Scalar::zero();
        for (ai, bi) in a.into_iter().zip(b) {
            let producti = ai.mul(bi);
            res.add_assign(producti);
        }
        res
    }

    /// https://en.wikipedia.org/wiki/Dot_product
    /// Suppose A(m * n), B(n, p) => A * B = C(m * p)
    pub fn matrix_mul(m_a: &Matrix, m_b: &Matrix) -> Self {
        assert!(m_a.cols> 0 || m_b.rows> 0, "matrix a is empty");
        assert!(m_b.cols> 0 || m_b.rows> 0, "matrix a is empty");
        // ma.cols == mb.rows
        assert_eq!(m_a.cols, m_b.rows);
        let m = m_a.rows;
        let n = m_a.cols;
        // let n = m_b.rows;
        let p = m_b.cols;


        let mut matrix:Vec<Vec<Scalar>> = Vec::with_capacity(m);
        for i in 0..m {

            let mut new_row = Vec::with_capacity(p);

            let row_i = m_a.values.get(i).unwrap().clone();
            for j in 0..p {
                // todo: this can be optimized by converting m_b columns as rows
                let col_j = m_b.get_columns(j);
                let elem_ij = Self::vec_mul(&row_i, &col_j);
                new_row.push(elem_ij);
            }

            matrix.push(new_row);
        }

        Self{
            rows: m,
            cols: p,
            values: matrix,
        }
    }
}


#[cfg(test)]
mod test {
    use bls12_381::Scalar;
    use crate::matrix::Matrix;

    #[test]
    fn test_random_matrix() {
        let matrix = Matrix::random(3, 4);
        println!("{:#?}", matrix);
    }

    #[test]
    fn test_matrix_mul(){
        let m : usize = 2;
        let mut values : Vec<Vec<Scalar>> = Vec::with_capacity(m);
        let mut row_1: Vec<Scalar> = Vec::with_capacity(m);
        row_1.push(Scalar::one());
        row_1.push(Scalar::zero());
        let mut row_2: Vec<Scalar> = Vec::with_capacity(m);
        row_2.push(Scalar::zero());
        row_2.push(Scalar::one());
        values.push(row_1);
        values.push(row_2);

        let a = Matrix{
            rows: m,
            cols: m,
            values
        };
        let b= a.clone();


        let res = Matrix::matrix_mul(&a, &b);
        assert_eq!(a.values, res.values);
        println!("{:#?}", res);

    }
}