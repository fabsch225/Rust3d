
use std::rc::Rc;
use std::sync::Arc;

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

use crate::sphere::Sphere;
use crate::cube::Cube;
use crate::point::Point as V3;
use crate::face::Face;
use crate::poly_shape::Poly;
use crate::poly_shape::Collision;


pub trait PathtracingObject {
    fn d(&self, p: V3) -> f64;
	fn color(&self, p: V3) -> Color;
	fn rot(&mut self, r: V3); 
	fn trans(&mut self, p: V3); 
	fn scale(&mut self, p: V3);
	fn is_colliding(&mut self, p0: V3, p: V3) -> bool; //Todo
	fn get_collision(&self, p0: V3, p: V3) -> Collision;
}

pub struct PathtracingObjects {
    pub objects: Vec<Box<dyn PathtracingObject + Send + Sync>>
}

impl PathtracingObjects {
	pub fn new() -> Self {
		PathtracingObjects {
			objects: Vec::new()
		}
	}

	pub fn get(&mut self, i: usize) -> &mut Box<dyn PathtracingObject + 'static + Send + Sync>{
		return &mut self.objects[i];
	}

	pub fn add(&mut self, obj: impl PathtracingObject + 'static + Send + Sync) {
		self.objects.push(Box::new(obj));
	}  

	pub fn get_color(&self, p0: V3, p: V3) -> Color {
		for po in self.objects.iter() {
			let c = po.get_collision(p0, p);
			if (c.hit) {
				return c.c;
			}
		}

		return Color::RED;
	}
}


#[derive(Copy, Clone)]
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
    	
        PathtracingCamera {
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

	pub fn draw_section(&self, p: &Vec<Color>, canvas : &mut Canvas<Window>, i1: usize, j1 : usize, i2: usize, j2 : usize) {
		let mut pos : usize = 0;

		for i in i1..i2 {
			for j in j1..j2 {
				let c = p[pos];
				pos = pos + 1;
				canvas.set_draw_color(c);
        
        		canvas.draw_point(Point::new((i) as i32, (j) as i32));
			}
		}	
	}

	pub fn render_pixel_at(&self, j: usize, i : usize, canvas : &mut Canvas<Window>, objs: &PathtracingObjects, w: usize, h : usize,) {
		let vxp : f64 = j as f64 / w as f64;
        let vyp : f64 = i as f64 / h as f64;
        
        let v0 : V3 = self.x;
        let b : V3 = V3{x: self.v[0].x - v0.x, y: self.v[0].y - v0.y, z: self.v[0].z - v0.z};

        let v : V3 = V3{
            x: b.x,
            y: b.y + (self.v[1].y - self.v[0].y) * vyp + (self.v[2].y - self.v[0].y) * vxp,
            z: b.z + (self.v[1].z - self.v[0].z) * vyp + (self.v[2].z - self.v[0].z) * vxp
        };

        
        let mut c = objs.get_color(v0, v);

    
        canvas.set_draw_color(c);
        
        canvas.draw_point(Point::new(j as i32, i as i32));
	}

	pub fn render_section(&self, j1: usize, i1 : usize, j2: usize, i2 : usize, objs: &PathtracingObjects, w: usize, h : usize) -> Vec<Color> {
		let mut section: Vec<Color> = Vec::new();

		for i in i1..i2 {
			for j in j1..j2 {
				let vxp : f64 = j as f64 / w as f64;
				let vyp : f64 = i as f64 / h as f64;
				
				let v0 : V3 = self.x;
				let b : V3 = V3{x: self.v[0].x - v0.x, y: self.v[0].y - v0.y, z: self.v[0].z - v0.z};

				let v : V3 = V3{
					x: b.x,
					y: b.y + (self.v[1].y - self.v[0].y) * vyp + (self.v[2].y - self.v[0].y) * vxp,
					z: b.z + (self.v[1].z - self.v[0].z) * vyp + (self.v[2].z - self.v[0].z) * vxp
				};

				
				section.push(objs.get_color(v0, v).to_owned());
			}
		}

		return section;
	}
}


