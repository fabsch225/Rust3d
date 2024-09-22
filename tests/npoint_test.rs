#[cfg(test)]
mod tests {
    use rust3d::geometry::{npoint::NPoint, point::Point};

    fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
        if (a - b).abs() > epsilon {
            panic!("Values {} and {} are not approximately equal", a, b);
        }
    }

    #[test]
    fn test_2d_rotation_90_degrees() {
        let mut point = NPoint { n: 2, x: vec![1.0, 0.0] };
        let rotation = NPoint { n: 2, x: vec![std::f64::consts::FRAC_PI_2, 0.0] }; // Rotate 90 degrees in 2D
        
        point.rot(&rotation);
        
        // Expected point should be (0, 1) after rotating 90 degrees counter-clockwise
        assert_approx_eq(point.x[0], 0.0, 1e-6);
        assert_approx_eq(point.x[1], 1.0, 1e-6);
    }

    #[test]
    fn test_2d_rotation_45_degrees() {
        let mut point = NPoint { n: 2, x: vec![1.0, 0.0] };
        let rotation = NPoint { n: 2, x: vec![std::f64::consts::FRAC_PI_4, 0.0] }; // Rotate 45 degrees in 2D
        
        point.rot(&rotation);
        
        // Expected point should be (sqrt(2)/2, sqrt(2)/2)
        assert_approx_eq(point.x[0], (2.0f64).sqrt() / 2.0, 1e-6);
        assert_approx_eq(point.x[1], (2.0f64).sqrt() / 2.0, 1e-6);
    }

    #[test]
    fn test_2d_full_rotation() {
        let mut point = NPoint { n: 2, x: vec![1.0, 0.0] };
        let rotation = NPoint { n: 2, x: vec![2.0 * std::f64::consts::PI, 0.0] }; // Rotate 360 degrees in 2D
        
        point.rot(&rotation);
        
        // After a full 360-degree rotation, the point should be approximately the same
        assert_approx_eq(point.x[0], 1.0, 1e-6);
        assert_approx_eq(point.x[1], 0.0, 1e-6);
    }

    #[test]
    fn test_3d_rotation_in_first_plane() {
        let mut point = NPoint { n: 3, x: vec![1.0, 0.0, 1.0] };
        let rotation = NPoint { n: 3, x: vec![std::f64::consts::FRAC_PI_2, 0.0, 0.0] }; // Rotate 90 degrees in the first 2 dimensions (x[0], x[1])

        point.rot(&rotation);
        
        // After rotating in the first 2D plane, (x[0], x[1]) should rotate and (x[2]) should remain unchanged
        assert_approx_eq(point.x[0], 0.0, 1e-6);
        assert_approx_eq(point.x[1], 1.0, 1e-6);
        assert_approx_eq(point.x[2], 1.0, 1e-6);
    }

    #[test]
    fn test_3d_rotation_in_second_plane() {
        let mut point = Point { x: 1.0, y: 0.0, z: 1.0 };
        let rotation = Point { x: 0.0, y: std::f64::consts::FRAC_PI_2, z: 0.0 }; // Rotate 90 degrees in the second plane (x[1], x[2])

        point.print();
        point.rot(rotation);
        point.print();

        let mut point = NPoint { n: 3, x: vec![1.0, 0.0, 1.0] };
        let rotation = NPoint { n: 3, x: vec![0.0, std::f64::consts::FRAC_PI_2, 0.0] }; // Rotate 90 degrees in the second plane (x[1], x[2])

        point.print();
        point.rot(&rotation);
        point.print();

        // After rotating in the second plane (x[1], x[2]), x[0] should remain unchanged, (x[1], x[2]) should rotate
        assert_approx_eq(point.x[0], 1.0, 1e-6);
        assert_approx_eq(point.x[1], 0.0, 1e-6);
        assert_approx_eq(point.x[2], -1.0, 1e-6);  // Point rotated 90 degrees around z-axis
    }

    #[test]
    fn test_4d_rotation_in_all_planes() {
        let mut point = NPoint { n: 4, x: vec![1.0, 1.0, 1.0, 1.0] };
        let rotation = NPoint { n: 4, x: vec![std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_4, 0.0] };

        point.rot(&rotation);

        // After rotating in all the planes, the result should be computed for each pair
        assert_approx_eq(point.x[0], 0.0, 1e-6);  // First plane (x[0], x[1]) rotates by 45 degrees
        assert_approx_eq(point.x[1], (2.0f64).sqrt(), 1e-6);  // Rotation in first 2D plane
        // Similar checks can be done for the other pairs
    }

    #[test]
    fn test_no_rotation() {
        let mut point = NPoint { n: 3, x: vec![1.0, 2.0, 3.0] };
        let rotation = NPoint { n: 3, x: vec![0.0, 0.0, 0.0] }; // No rotation
        
        point.rot(&rotation);

        // No rotation should result in the same point
        assert_approx_eq(point.x[0], 1.0, 1e-6);
        assert_approx_eq(point.x[1], 2.0, 1e-6);
        assert_approx_eq(point.x[2], 3.0, 1e-6);
    }

    #[test]
    #[should_panic(expected = "NPoint::rot: self.n < r.n")]
    fn test_dimension_mismatch() {
        let mut point = NPoint { n: 1, x: vec![1.0] };
        let rotation = NPoint { n: 2, x: vec![std::f64::consts::FRAC_PI_2, 0.0] }; // Mismatched dimensions
        
        point.rot(&rotation); // This should panic
    }

    #[test]
    fn test_successive_rotations() {
        let mut point = NPoint { n: 2, x: vec![1.0, 0.0] };
        let rotation1 = NPoint { n: 2, x: vec![std::f64::consts::FRAC_PI_4, 0.0] }; // 45 degrees rotation
        let rotation2 = NPoint { n: 2, x: vec![std::f64::consts::FRAC_PI_4, 0.0] }; // Another 45 degrees rotation

        point.rot(&rotation1); // First 45 degrees
        point.rot(&rotation2); // Another 45 degrees (cumulative to 90 degrees)

        // After successive rotations, the point should be rotated by 90 degrees in total
        assert_approx_eq(point.x[0], 0.0, 1e-6);
        assert_approx_eq(point.x[1], 1.0, 1e-6);
    }
}