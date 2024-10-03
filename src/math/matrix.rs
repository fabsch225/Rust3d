use super::vector::NVector;

#[derive(Clone, Debug)]
pub struct NMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

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

    pub fn print(&self) {
        println!("===");
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.get(i, j));
            }
            println!();
        }
        println!("===");
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    pub fn transpose(&self) -> Self {
        let mut result = NMatrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        result
    }

    pub fn multiply_single_thread(&self, other: &NMatrix) -> Self {
        assert_eq!(self.cols, other.rows);
        let mut result = NMatrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        result
    }

    pub fn multiply_scalar(&self, scalar: f64) -> Self {
        let mut result = NMatrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(i, j, self.get(i, j) * scalar);
            }
        }
        result
    }

    pub fn multiply_nvector(&self, vec: &NVector) -> NVector {
        assert_eq!(self.cols, vec.n);
        let mut result = NVector::new(self.rows, vec![0.0; self.rows]);
        for i in 0..self.rows {
            let mut sum = 0.0;
            for j in 0..self.cols {
                sum += self.get(i, j) * vec.x[j];
            }
            result.x[i] = sum;
        }
        result
    }

    pub fn add_entry(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] += value;
    }

    pub fn add(&self, other: &NMatrix) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut result = NMatrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(i, j, self.get(i, j) + other.get(i, j));
            }
        }
        result
    }

    pub fn subtract(&self, other: &NMatrix) -> Self {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut result = NMatrix::new(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(i, j, self.get(i, j) - other.get(i, j));
            }
        }
        result
    }

    pub fn get_col_vector(&self, col: usize) -> NVector {
        let mut result = NVector::new(self.rows, vec![0.0; self.rows]);
        for i in 0..self.rows {
            result.set(i, self.get(i, col))
        }
        return result;
    }

    pub fn get_row_vector(&self, row: usize) -> NVector {
        let mut result = NVector::new(self.cols, vec![0.0; self.cols]);
        for i in 0..self.cols {
            result.set(i, self.get(row, i))
        }
        return result;
    }

    pub fn translation_matrix(v: &NVector) -> NMatrix {
        let mut result = NMatrix::identity(v.n + 1);
        for i in 0..v.n {
            result.set(v.n, i, v.get(i));
        }
        result
    }

    //recursive function to find inverse of NMatrix
    pub fn inverse_single_thread(&self) -> Self {
        let mut result = NMatrix::new(self.rows, self.cols);
        let det = self.determinant_single_thread();
        if det == 0.0 {
            panic!("NMatrix is not invertible");
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                let cofactor = self.cofactor(i, j);
                result.set(j, i, cofactor / det);
            }
        }
        result
    }

    pub fn determinant_single_thread(&self) -> f64 {
        assert_eq!(self.rows, self.cols);
        if self.rows == 1 {
            return self.get(0, 0);
        }
        if self.rows == 2 {
            return self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0);
        }
        let mut det = 0.0;
        for j in 0..self.cols {
            det += self.get(0, j) * self.cofactor(0, j);
        }
        det
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let mut result = NMatrix::new(self.rows - 1, self.cols - 1);
        let mut i_prime = 0;
        for i in 0..self.rows {
            if i == row {
                continue;
            }
            let mut j_prime = 0;
            for j in 0..self.cols {
                if j == col {
                    continue;
                }
                result.set(i_prime, j_prime, self.get(i, j));
                j_prime += 1;
            }
            i_prime += 1;
        }
        result.determinant_single_thread()
    }

    //same Matrix, but in homogeneous coordinates
    pub fn expand_to_homogeneous(&mut self) {
        let mut clone = self.clone();
        self.cols += 1;
        self.rows += 1;
        self.data = vec![0.0; self.cols * self.rows];
        for i in 0..clone.rows {
            for j in 0..clone.cols {
                self.set(i, j, clone.get(i, j));
            }
        }

        self.set(self.rows - 1, self.cols - 1, 1.);

    }

    //Givens Rotation
    //https://math.stackexchange.com/questions/1402362/can-rotations-in-4d-be-given-an-explicit-matrix-form
    // pub fn givens(n: usize, i: usize, alpha: f64) -> NMatrix {
    //     panic!("not implemented");
    // }

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
/*
        for i in 0..n {
            for j in 0..n {
                rotation[i, j] += (cos_theta - 1) * (v1_orth[i] * v1_orth[j] + v2_orth[i] * v2_orth[j])

                rotation[i, j] += sin_theta * (v2_orth[i] * v1_orth[j] - v1_orth[i] * v2_orth[j])

            }
        }
*/
        rotation
    }
}