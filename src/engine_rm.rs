

use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

use crate::sphere::Sphere;
use crate::cube::Cube;
use crate::point::Point as V3;
use crate::face::Face;

pub trait RayMarchingObject {
    fn d(&self, p: V3) -> f64;
	fn d_r(&self, p: V3) -> f64;
	fn color(&self, p: V3) -> Color;
	fn nearest_point(&self, p: V3) -> V3;
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

	pub fn nearest_distance_smoothed(&self, p : V3, epsilon: f64) -> f64{ // generell dumme idee
		let trad_d = self.nearest_distance(p);
		
		
		let mut bd : f64 = f64::MAX;
		let mut avg : V3 = V3{x: 0f64, y: 0f64, z: 0f64};
		let mut d = 0.0;
		let mut l: f64 = 0.0;

		for component in self.objects.iter() {
			let cp = component.nearest_point(p);
			avg.add(cp);
			l = l + 1f64; 
		}

		avg.mult(1f64 / l);
		let new_d = p.d(avg);

		if (trad_d < new_d * 0.5) {
			return(trad_d);
		}
		else {
			return(new_d * 1.1);
		}
		
    }

    pub fn nearest_distance(&self, p : V3) -> f64{
		let mut result : f64 = f64::MAX;
		//let mut cd : f64 = 0.0;

        for component in self.objects.iter() {
			let cd = component.d_r(p);
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

		let mut candidates: Vec<Color> = Vec::new();
		let mut vals: Vec<f64> = Vec::new();

        for component in self.objects.iter() {
            cd = component.d(p);
			if (cd < bd) {
				result = component.color(p);
				bd = cd;
			}
        }

		return result;
	}

	pub fn current_color_gradient(&self, p : V3, epsilon: f64) -> Color {
		let mut result : V3 = V3{x: 0.0, y: 0.0, z: 0.0};
		let mut bd : f64 = f64::MAX;
		let mut cd : f64 = 0.0;

		let mut candidates: Vec<Color> = Vec::new();
		let mut vals: Vec<f64> = Vec::new();

		let mut ad = 0.0;

        for component in self.objects.iter() {
            cd = component.d(p);
			if (f64::abs(cd) < epsilon) {
				candidates.push(component.color(p));
				vals.push(cd);
				ad = ad + cd;
			}
        }

		let l : usize = vals.len();

		ad = ad / l as f64;

		for i in 0..vals.len() {
			let c = candidates[i];
			let v = vals[i];

			let mut vc: V3 = V3{x: c.r as f64, y: c.g as f64, z: c.b as f64};
			vc.mult(v / ad);
            result.add(vc);
        }

		result.mult(1.0 / l as f64);

		return Color::RGB(result.x as u8, result.y as u8, result.z as u8);
	}
}

pub struct RayMarchingCamera {
	pub v: [V3; 3],
	pub zoom: f64,
	pub x: V3,
	pub rx: f64,
	pub ry: f64,
	pub rz: f64,
	pub epsilon: f64,
	pub view_distance: f64,
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
            zoom: 1.0,
			epsilon: 0.8f64,
			view_distance: 100.0,
        }
    }

	pub fn rot(&mut self, p : V3) {
		for i in 0..3 {
			self.v[i].rot(V3{x: p.x, y: p.y, z: p.z});
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
        		let mut c = Color::RGB(51, 51, 51); //TODO Base-Color as Attribute of RMC

        		loop {
		            //d = objs.nearest_distance_smoothed(p, self.epsilon * 0.5f64);
					d = objs.nearest_distance(p);
					
		            if (d < self.epsilon) {
		            	//c = objs.current_color(p); // need delta function that exaddertes the edges WRONG!
						c = objs.current_color_gradient(p, 10f64);
		            	break;
		            }
		            else if (p.d(v0) > self.view_distance) {
		            	c = Color::RGB(51, 51, 51);
		            	break;
		            }
		            else {
		            	p.trans(v.x * d / 2.0, v.y * d / 2.0, v.z * d / 2.0);
		            }
        		}

        		canvas.set_draw_color(c);
        		
        		canvas.draw_point(Point::new(j as i32, i as i32));
        	}
        }

		//return canvas;
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
        		let mut d : f64 = 0.0;
        		let mut c = Color::RGB(51, 51, 51); //TODO Base-Color as Attribute of RMC

        		loop {
		            //d = objs.nearest_distance_smoothed(p, self.epsilon * 0.5f64);
					d = objs.nearest_distance(p);
					
		            if (d < self.epsilon) {
		            	//c = objs.current_color(p); // need delta function that exaddertes the edges WRONG!
						c = objs.current_color_gradient(p, 7f64);
		            	break;
		            }
		            else if (p.d(v0) > self.view_distance) {
		            	c = Color::RGB(51, 51, 51);
		            	break;
		            }
		            else {
		            	p.trans(v.x * d / 2.0, v.y * d / 2.0, v.z * d / 2.0);
		            }
        		}

        		canvas.set_draw_color(c);
        		
        		canvas.draw_point(Point::new(j as i32, i as i32));
	}
}


