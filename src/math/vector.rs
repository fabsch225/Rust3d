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

    pub fn is_null(&self) -> bool {
        for i in 0..self.n {
            if self.get(i).abs() > 1e-5 {
                return false;
            }
        }
        true
    }

    pub fn expand(&mut self) {
        self.x.push(1.);
        self.n += 1;
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

        panic!("NVector::rot: not implemented");
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

    pub fn scale(&mut self, v : f64) {
        for i in 0..self.n {
            self.x[i] = self.x[i] * v;
        }
    }

    pub fn dot(&self, p : &NVector) -> f64 {
        let mut result = 0.;
        for i in 0..self.n {
            result  += self.x[i] * p.x[i];
        }
        result
    }

    pub fn norm(&self) -> f64 {
        let mut result = 0.;
        for i in 0..self.n {
            result += self.x[i] * self.x[i];
        }
        result.sqrt()
    }

    pub fn print(&self) {
        println!("{:?}", self.x);
    }
}