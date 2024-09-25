use super::vector::NVector;

#[derive(Clone)]
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
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.get(i, j));
            }
            println!();
        }
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

    pub fn rotation2d(row: usize, col: usize, angle: f64, size: usize) -> NMatrix {
        let mut m_r = NMatrix::identity(size);
    
        for m in 0..size {
            for n in 0..size {
                if m == row && n == row {
                    m_r.set(m, n, angle.cos());
                } else if m == col && n == col {
                    m_r.set(m, n, angle.cos());
                } else if m == row && n == col {
                    m_r.set(m, n, -angle.sin());
                } else if m == col && n == row {
                    m_r.set(m, n, angle.sin());
                } else if m == n && m != row && n != col {
                    m_r.set(m, n, 1.0);
                }
            }
        }

        m_r
    }

    pub fn rotation(angle: &NVector) -> NMatrix {
        let mut m = NMatrix::identity(angle.n);
        let n = angle.n;
        for i in 0..n {
            let r = NMatrix::rotation2d(i, i + 1, angle.x[i], n);
            m = m.multiply_single_thread(&r);
        }

        m
    }

    //http://wscg.zcu.cz/wscg2004/Papers_2004_Short/N29.pdf
    pub fn aquila_perez_single_thread(v_: NMatrix, alpha: f64, n: usize) -> NMatrix {
        panic!("Not implemented");
    }
}