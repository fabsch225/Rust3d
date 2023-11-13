pub mod cube;
pub mod point;
pub mod face;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

use cube::Cube;
use point::Point as V3;
use face::Face;

pub trait RayMarchingObject {
    fn d(&self, p: V3) -> f64;
	fn color(&self, p: V3) -> Color;
	fn rot(&mut self, p: V3);
}

pub struct RayMarchingObjects {
    pub objects: Vec<Box<dyn RayMarchingObject>>
}

impl RayMarchingObjects {
	pub fn new() -> Self {
		RayMarchingObjects {
			objects: Vec::new()
		}
	}

	pub fn get(&mut self, i: usize) -> &mut Box<dyn RayMarchingObject + 'static>{
		return &mut self.objects[i];
	}

	pub fn add(&mut self, obj: impl RayMarchingObject + 'static) {
		self.objects.push(Box::new(obj));
	}

    pub fn nearest_distance(&self, p : V3) -> f64{
		let mut result : f64 = f64::MAX;
		//let mut cd : f64 = 0.0;

        for component in self.objects.iter() {
			let cd = component.d(p);
			if (cd < result) {
				result = cd;
			}
        }

		return result;
    }

	pub fn current_color(&self, p : V3) -> Color {
		let mut result : Color = Color::RGB(0, 0, 0);
		let mut bd : f64 = f64::MAX;
		let mut cd : f64 = 0.0;

        for component in self.objects.iter() {
            cd = component.d(p);
			if (cd < bd) {
				bd = cd;
				result = component.color(p);
			}
        }

		return result;
	}
}

pub struct RayMarchingCamera {
	pub v: [V3; 3],
	pub zoom: f64,
	pub x: V3,
	pub rx: f64,
	pub ry: f64,
	pub rz: f64,
}

impl RayMarchingCamera {
	pub fn new(p: V3, rx_: f64, ry_: f64, rz_: f64) -> Self {
		let mut v_ : [V3; 3] = [
	    		V3{x: 1.0, y: -0.5, z: -0.5},
	    		V3{x: 1.0, y: 0.5, z: -0.5},
	    		V3{x: 1.0, y: -0.5, z: 0.5}
	    	];

		for i in 0..3 {
			v_[i].rot(V3{x: rx_, y: ry_, z: rz_});
			v_[i].trans(p.x, p.y, p.z);
		}
    	
        RayMarchingCamera {
	        v: v_,
        	x: p,
            rx: rx_,
            ry: ry_,
            rz: rz_,
            zoom: 1.0
        }
    }

	pub fn rot(&mut self, p : V3) {
		for i in 0..3 {
			self.v[i].subtr(self.x);
			self.v[i].rot(p);
			self.v[i].add(self.x);
		}
	}

	pub fn render(&self, canvas : &mut Canvas<Window>, objs: &RayMarchingObjects) {
		let (w, h) = canvas.output_size().unwrap();

		for i in 0..h {
        	for j in 0..w {

        		//let p : V3 = V3::new(w as i32, h as i32);
        		
        		let vxp : f64 = j as f64 / w as f64;
        		let vyp : f64 = i as f64 / h as f64;
        		

        		let v0 : V3 = self.x;
				let b : V3 = V3{x: self.v[0].x - v0.x, y: self.v[0].y - v0.y, z: self.v[0].z - v0.z};

        		let v : V3 = V3{
	        		x: b.x,
	        		y: b.y + (self.v[1].y - self.v[0].y) * vyp + (self.v[2].y - self.v[0].y) * vxp,
	        		z: b.z + (self.v[1].z - self.v[0].z) * vyp + (self.v[2].z - self.v[0].z) * vxp
        		};

        		let mut p : V3 = v0;
        		let mut d : f64 = 0.0;
        		let mut last_d : f64 = objs.nearest_distance(v0) + 1.0;
        		let mut c = Color::RGB(0, 0, 0); //TODO Base-Color as Attribute of RMC

        		loop {
		            d = objs.nearest_distance(p);
					
		            if (d < 4.0) { // TODO epsilon 
		            	c = objs.current_color(p);
		            	break;
		            }
		            else if (d > last_d) {
		            	c = Color::RGB(0, 0, 0);
		            	break;
		            }
		            else {
		            	last_d = d;
		            	p.trans(v.x * d / 2.0, v.y * d / 2.0, v.z * d / 2.0);
		            }
        		}

        		canvas.set_draw_color(c);
        		
        		canvas.draw_point(Point::new(j as i32, i as i32));
        	}
        }

		//return canvas;
	}
}



