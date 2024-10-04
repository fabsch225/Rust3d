use crate::math::matrix::NMatrix;
use crate::math::vector::NVector;

impl NMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        NMatrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut result = NMatrix::new(size, size);
        for i in 0..size {
            result.set(i, i, 1.0);
        }
        result
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        assert_eq!(rows * cols, data.len());
        NMatrix { rows, cols, data }
    }

    pub fn from_field(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        let mut result = NMatrix::new(rows, cols);
        for i in 0..rows {
            assert_eq!(cols, data[i].len());
            for j in 0..cols {
                result.set(i, j, data[i][j]);
            }
        }

        result
    }

    pub fn translation_matrix(v: &NVector) -> NMatrix {
        let mut result = NMatrix::identity(v.n + 1);
        for i in 0..v.n {
            result.set(v.n, i, v.get(i));
        }
        result
    }

    pub fn outer_product(v1: &NVector, v2: &NVector) -> NMatrix {
        let rows = v1.n;
        let cols = v2.n;
        let mut result = NMatrix::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                result.set(i, j, v1.get(i) * v2.get(j));
            }
        }

        result
    }

    pub fn householder_reflection(v: &NVector) -> NMatrix {
        let mut v = v.clone();
        assert!(!v.is_null());

        v.normalize();
        let size = v.len();
        let mut h = NMatrix::identity(size);

        for i in 0..size {
            for j in 0..size {
                let value = -2.0 * v.get(i) * v.get(j);
                h.add_entry(i, j, value);
            }
        }

        h
    }
}