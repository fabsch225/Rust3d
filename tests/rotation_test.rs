#[cfg(test)]
mod tests {
    use rust3d::math::{matrix::NMatrix, vector::NVector};
    use rust3d::math::utils::assertion_utils;
    use rust3d::math::utils::assertion_utils::assert_equal_nvector;

    #[test]
    fn test_rotation_matrix_difference() {
        let a = NVector::from_vec(vec![1.0, 0.0, 0.0]);
        let b = NVector::from_vec(vec![0.0, 1.0, 0.0]);
        let m = NMatrix::rotation_matrix_difference(&a, &b);
        let b_ = m.multiply_nvector(&a);
        assert_equal_nvector(&b, &b_);
        let c = NVector::from_vec(vec![0.0, 0.0, 1.0]);
        let c_ = m.multiply_nvector(&c);
        assert_equal_nvector(&c, &c_);
        let d = NVector::from_vec(vec![0.0, 1.0, 1.0]);
        let e = NVector::from_vec(vec![-1.0, 0.0, 1.0]);
        let e_ = m.multiply_nvector(&d);
        assert_equal_nvector(&e, &e_);
    }
}