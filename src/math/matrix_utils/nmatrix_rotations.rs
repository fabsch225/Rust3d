use crate::math::matrix::NMatrix;
use crate::math::vector::NVector;

impl NMatrix {

    //Givens Rotation
    //https://math.stackexchange.com/questions/1402362/can-rotations-in-4d-be-given-an-explicit-matrix-form

    pub fn givens_rotation_from_indices(n: usize, a: usize, b: usize, alpha: f64) -> NMatrix {
        let mut result = NMatrix::identity(n);
        result.set(a, a, alpha.cos());
        result.set(a, b, - alpha.sin());
        result.set(b, b, alpha.cos());
        result.set(b, a, alpha.sin());
        result
    }

    //http://wscg.zcu.cz/wscg2004/Papers_2004_Short/N29.pdf
    //https://stackoverflow.com/questions/50337642/how-to-calculate-a-rotation-matrix-in-n-dimensions-given-the-point-to-rotate-an
    /*
    % Implementation of the Aguilera-Perez Algorithm.
    % Aguilera, Antonio, and Ricardo Pérez-Aguila. "General n-dimensional rotations." (2004).
    function M = rotmnd(v,theta)
        n = size(v,1);
        M = eye(n);
        for c = 1:(n-2)
            for r = n:-1:(c+1)
                t = atan2(v(r,c),v(r-1,c));
                R = eye(n);
                R([r r-1],[r r-1]) = [cos(t) -sin(t); sin(t) cos(t)];
                v = R*v;
                M = R*M;
            end
        end
        R = eye(n);
        R([n-1 n],[n-1 n]) = [cos(theta) -sin(theta); sin(theta) cos(theta)];
        M = M\R*M;
     */
    //ToDo cache this!
    pub fn aguilera_perez_single_thread(v: &NMatrix, alpha: f64, n: usize) -> NMatrix {
        let mut v = v.clone();
        assert_eq!(v.rows, n);
        assert_eq!(v.cols, n - 2);

        let mut m = NMatrix::identity(n);

        for c in 1..(n-1) {
            for r in ((c+1)..(n+1)).rev() {
                println!("V: ({}, {})", r, c);
                let theta = f64::atan2(v.get(r - 1, c - 1), v.get(r - 2, c - 1));
                let rm = NMatrix::givens_rotation_from_indices(n, r - 1, r - 2, theta);
                v = rm.multiply_single_thread(&v);
                v.print();
                m = rm.multiply_single_thread(&m);
            }
        }
        let rm = NMatrix::givens_rotation_from_indices(n, n - 2, n - 1, alpha);
        let mi = m.inverse_single_thread();
        m = mi.multiply_single_thread(&rm);
        m = m.multiply_single_thread(&mi);
        m
    }

    pub fn rotation_matrix(a: NVector, b: NVector, theta: f64) -> NMatrix {
        let n = a.n;
        assert_eq!(b.n, n);
        let mut rotation = NMatrix::identity(n);

        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        for i in 0..n {
            for j in 0..n {
                rotation.add_entry(i, j, (cos_theta - 1.0) * (a.get(i) * a.get(j) + b.get(i) * b.get(j)));
                rotation.add_entry(i, j, sin_theta * (b.get(i) * a.get(j) + a.get(i) * b.get(j)));
            }
        }

        rotation
    }

}