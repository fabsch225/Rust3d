#[cfg(test)]
mod tests {
    use rust3d::geometry::point::Point;
    use rust3d::math::vector::NVector;

    fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
        if (a - b).abs() > epsilon {
            panic!("Values {} and {} are not approximately equal", a, b);
        }
    }

    #[test]
    fn test_dot_product() {
        let v1 = NVector{n: 2, x: vec![1., 2.]};
        let v2 = NVector{n: 2, x: vec![3., 2.]};
        let d = v1.dot(&v2);

        assert_approx_eq(d, 7., 0.001);
    }
}