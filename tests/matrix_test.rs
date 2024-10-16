#[cfg(test)]
mod tests {
    use rust3d::math::{matrix::MatrixND, vector::NVector};

    fn assert_nvector_equal(a: NVector, b: NVector) {
        for i in 0..3 {
            assert!((a.get(i) - b.get(i)).abs() < 1e-5);
        }
    }

    #[test]
    fn test_nmatrix_creation() {
        let m = MatrixND::new(3, 3);
        assert_eq!(m.rows, 3);
        assert_eq!(m.cols, 3);
        assert_eq!(m.data.len(), 9);
        for &val in &m.data {
            assert_eq!(val, 0.0);
        }
    }

    #[test]
    fn test_nmatrix_from_vec() {
        let m = MatrixND::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 1), 2.0);
        assert_eq!(m.get(1, 0), 3.0);
        assert_eq!(m.get(1, 1), 4.0);
    }

    #[test]
    fn test_nmatrix_set_and_get() {
        let mut m = MatrixND::new(2, 2);
        m.set(0, 0, 5.0);
        assert_eq!(m.get(0, 0), 5.0);
        m.set(1, 1, 10.0);
        assert_eq!(m.get(1, 1), 10.0);
    }

    #[test]
    fn test_nmatrix_addition() {
        let m1 = MatrixND::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = MatrixND::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let result = m1.add(&m2);
        assert_eq!(result.get(0, 0), 6.0);
        assert_eq!(result.get(0, 1), 8.0);
        assert_eq!(result.get(1, 0), 10.0);
        assert_eq!(result.get(1, 1), 12.0);
    }

    #[test]
    fn test_nmatrix_subtraction() {
        let m1 = MatrixND::from_vec(2, 2, vec![5.0, 6.0, 7.0, 8.0]);
        let m2 = MatrixND::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let result = m1.subtract(&m2);
        assert_eq!(result.get(0, 0), 4.0);
        assert_eq!(result.get(0, 1), 4.0);
        assert_eq!(result.get(1, 0), 4.0);
        assert_eq!(result.get(1, 1), 4.0);
    }

    #[test]
    fn test_nmatrix_scalar_multiplication() {
        let m = MatrixND::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let result = m.multiply_scalar(2.0);
        assert_eq!(result.get(0, 0), 2.0);
        assert_eq!(result.get(0, 1), 4.0);
        assert_eq!(result.get(1, 0), 6.0);
        assert_eq!(result.get(1, 1), 8.0);
    }

    #[test]
    fn test_nmatrix_transpose() {
        let m = MatrixND::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
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
        let m1 = MatrixND::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let m2 = MatrixND::from_vec(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0]);
        let result = m1.multiply_single_thread(&m2);
        assert_eq!(result.get(0, 0), 58.0); // 1*7 + 2*9 + 3*11
        assert_eq!(result.get(0, 1), 64.0); // 1*8 + 2*10 + 3*12
        assert_eq!(result.get(1, 0), 139.0); // 4*7 + 5*9 + 6*11
        assert_eq!(result.get(1, 1), 154.0); // 4*8 + 5*10 + 6*12
    }

    #[test]
    fn test_nmatrix_multiplication_nvector() {
        let m = MatrixND::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let v = NVector::new(2, vec![5.0, 6.0]);
        let vec = MatrixND::multiply_nvector(&m, &v);
        assert_eq!(vec.x[0], 17.0); // 1*5 + 2*6
        assert_eq!(vec.x[1], 39.0); // 3*5 + 4*6
    }

    #[test]
    fn test_nmatrix_determinant() {
        let m = MatrixND::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let det = m.determinant_single_thread();
        assert_eq!(det, -2.0);
    }

    #[test]
    fn test_nmatrix_inverse() {
        let m = MatrixND::from_vec(2, 2, vec![4.0, 7.0, 2.0, 6.0]);
        let inv = m.inverse_single_thread();
        let expected = MatrixND::from_vec(2, 2, vec![0.6, -0.7, -0.2, 0.4]);
        for i in 0..2 {
            for j in 0..2 {
                assert!((inv.get(i, j) - expected.get(i, j)).abs() < 1e-6);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_non_invertible_NMatrix() {
        let m = MatrixND::from_vec(2, 2, vec![1.0, 2.0, 2.0, 4.0]);
        m.inverse_single_thread(); // should panic
    }

    /*
    #[test]
    fn test_rotation_matrix_with_vector() {
        let a = NVector::new(3, vec![1.0, 0.0, 0.0]);
        let b = NVector::new(3, vec![0.0, 1.0, 0.0]);
        let theta = std::f64::consts::FRAC_PI_2;
        let rotation_matrix = NMatrix::rotation_matrix(a.clone(), b.clone(), theta);
        println!("{:?}", rotation_matrix);

        let vector = NVector::new(3, vec![1.0, 0.0, 0.0]);
        let rotated_vector = rotation_matrix.multiply_nvector(&vector);
        let expected = NVector::new(3, vec![0.0, 1.0, 0.0]);
        assert_nvector_equal(rotated_vector, expected);

        let vector = NVector::new(3, vec![0.0, 1.0, 0.0]);
        let rotated_vector = rotation_matrix.multiply_nvector(&vector);
        let expected = NVector::new(3, vec![1.0, 0.0, 0.0]);
        assert_nvector_equal(rotated_vector, expected);

        let vector = NVector::new(3, vec![0.0, 0.0, 1.0]);
        let rotated_vector = rotation_matrix.multiply_nvector(&vector);
        println!("{:?}", rotated_vector);
        let expected = NVector::new(3, vec![0.0, 0.0, 1.0]);
        assert_nvector_equal(rotated_vector, expected);
    }*/
}
