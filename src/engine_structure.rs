#[derive(Copy, Clone)]
pub struct Point {
	x: f32,
	y: f32,
	z: f32
}

impl Point {
	pub fn trans(&mut self, x_: f32, y_: f32, z_: f32) {
		self.x += x_;
		self.y += y_;
		self.z += z_;
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

struct Camera {
	v: [Point; 3],
	zoom: f32,
	x: Point,
	rx: f32,
	ry: f32,
	rz: f32,
}

impl Camera {
	 pub fn new(p: Point, rx_: f32, ry_: f32, rz_: f32) -> Self {
		let mut v_ : [Point; 3] = [
	    		Point{x: 0.0, y: 0.0, z: 0.0},
	    		Point{x: 0.0, y: 1.0, z: 0.0},
	    		Point{x: 0.0, y: 0.0, z: 1.0}
	    	];

		for mut p in v_ {
			p.rot(rx_, ry_, rz_);
			p.trans(p.x, p.y, p.z);
		}
    	
        Camera {
	        v: [
	    		Point{x: 0.0, y: 0.0, z: 0.0},
	    		Point{x: 0.0, y: 1.0, z: 0.0},
	    		Point{x: 0.0, y: 0.0, z: 1.0}
	    	],
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
	m: Point,
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
    	self.trans(Point{x: -self.m.x, y: -self.m.y, z: -self.m.z});
    
    	self.rx += x_;
    	self.ry += y_;
    	self.rz += z_;
    
    	let cos_x : f32 = f32::cos(x_);
    	let cos_y : f32 = f32::cos(y_);
    	let cos_z : f32 = f32::cos(z_);
    	let sin_x : f32 = f32::sin(x_);
    	let sin_y : f32 = f32::sin(y_);
    	let sin_z : f32 = f32::sin(z_);

	    for p in &mut self.x {
	        let mut tp = Point { x: p.x, y: p.y, z: p.z };
	
	        p.x = tp.x * (cos_y * cos_z)
	            + tp.y * (sin_x * sin_y * cos_z - cos_x * sin_z)
	            + tp.z * (cos_x * sin_y * cos_z + sin_x * sin_z);
	        p.y = tp.x * (cos_y * sin_z)
	            + tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
	            + tp.z * (cos_x * sin_y * sin_z - sin_x * cos_z);
	        p.z = tp.x * (-sin_y) + tp.y * (sin_x * cos_y) + tp.z * (cos_x * cos_y);
	    }
	    
	    self.trans(Point{x: self.m.x, y: self.m.y, z: self.m.z});
    }
    
    pub fn trans(&mut self, p: Point) {
    	for p in &mut self.x {
    		p.trans(p.x, p.y, p.z);
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
    	
    	let res : bool = (self.x[3].x <= cp.x && self.x[3].y <= cp.y && self.x[3].z <= cp.z && self.x[5].x >= cp.x && self.x[5].y >= cp.y && self.x[5].z >= cp.z);

    	self.rot(crx, crx, crx);
    	
    	return res;
    }
}

