#[cfg(test)]
mod tests {
    use rust3d::geometry::vector3::Vector3;
    use rust3d::math::vector::NVector;

    fn assert_approx_eq(a: f64, b: f64, epsilon: f64) {
        if (a - b).abs() > epsilon {
            panic!("Values {} and {} are not approximately equal", a, b);
        }
    }

    fn vec_approx_equal(v1: &NVector, v2: &NVector, tolerance: f64) -> bool {
        //wtf
        v1.x.iter()
            .zip(v2.x.iter())
            .all(|(a, b)| (a - b).abs() < tolerance)
    }

    #[test]
    fn test_dot_product() {
        let v1 = NVector{n: 2, x: vec![1., 2.]};
        let v2 = NVector{n: 2, x: vec![3., 2.]};
        let d = v1.dot(&v2);

        assert_approx_eq(d, 7., 0.001);
    }

    #[test]
    fn test_gram_schmidt2() {
        let v1 = NVector {
            n: 3,
            x: vec![1.0, 1.0, 1.0],
        };

        let v2 = NVector {
            n: 3,
            x: vec![1.0, 0.0, 1.0],
        };

        let (u1, u2) = NVector::gram_schmidt2(v1.clone(), v2.clone());

        assert_approx_eq(u1.norm(), 1., 0.0001);
        assert_approx_eq(u2.norm(), 1., 0.0001);

        assert_approx_eq(u1.dot(&u2), 0., 0.0001);
    }

    #[test]
    fn test_gram_schmidt_multiple() {
        let v1 = NVector {
            n: 3,
            x: vec![1.0, 1.0, 1.0],
        };

        let v2 = NVector {
            n: 3,
            x: vec![1.0, 0.0, 1.0],
        };

        let v3 = NVector {
            n: 3,
            x: vec![0.0, 1.0, 1.0],
        };

        let orthogonalized = NVector::gram_schmidt(vec![v1.clone(), v2.clone(), v3.clone()]);

        assert_approx_eq(orthogonalized[0].norm(), 1., 0.0001);
        assert_approx_eq(orthogonalized[1].norm(), 1., 0.0001);
        assert_approx_eq(orthogonalized[2].norm(), 1., 0.0001);

        assert_approx_eq(orthogonalized[0].dot(&orthogonalized[1]), 0., 0.0001);
        assert_approx_eq(orthogonalized[2].dot(&orthogonalized[1]), 0., 0.0001);
    }
}