use crate::math::matrix::NMatrix;
use crate::math::vector::NVector;

pub fn assert_equal_nvector_custom_precision(v1: &NVector, v2: &NVector, precision: f64) {
    assert_eq!(v1.n, v2.n, "Vectors must have the same length.");

    for i in 0..v1.n {
        let diff = (v1.get(i) - v2.get(i)).abs();
        assert!(
            diff < precision,
            "Vectors differ at index {}: expected {}, got {}",
            i,
            v2.get(i),
            v1.get(i)
        );
    }
}

pub fn assert_equal_nmatrix_custom_precision(m1: &NMatrix, m2: &NMatrix, precision: f64) {
    assert_eq!(m1.rows, m2.rows, "Matrices must have the same number of rows.");
    assert_eq!(m1.cols, m2.cols, "Matrices must have the same number of columns.");

    for i in 0..m1.rows {
        for j in 0..m1.cols {
            let diff = (m1.get(i, j) - m2.get(i, j)).abs();
            assert!(
                diff < precision,
                "Matrices differ at ({}, {}): expected {}, got {}",
                i,
                j,
                m2.get(i, j),
                m1.get(i, j)
            );
        }
    }
}

pub fn assert_equal_nvector(v1: &NVector, v2: &NVector) {
    assert_equal_nvector_custom_precision(v1, v2, 0.00001);
}

pub fn assert_equal_nmatrix(m1: &NMatrix, m2: &NMatrix) {
    assert_equal_nmatrix_custom_precision(m1, m2, 0.00001);
}
