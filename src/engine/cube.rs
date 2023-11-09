use crate::engine::point::Point;

pub struct Cube {
	x: [Point; 8],
	xc: [Point; 8],
	s: [Point; 6],
	pub r: f64,
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
     
	pub fn rot_reverse(&mut self, p:Point) {
    	let cm : Point = self.m.clone();   	

    	self.rx -= p.x;
    	self.ry -= p.y;
    	self.rz -= p.z;

	    for i in 0..8 {
	    	self.x[i].subtr(self.m);
	        self.x[i].rot_reverse(p);
	        self.x[i].add(self.m);
	    }
	    
	     for i in 0..6 {
	    	self.s[i].subtr(self.m);
	        self.s[i].rot_reverse(p);
	        self.s[i].add(self.m);
	    }
 	}
    
    pub fn rot(&mut self, p:Point) {
    	let cm : Point = self.m.clone();
    	

    	self.rx += p.x;
    	self.ry += p.y;
    	self.rz += p.z;
    

	    for i in 0..8 {
	    	self.x[i].subtr(self.m);
	        self.x[i].rot(p);
	        self.x[i].add(self.m);
	    }
	    
	     for i in 0..6 {
	    	self.s[i].subtr(self.m);
	        self.s[i].rot(p);
	        self.s[i].add(self.m);
	    }
 	}
    
    pub fn trans(&mut self, p: Point) {
    	self.m.trans(p.x, p.y, p.z);
      	
    	for i in 0..8 {
    		self.x[i].trans(p.x, p.y, p.z);
    		self.xc[i].trans(p.x, p.y, p.z);
    	}
    	for i in 0..6 {
    		self.s[i].trans(p.x, p.y, p.z);
    	}
    }
    
    pub fn has_point(&mut self, p: Point) -> u32 {
    	
    	
    	if (true) {
			return 5;//return self.find_s_index(cp);//
    	}
		else {
			return 0;
		}
		
    	

    }
    
    pub fn print_points(&mut self) {
    	for i in 0..8 {
    		println!("{}", self.x[i].x.to_string());
    		println!("{}", self.x[i].y.to_string());
    		println!("{}", self.x[i].z.to_string());
    	}
    }
    
    pub fn find_s_index(&mut self, p: Point) -> u32 {
    	let mut min_d : Point = self.s[0];
    	let mut result : u32 = 1;
    	for i in 0..6 {
    		if (p.d(min_d) > p.d(self.s[i])) {
    			min_d = self.s[i].clone();
    			result = i as u32 + 1;
    		}
    	}
    	return result;
    }
    
    fn mins(&mut self) -> [f64; 6] {
    	let mut result : [f64; 6] = [self.xc[3].x, self.xc[3].y, self.xc[3].z, self.xc[5].x, self.xc[5].y, self.xc[5].z];
 		for i in 1..8 {
 			if (self.x[i].x < result[0]) {
 				result[0] = self.x[i].x;
 			}
 			if (self.x[i].y < result[1]) {
 				result[1] = self.x[i].y;
 			}
 			if (self.x[i].z < result[2]) {
 				result[2] = self.x[i].z;
 			}
 			
 			if (self.x[i].x > result[3]) {
 				result[3] = self.x[i].x;
 			}
 			if (self.x[i].y > result[4]) {
 				result[4] = self.x[i].y;
 			}
 			if (self.x[i].z > result[5]) {
 				result[5] = self.x[i].z;
 			}
 		}
    	return result;
    }
    
    
    pub fn d(&mut self, p : Point) -> f64 {
    	let mut pc : Point = p.clone();

    	let mut o : Point = self.x[3];
    	let mut x : Point = self.x[2];
    	let mut y : Point = self.x[0];
    	let mut z : Point = self.x[7];
    	
    	/*println!("{},  {},  {}", pc.x, pc.y, pc.z);
    	
    	println!("start");
    	
    	println!("{},  {},  {}", o.x, o.y, o.z);
    	println!("{},  {},  {}", x.x, x.y, x.z);
    	println!("{},  {},  {}", y.x, y.y, y.z);
    	println!("{},  {},  {}", z.x, z.y, z.z);*/
    	
    	pc.subtr(o);
    	
    	x.subtr(o);
    	y.subtr(o);
    	z.subtr(o);
    	
    	let mut tx : f64 = pc.dt(x) / x.norm();
    	let mut ty : f64 = pc.dt(y) / y.norm();
    	let mut tz : f64 = pc.dt(z) / z.norm();
    	
    	//println!("{},  {},  {}", tx, ty, tz);
    	
    	tx = if tx < 0.0 { 0.0 } else if tx > 1.0 { 1.0 } else { tx };
		ty = if ty < 0.0 { 0.0 } else if ty > 1.0 { 1.0 } else { ty };
		tz = if tz < 0.0 { 0.0 } else if tz > 1.0 { 1.0 } else { tz };
    	
    	/*println!("dirs before mult");
    	
    	println!("{},  {},  {}", o.x, o.y, o.z);
    	println!("{},  {},  {}", x.x, x.y, x.z);
    	println!("{},  {},  {}", y.x, y.y, y.z);
    	println!("{},  {},  {}", z.x, z.y, z.z);*/
    	
    	x.mult(tx);
    	y.mult(ty);
    	z.mult(tz);
    	
    	/*println!("dirs after mult");
    	
    	println!("{},  {},  {}", x.x, x.y, x.z);
    	println!("{},  {},  {}", y.x, y.y, y.z);
    	println!("{},  {},  {}", z.x, z.y, z.z);
    	
    	println!("origin before addition");
    	
    	println!("{},  {},  {}", o.x, o.y, o.z);*/
    	
    	o.add(x);
    	o.add(y);
    	o.add(z);
    	//o.print();
    	
    	return o.d(p);
    }
}
