pub mod cube;
pub mod point;
pub mod face;
pub mod sphere;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

use sphere::Sphere;
use cube::Cube;
use point::Point as V3;
use face::Face;

pub trait PathtracingObject {
    fn d(&self, p: V3) -> f64;
	fn d_r(&self, p: V3) -> f64;
	fn color(&self, p: V3) -> Color;
	fn nearest_point(&self, p: V3) -> V3;
	fn rot(&mut self, p: V3);
}

pub struct PathtracingObjects {
    pub objects: Vec<Box<dyn PathtracingObject>>
}

impl PathtracingObjects {
	pub fn new() -> Self {
		PathtracingObjects {
			objects: Vec::new()
		}
	}

	pub fn get(&mut self, i: usize) -> &mut Box<dyn PathtracingObject + 'static>{
		return &mut self.objects[i];
	}

	pub fn add(&mut self, obj: impl PathtracingObject + 'static) {
		self.objects.push(Box::new(obj));
	}
}

pub struct PathtracingCamera {
	pub v: [V3; 3],
	pub zoom: f64,
	pub x: V3,
	pub rx: f64,
	pub ry: f64,
	pub rz: f64,
	pub epsilon: f64,
	pub view_distance: f64,
}

impl PathtracingCamera {
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
    	
        Camera {
	        v: v_,
        	x: p,
            rx: rx_,
            ry: ry_,
            rz: rz_,
            zoom: 1.0,
			epsilon: 0.8f64,
			view_distance: 100.0,
        }
    }

	pub fn rot(&mut self, p : V3) {
		for i in 0..3 {
			self.v[i].subtr(self.x);
			self.v[i].rot(p);
			self.v[i].add(self.x);
		}
	}


	pub fn render_pixel_at(&self, j: usize, i : usize, canvas : &mut Canvas<Window>, objs: &RayMarchingObjects, w: usize, h : usize,) {
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

        let mut c = Color::RGB(51, 51, 51); //TODO Base-Color as Attribute of RMC

    
        canvas.set_draw_color(c);
        
        canvas.draw_point(Point::new(j as i32, i as i32));
	}
}


