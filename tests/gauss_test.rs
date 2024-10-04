#[cfg(test)]
mod tests {
    use rust3d::math::matrix::NMatrix;
    use rust3d::math::utils::assertion_utils::assert_equal_nvector;
    use rust3d::math::vector::NVector;

    #[test]
    fn test_2x2_system() {
        let b = NVector::from_vec(vec![5.0, 11.0]);
        let x = NMatrix::from_field(vec![
            vec![2.0, 1.0],
            vec![1.0, 3.0]
        ]);

        let result = NMatrix::gauss_single_thread(&b, &x);
        assert!(result.is_some());
        let b_ = x.multiply_nvector(&result.unwrap());
        assert_equal_nvector(&b, &b_);
    }

    #[test]
    fn test_3x3_system() {
        let b = NVector::from_vec(vec![7.0, -8.0, 6.0]);
        let x = NMatrix::from_field(vec![
            vec![3.0, 2.0, -4.0],
            vec![2.0, 3.0, 3.0],
            vec![5.0, -3.0, 1.0]
        ]);

        let result = NMatrix::gauss_single_thread(&b, &x);
        assert!(result.is_some());
        let b_ = x.multiply_nvector(&result.unwrap());
        assert_equal_nvector(&b, &b_);
    }

    #[test]
    fn test_single_variable() {
        let b = NVector::from_vec(vec![10.0]);
        let x = NMatrix::from_field(vec![
            vec![5.0]
        ]);

        let result = NMatrix::gauss_single_thread(&b, &x);
        assert!(result.is_some());
        let b_ = x.multiply_nvector(&result.unwrap());
        assert_equal_nvector(&b, &b_);
    }

    #[test]
    fn test_singular_matrix() {
        let b = NVector::from_vec(vec![2.0, 4.0]);
        let x = NMatrix::from_field(vec![
            vec![1.0, 2.0],
            vec![2.0, 4.0]
        ]);

        let result = NMatrix::gauss_single_thread(&b, &x);
        assert!(result.is_none());
    }

    #[test]
    fn test_floating_point_precision() {
        let b = NVector::from_vec(vec![0.3333, 1.3333]);
        let x = NMatrix::from_field(vec![
            vec![0.1, 0.2],
            vec![0.3, 0.4]
        ]);

        let result = NMatrix::gauss_single_thread(&b, &x);
        assert!(result.is_some());
        let b_ = x.multiply_nvector(&result.unwrap());
        assert_equal_nvector(&b, &b_);
    }

    #[test]
    fn test_zero_matrix() {
        let b = NVector::from_vec(vec![0.0, 0.0]);
        let x = NMatrix::from_field(vec![
            vec![0.0, 0.0],
            vec![0.0, 0.0]
        ]);

        let result = NMatrix::gauss_single_thread(&b, &x);
        assert!(result.is_none());
    }
}
