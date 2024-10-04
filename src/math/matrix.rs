use super::vector::NVector;

#[derive(Clone, Debug)]
pub struct NMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl NMatrix {
    //ToDo make it nice
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

    pub fn add_entry(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] += value;
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
}