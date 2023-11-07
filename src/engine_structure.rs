#[derive(Copy, Clone)]
pub struct Point {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Point {
	pub fn d(self, p: Point) -> f32 {
		return f32::sqrt(((self.x - p.x)*(self.x - p.x) + (self.y - p.y)*(self.y - p.y) + (self.z - p.z)*(self.z - p.z)));
	}

	pub fn trans(&mut self, x_: f32, y_: f32, z_: f32) {
		self.x = self.x + x_;
		self.y = self.y + y_;
		self.z = self.z + z_;
	}
    
	pub fn rot(&mut self, x_: f32, y_: f32, z_: f32) {
		
		let cos_x : f32 = f32::cos(x_);
		let cos_y : f32 = f32::cos(y_);
		let cos_z : f32 = f32::cos(z_);
		let sin_x : f32 = f32::sin(x_);
		let sin_y : f32 = f32::sin(y_);
		let sin_z : f32 = f32::sin(z_);
		
		
	    let mut tp = Point { x: self.x, y: self.y, z: self.z };
	
	    self.x = tp.x * (cos_y * cos_z)
	        + tp.y * (sin_x * sin_y * cos_z - cos_x * sin_z)
	        + tp.z * (cos_x * sin_y * cos_z + sin_x * sin_z);
	    self.y = tp.x * (cos_y * sin_z)
	        + tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
	        + tp.z * (cos_x * sin_y * sin_z - sin_x * cos_z);
	    self.z = tp.x * (-sin_y) + tp.y * (sin_x * cos_y) + tp.z * (cos_x * cos_y);
		
	}
}

pub struct Camera {
	pub v: [Point; 3],
	pub zoom: f32,
	pub x: Point,
	pub rx: f32,
	pub ry: f32,
	pub rz: f32,
}

impl Camera {
	 pub fn new(p: Point, rx_: f32, ry_: f32, rz_: f32) -> Self {
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
	pub m: Point,
	rx: f32,
	ry: f32,
	rz: f32,
}

impl Cube {
    pub fn new(p: Point, a: f32) -> Self {
    	let half_a : f32 = a / 2.0;
    
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
            m: p,
            rx: 0.0,
            ry: 0.0,
            rz: 0.0
        }
    }
    
    pub fn rot(&mut self, x_: f32, y_: f32, z_: f32) {
    	let cm : Point = self.m.clone();
    	
    	self.trans(Point{x: -cm.x, y: -cm.y, z: -cm.z});
    
    	self.rx += x_;
    	self.ry += y_;
    	self.rz += z_;
    
    	let cos_x : f32 = f32::cos(x_);
    	let cos_y : f32 = f32::cos(y_);
    	let cos_z : f32 = f32::cos(z_);
    	let sin_x : f32 = f32::sin(x_);
    	let sin_y : f32 = f32::sin(y_);
    	let sin_z : f32 = f32::sin(z_);

	    for i in 0..8 {
	        let mut tp = self.x[i].clone();
	
	        self.x[i].x = tp.x * (cos_y * cos_z)
	            + tp.y * (sin_x * sin_y * cos_z - cos_x * sin_z)
	            + tp.z * (cos_x * sin_y * cos_z + sin_x * sin_z);
	        self.x[i].y = tp.x * (cos_y * sin_z)
	            + tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
	            + tp.z * (cos_x * sin_y * sin_z - sin_x * cos_z);
	        self.x[i].z = tp.x * (-sin_y) + tp.y * (sin_x * cos_y) + tp.z * (cos_x * cos_y);
	    }
	    
	    self.trans(Point{x: cm.x, y: cm.y, z: cm.z});
    }
    
    pub fn trans(&mut self, p: Point) {
    	self.m.trans(p.x, p.y, p.z);
      	
    	for i in 0..8 {
    		self.x[i].trans(p.x, p.y, p.z);
    	}
    }
    
    pub fn has_point(&mut self, p: Point) -> bool {
    	let crx : f32 = self.rx;
    	let cry : f32 = self.ry;
    	let crz : f32 = self.rz;
    	
    	let mut cp : Point = p.clone();
    
    	self.rot(-self.rx, -self.ry, -self.rz);
    	
    	cp.trans(-self.m.x, -self.m.y, -self.m.z);
    	cp.rot(crx, cry, crz);
    	cp.trans(self.m.x, self.m.y, self.m.z);

		/*for i in 0..8 {
	    	println!("{}", self.x[3].x.to_string());
			println!("{}", self.x[3].y.to_string());
			println!("{}", self.x[3].z.to_string());
		}*/
    	
    	let res : bool = (self.x[3].x <= cp.x && self.x[3].y <= cp.y && self.x[3].z <= cp.z && self.x[5].x >= cp.x && self.x[5].y >= cp.y && self.x[5].z >= cp.z);

    	self.rot(crx, crx, crx);
    	
    	return res;//p.d(self.m) < 3.0;
    }
}

