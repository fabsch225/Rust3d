use super::matrix::NMatrix;

pub struct NVector {
    pub n : usize,
    pub x : Vec<f64>,
}

impl NVector {
    pub fn new(n_ : usize, x_ : Vec<f64>) -> Self {
        NVector { 
            n: n_,
            x: x_
        }
    }    

    pub fn get(&self, i : usize) -> f64 {
        if i >= self.n {
            panic!("NVector::get: i >= self.n");
        }
        self.x[i]
    }

    pub fn set(&mut self, i : usize, v : f64) {
        if i >= self.n {
            panic!("NVector::set: i >= self.n");
        }
        self.x[i] = v;
    }

    pub fn new_from(p: &NVector) -> NVector {
        NVector { 
            n: p.n,
            x: p.x.clone()
        }
    }

    pub fn rot(&mut self, r : &NVector) {
        if self.n < r.n {
            panic!("NVector::rot: self.n < r.n");
        }
        //add zero padding to r
        let mut alpha = NVector::new(self.n, vec![0.0; self.n]);
        for i in 0..r.n {
            alpha.set(i, r.x[i]);
        }

        let m = NMatrix::rotation(&alpha);
        m.print();
        let result = NMatrix::multiply_nvector(&m, self);
        for i in 0..self.n {
            self.x[i] = result.x[i];
        }
    }

    pub fn rot_reverse(&mut self, r : f64) {
       
    }

    pub fn add(&mut self, p : &NVector) {
        if self.n < p.n {
            panic!("NVector::add: self.n < p.n");
        }
        for i in 0..p.n {
            self.x[i] = self.x[i] + p.x[i];
        }
    }

    pub fn subtr(&mut self, p : &NVector) {
        if self.n < p.n {
            panic!("NVector::add: self.n < p.n");
        }
        for i in 0..p.n {
            self.x[i] = self.x[i] - p.x[i];
        }
    }

    pub fn mult(&mut self, v : f64) {
        for i in 0..self.n {
            self.x[i] = self.x[i] * v;
        }
    }

    pub fn print(&self) {
        for i in 0..self.n {
            print!("x{} = {}, ", i, self.x[i]);
        }
        println!();
    }
}