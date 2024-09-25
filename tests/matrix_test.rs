#[cfg(test)]
mod tests {
    use rust3d::math::{matrix::NMatrix, vector::NVector};

    #[test]
    fn test_nmatrix_creation() {
        let m = NMatrix::new(3, 3);
        assert_eq!(m.rows, 3);
        assert_eq!(m.cols, 3);
        assert_eq!(m.data.len(), 9);
        for &val in &m.data {
            assert_eq!(val, 0.0);
        }
    }

    #[test]
    fn test_nmatrix_from_vec() {
        let m = NMatrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 1), 2.0);
        assert_eq!(m.get(1, 0), 3.0);
        assert_eq!(m.get(1, 1), 4.0);
    }

    #[test]
    fn test_nmatrix_set_and_get() {
        let mut m = NMatrix::new(2, 2);
        m.set(0, 0, 5.0);
        assert_eq!(m.get(0, 0), 5.0);
        m.set(1, 1, 10.0);
        assert_eq!(m.get(1, 1), 10.0);
    }

    #[test]
    fn test_nmatrix_addition() {
        let m1 = NMatrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = NMatrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let result = m1.add(&m2);
        assert_eq!(result.get(0, 0), 6.0);
        assert_eq!(result.get(0, 1), 8.0);
        assert_eq!(result.get(1, 0), 10.0);
        assert_eq!(result.get(1, 1), 12.0);
    }

    #[test]
    fn test_nmatrix_subtraction() {
        let m1 = NMatrix::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let m2 = NMatrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let result = m1.subtract(&m2);
        assert_eq!(result.get(0, 0), 4.0);
        assert_eq!(result.get(0, 1), 4.0);
        assert_eq!(result.get(1, 0), 4.0);
        assert_eq!(result.get(1, 1), 4.0);
    }

    #[test]
    fn test_nmatrix_scalar_multiplication() {
        let m = NMatrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let result = m.multiply_scalar(2.0);
        assert_eq!(result.get(0, 0), 2.0);
        assert_eq!(result.get(0, 1), 4.0);
        assert_eq!(result.get(1, 0), 6.0);
        assert_eq!(result.get(1, 1), 8.0);
    }

    #[test]
    fn test_nmatrix_transpose() {
        let m = NMatrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let result = m.transpose();
        assert_eq!(result.get(0, 0), 1.0);
        assert_eq!(result.get(1, 0), 2.0);
        assert_eq!(result.get(2, 0), 3.0);
        assert_eq!(result.get(0, 1), 4.0);
        assert_eq!(result.get(1, 1), 5.0);
        assert_eq!(result.get(2, 1), 6.0);
    }

    #[test]
    fn test_nmatrix_multiplication() {
        let m1 = NMatrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let m2 = NMatrix::from_vec(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
        let result = m1.multiply_single_thread(&m2);
        assert_eq!(result.get(0, 0), 58.0); // 1*7 + 2*9 + 3*11
        assert_eq!(result.get(0, 1), 64.0); // 1*8 + 2*10 + 3*12
        assert_eq!(result.get(1, 0), 139.0); // 4*7 + 5*9 + 6*11
        assert_eq!(result.get(1, 1), 154.0); // 4*8 + 5*10 + 6*12
    }

    #[test]
    fn test_nmatrix_multiplication_nvector() {
        let m = NMatrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let v = NVector::new(2, vec![5.0, 6.0]);
        let vec = NMatrix::multiply_nvector(&m, &v);
        assert_eq!(vec.x[0], 17.0); // 1*5 + 2*6
        assert_eq!(vec.x[1], 39.0); // 3*5 + 4*6
    }

    #[test]
    fn test_nmatrix_determinant() {
        let m = NMatrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let det = m.determinant_single_thread();
        assert_eq!(det, -2.0);
    }

    #[test]
    fn test_nmatrix_inverse() {
        let m = NMatrix::from_vec(2, 2, vec![4.0, 7.0, 2.0, 6.0]);
        let inv = m.inverse_single_thread();
        let expected = NMatrix::from_vec(2, 2, vec![0.6, -0.7, -0.2, 0.4]);
        for i in 0..2 {
            for j in 0..2 {
                assert!((inv.get(i, j) - expected.get(i, j)).abs() < 1e-6);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_non_invertible_NMatrix() {
        let m = NMatrix::from_vec(2, 2, vec![1.0, 2.0, 2.0, 4.0]);
        m.inverse_single_thread(); // should panic
    }
}
