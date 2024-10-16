use std::cmp;
use crate::math::optimization::float_ops::fast_square_root;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Vector3 {
	pub x: f64,
	pub y: f64,
	pub z: f64
}

impl Vector3 {
	pub fn new(x_: f64, y_: f64, z_: f64) -> Vector3 {
		Vector3 {x: x_, y: y_, z: z_}
	}

	pub fn empty() -> Vector3 {
		Vector3 {x: 0.0, y: 0.0, z: 0.0}
	}

	pub fn normalize(&mut self) {
		let len : f64 = f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
		self.mult( 1f64 / len);
	}

	pub fn norm(self) -> f64 {
		f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
	}

	pub fn norm_sq(self) -> f64 {
		return self.dt(self);
	}

	pub fn dt(self, p: Vector3) -> f64 {
		return self.x * p.x + self.y * p.y + self.z * p.z;
	}

	pub fn d(self, p: Vector3) -> f64 {
		return f64::sqrt(((self.x - p.x)*(self.x - p.x) + (self.y - p.y)*(self.y - p.y) + (self.z - p.z)*(self.z - p.z)));
	}

	//ToDo Remove this and replace uses with Debug trait
	pub fn print(&mut self) {
		println!("{}, {}, {}", self.x.to_string(), self.y.to_string(), self.z.to_string());
	}

	pub fn translate(&mut self, x_: f64, y_: f64, z_: f64) {
		self.x = self.x + x_;
		self.y = self.y + y_;
		self.z = self.z + z_;
	}
	
	pub fn add(&mut self, p : Vector3) {
		self.x = self.x + p.x;
		self.y = self.y + p.y;
		self.z = self.z + p.z;
	}
	
	pub fn subtr(&mut self, p : Vector3) {
		self.x = self.x - p.x;
		self.y = self.y - p.y;
		self.z = self.z - p.z;
	}
	
	pub fn mult(&mut self, x : f64) {
		self.x = self.x * x;
		self.y = self.y * x;
		self.z = self.z * x;
	}
    
	pub fn cross(&mut self, p : Vector3) {
		let x : f64 = self.y * p.z - self.z * p.y;
		let y : f64 = self.z * p.x - self.x * p.z;
		let z : f64 = self.x * p.y - self.y * p.x;
		self.x = x;
		self.y = y;
		self.z = z;
	}

	pub fn rot_by(&mut self, p0 : Vector3, r: Vector3)  {
		self.subtr(p0);
		self.rotate(r);
		self.add(p0);
	}

	pub fn rot_reverse_by(&mut self, p0 : Vector3, r: Vector3)  {
		self.subtr(p0);
		self.rot_reverse(r);
		self.add(p0);
	}

	pub fn rotate(&mut self, p : Vector3) {
		
		let cos_x : f64 = f64::cos(p.x);
		let cos_y : f64 = f64::cos(p.y);
		let cos_z : f64 = f64::cos(p.z);
		let sin_x : f64 = f64::sin(p.x);
		let sin_y : f64 = f64::sin(p.y);
		let sin_z : f64 = f64::sin(p.z);
		
	    let mut tp = Vector3 { x: self.x, y: self.y, z: self.z };
	
	    self.x =   tp.x * (cos_y * cos_z)
	             + tp.y * (sin_x * sin_y * cos_z - cos_x * sin_z)
	             + tp.z * (cos_x * sin_y * cos_z + sin_x * sin_z);
	    self.y =   tp.x * (cos_y * sin_z)
	        	 + tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
	        	 + tp.z * (cos_x * sin_y * sin_z - sin_x * cos_z);
	    self.z =   tp.x * (-sin_y)
				 + tp.y * (sin_x * cos_y)
			     + tp.z * (cos_x * cos_y);
		
	}
	
	pub fn rot_reverse(&mut self, p : Vector3) {
		
		let cos_x : f64 = f64::cos(p.x);
		let cos_y : f64 = f64::cos(p.y);
		let cos_z : f64 = f64::cos(p.z);
		let sin_x : f64 = f64::sin(p.x);
		let sin_y : f64 = f64::sin(p.y);
		let sin_z : f64 = f64::sin(p.z);

	    
        let mut tp = Vector3 { x: self.x, y: self.y, z: self.z };

        self.x =   tp.x * (cos_y * cos_z)
        	     + tp.y * (cos_y * sin_z)
          	     + tp.z * (-sin_y);
		self.y =   tp.x * (sin_x * sin_y * cos_z - cos_x * sin_z)
		 	     + tp.y * (sin_x * sin_y * sin_z + cos_x * cos_z)
		 	     + tp.z * (sin_x * cos_y);
		self.z =   tp.x * (cos_x * sin_y * cos_z + sin_x * sin_z)
		 	     + tp.y * (cos_x * sin_y * sin_z - sin_x * cos_z)
		  	     + tp.z * (cos_x * cos_y);
	}
}