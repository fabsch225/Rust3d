#[derive(Copy, Clone)]
pub struct Point {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Point {
	pub fn d(self, p: Point) -> f64 {
		return f64::sqrt(((self.x - p.x)*(self.x - p.x) + (self.y - p.y)*(self.y - p.y) + (self.z - p.z)*(self.z - p.z)));
	}

	pub fn trans(&mut self, x_: f64, y_: f64, z_: f64) {
		self.x = self.x + x_;
		self.y = self.y + y_;
		self.z = self.z + z_;
	}
    
	pub fn rot(&mut self, x_: f64, y_: f64, z_: f64) {
		
		let cos_x : f64 = f64::cos(x_);
		let cos_y : f64 = f64::cos(y_);
		let cos_z : f64 = f64::cos(z_);
		let sin_x : f64 = f64::sin(x_);
		let sin_y : f64 = f64::sin(y_);
		let sin_z : f64 = f64::sin(z_);
		
		
	    let mut tp = Point { x: self.x, y: self.y, z: self.z };
	
	    self.x = tp.x * (cos_y * cos_z)
	        + tp.y * (sin_x * sin_y * cos_z - cos_x * sin_z)
	        + tp.z * (cos_x * sin_y * cos_z + sin_x * sin_z);
	    self.y = tp.x * (cos_y * sin_z)
	        + tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
	        + tp.z * (cos_x * sin_y * sin_z - sin_x * cos_z);
	    self.z = tp.x * (-sin_y) + tp.y * (sin_x * cos_y) + tp.z * (cos_x * cos_y);
		
	}
	
	pub fn rot_reverse(&mut self, x_: f64, y_: f64, z_: f64) {

    	let cos_x : f64 = f64::cos(x_);
    	let cos_y : f64 = f64::cos(y_);
    	let cos_z : f64 = f64::cos(z_);
    	let sin_x : f64 = f64::sin(x_);
    	let sin_y : f64 = f64::sin(y_);
    	let sin_z : f64 = f64::sin(z_);

	    
       let mut tp = Point { x: self.x, y: self.y, z: self.z };

        self.x = tp.x * (cos_y * cos_z)
        	+ tp.y * (cos_y * sin_z)
          	+ tp.z * (-sin_y);
		self.y = tp.x * (sin_x * sin_y * cos_z - cos_x * sin_z)
		 	+ tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
		 	+ tp.z * (sin_x * cos_y);
		self.z = tp.x * (cos_x * sin_y * cos_z + sin_x * sin_z)
		 	+ tp.y * (cos_x * sin_y * sin_z - sin_x * cos_z)
		  	+ tp.z * (cos_x * cos_y);
	}
}

pub struct Camera {
	pub v: [Point; 3],
	pub zoom: f64,
	pub x: Point,
	pub rx: f64,
	pub ry: f64,
	pub rz: f64,
}

impl Camera {
	 pub fn new(p: Point, rx_: f64, ry_: f64, rz_: f64) -> Self {
		let mut v_ : [Point; 3] = [
	    		Point{x: 1.0, y: -0.5, z: -0.5},
	    		Point{x: 1.0, y: 0.5, z: -0.5},
	    		Point{x: 1.0, y: -0.5, z: 0.5}
	    	];

		 for i in 0..3 {
			v_[i].rot(rx_, ry_, rz_);
			v_[i].trans(p.x, p.y, p.z);
		}
    	
        Camera {
	        v: v_,
        	x: p,
            rx: rx_,
            ry: ry_,
            rz: rz_,
            zoom: 1.0
        }
    }

}

pub struct Cube {
	x: [Point; 8],
	xc: [Point; 8],
	s: [Point; 6],
	r: f64,
	pub m: Point,
	rx: f64,
	ry: f64,
	rz: f64,
}

impl Cube {
    pub fn new(p: Point, a: f64) -> Self {
    	let half_a : f64 = a / 2.0;
    
        Cube {
        	x: [
		    	Point{x: p.x - half_a, y: p.y + half_a, z: p.z - half_a},
		    	Point{x: p.x + half_a, y: p.y + half_a, z: p.z - half_a},
		 		Point{x: p.x + half_a, y: p.y - half_a, z: p.z - half_a},
		    	Point{x: p.x - half_a, y: p.y - half_a, z: p.z - half_a},
		    	Point{x: p.x - half_a, y: p.y + half_a, z: p.z + half_a},
		    	Point{x: p.x + half_a, y: p.y + half_a, z: p.z + half_a},
		 		Point{x: p.x + half_a, y: p.y - half_a, z: p.z + half_a},
		    	Point{x: p.x - half_a, y: p.y - half_a, z: p.z + half_a}
        	],
        	xc : [
		    	Point{x: p.x - half_a, y: p.y + half_a, z: p.z - half_a},
		    	Point{x: p.x + half_a, y: p.y + half_a, z: p.z - half_a},
		 		Point{x: p.x + half_a, y: p.y - half_a, z: p.z - half_a},
		    	Point{x: p.x - half_a, y: p.y - half_a, z: p.z - half_a},
		    	Point{x: p.x - half_a, y: p.y + half_a, z: p.z + half_a},
		    	Point{x: p.x + half_a, y: p.y + half_a, z: p.z + half_a},
		 		Point{x: p.x + half_a, y: p.y - half_a, z: p.z + half_a},
		    	Point{x: p.x - half_a, y: p.y - half_a, z: p.z + half_a}
        	],
        	s : [
		    	Point{x: p.x - half_a, y: p.y, z: p.z},
		    	Point{x: p.x + half_a, y: p.y , z: p.z},
		 		Point{x: p.x, y: p.y - half_a, z: p.z},
		    	Point{x: p.x, y: p.y + half_a, z: p.z},
		    	Point{x: p.x, y: p.y, z: p.z - half_a},
		    	Point{x: p.x, y: p.y, z: p.z + half_a}
        	],
            m: p,
            r: a,
            rx: 0.0,
            ry: 0.0,
            rz: 0.0
        }
    }
     
	pub fn rot_reverse(&mut self, x_: f64, y_: f64, z_: f64) {
		let cm : Point = self.m.clone();
    	
    	self.trans(Point{x: -cm.x, y: -cm.y, z: -cm.z});
    
    	self.rx -= x_;
    	self.ry -= y_;
    	self.rz -= z_;
    
    	let cos_x : f64 = f64::cos(x_);
    	let cos_y : f64 = f64::cos(y_);
    	let cos_z : f64 = f64::cos(z_);
    	let sin_x : f64 = f64::sin(x_);
    	let sin_y : f64 = f64::sin(y_);
    	let sin_z : f64 = f64::sin(z_);

	    for i in 0..8 {
	        let tp = self.x[i].clone();
	
	        self.x[i].x = tp.x * (cos_y * cos_z)
	        			+ tp.y * (cos_y * sin_z)
	          			+ tp.z * (-sin_y);
			self.x[i].y = tp.x * (sin_x * sin_y * cos_z - cos_x * sin_z)
			 			+ tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
			 			+ tp.z * (sin_x * cos_y);
			self.x[i].z = tp.x * (cos_x * sin_y * cos_z + sin_x * sin_z)
			 			+ tp.y * (cos_x * sin_y * sin_z - sin_x * cos_z)
			  			+ tp.z * (cos_x * cos_y);

	    }
	    
	    self.trans(Point{x: cm.x, y: cm.y, z: cm.z});
	}
    
    pub fn rot(&mut self, x_: f64, y_: f64, z_: f64) {
    	let cm : Point = self.m.clone();
    	
    	self.trans(Point{x: -cm.x, y: -cm.y, z: -cm.z});
    
    	self.rx += x_;
    	self.ry += y_;
    	self.rz += z_;
    
    	let cos_x : f64 = f64::cos(x_);
    	let cos_y : f64 = f64::cos(y_);
    	let cos_z : f64 = f64::cos(z_);
    	let sin_x : f64 = f64::sin(x_);
    	let sin_y : f64 = f64::sin(y_);
    	let sin_z : f64 = f64::sin(z_);

	    for i in 0..8 {
	        let tp = self.x[i].clone();
	
	        self.x[i].x = tp.x * (cos_y * cos_z)
			            + tp.y * (sin_x * sin_y * cos_z - cos_x * sin_z)
			            + tp.z * (cos_x * sin_y * cos_z + sin_x * sin_z);
	        self.x[i].y = tp.x * (cos_y * sin_z)
			            + tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
			            + tp.z * (cos_x * sin_y * sin_z - sin_x * cos_z);
	        self.x[i].z = tp.x * (-sin_y) 
	        			+ tp.y * (sin_x * cos_y) 
	        			+ tp.z * (cos_x * cos_y);
	    }
	    
	    self.trans(Point{x: cm.x, y: cm.y, z: cm.z});
    }
    
    pub fn trans(&mut self, p: Point) {
    	self.m.trans(p.x, p.y, p.z);
      	
    	for i in 0..8 {
    		self.x[i].trans(p.x, p.y, p.z);
    		self.xc[i].trans(p.x, p.y, p.z);
    	}
    }
    
    pub fn has_point(&mut self, p: Point) -> u32 {
    	let mut cp : Point = Point{x:p.x, y:p.y, z:p.z};
    
    	//self.rot_reverse(crx, cry, crz);//reverse rotation
    	
    	if (p.d(self.m) > self.r) {
    		return 0;
    	}

    	let crx : f64 = self.rx;
    	let cry : f64 = self.ry;
    	let crz : f64 = self.rz;
    	
    	cp.trans(-self.m.x, -self.m.y, -self.m.z);
    	cp.rot(crx, cry, crz);
    	cp.trans(self.m.x, self.m.y, self.m.z);

		/*for i in 0..8 {
	    	println!("{}", self.x[3].x.to_string());
			println!("{}", self.x[3].y.to_string());
			println!("{}", self.x[3].z.to_string());
		}*/
    	
    	if (self.xc[3].x <= cp.x && self.xc[3].y <= cp.y && self.xc[3].z <= cp.z && self.xc[5].x >= cp.x && self.xc[5].y >= cp.y && self.xc[5].z >= cp.z) {
			println!("{}", self.find_s_index(cp).to_string());
			return self.find_s_index(cp);

    	}
		else {
			return 0;
		}
    	//self.rot(crx, cry, crz);
    	
    	//return res;//p.d(self.m) < 3.0;
    }
    
    pub fn print_points(&mut self) {
    	for i in 0..8 {
    		println!("{}", self.x[i].x.to_string());
    		println!("{}", self.x[i].y.to_string());
    		println!("{}", self.x[i].z.to_string());
    	}
    }
    
    fn find_s_index(&mut self, p: Point) -> u32 {
    	let mut min_d : Point = Point{x: f64::MAX, y: f64::MAX, z: f64::MAX};
    	let mut result : u32 = 1;
    	for i in 0..6 {
    		if (p.d(min_d) > p.d(self.s[i])) {
    			min_d = self.s[i].clone();
    			result = i as u32 + 1;
    		}
    	}
    	return result;
    }
}

