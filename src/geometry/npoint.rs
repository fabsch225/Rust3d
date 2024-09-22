pub struct NPoint {
	pub n : usize,
    pub x : Vec<f64>,
}

impl NPoint {
    pub fn new(n_ : usize, x_ : Vec<f64>) -> Self {
        NPoint { 
            n: n_,
            x: x_
        }
    }    

    pub fn new_from(p: &NPoint) -> NPoint {
        NPoint { 
            n: p.n,
            x: p.x.clone()
        }
    }

    pub fn rot(&mut self, r : &NPoint) {
        if self.n < r.n {
            panic!("NPoint::rot: self.n < r.n");
        }

        for i in 0..self.n {
            for j in 0..self.n {
                if i != j {
                    let angle = r.x[j];  // Use the j-th rotation angle

                    let xi = self.x[i];
                    let xj = self.x[j];

                    // Apply 2D rotation matrix for (xi, xj)
                    let new_xi = xi * angle.cos() - xj * angle.sin();
                    let new_xj = xi * angle.sin() + xj * angle.cos();

                    // Update the coordinates with the rotated values
                    self.x[i] = new_xi;
                    self.x[j] = new_xj;
                }
            }
        }
    }

    pub fn rot_reverse(&mut self, r : f64) {
       
    }

    pub fn add(&mut self, p : &NPoint) {
        if self.n < p.n {
            panic!("NPoint::add: self.n < p.n");
        }
        for i in 0..p.n {
            self.x[i] = self.x[i] + p.x[i];
        }
    }

    pub fn subtr(&mut self, p : &NPoint) {
        if self.n < p.n {
            panic!("NPoint::add: self.n < p.n");
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