use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;

use crate::engine::utils::Collision;
use crate::geometry::sphere::Sphere;
use crate::geometry::cube::Cube;
use crate::geometry::point::Point as V3;
use crate::geometry::face::Face;

use super::utils::Renderable;

pub trait RayMarchingObject {
    fn d(&self, p: V3) -> f64;
	fn d_r(&self, p: V3) -> f64;
	fn color(&self, p: V3) -> Color;
	fn nearest_point(&self, p: V3) -> V3;
	fn rot(&mut self, p: V3);
}

pub struct RayMarchingObjects {
    pub objects: Vec<Box<dyn RayMarchingObject + Send + Sync>>,
	pub epsilon: f64,
}

impl RayMarchingObjects {
	pub fn new() -> Self {
		RayMarchingObjects {
			objects: Vec::new(),
			epsilon: 0.8f64,
		}
	}

	pub fn get(&mut self, i: usize) -> &mut Box<dyn RayMarchingObject + 'static + Send + Sync>{
		return &mut self.objects[i];
	}

	pub fn add(&mut self, obj: impl RayMarchingObject + 'static + Send + Sync) {
		self.objects.push(Box::new(obj));
	}

	pub fn nearest_distance_smoothed(&self, p : V3, epsilon: f64) -> f64 { // generell dumme idee
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

impl Renderable for RayMarchingObjects {
	fn get_collision(&self, p0 : V3, v : V3, radius : f64) -> Collision {
		let mut p : V3 = p0;
		let mut d : f64 = 0.0;
		let mut c = Color::RGB(51, 51, 51);
		loop {
			d = self.nearest_distance(p);
			if (d < self.epsilon) {
				
				c = self.current_color_gradient(p, 10f64);
				break;
			}
			else if (p.d(p0) > radius) {
				c = Color::RGB(51, 51, 51);
				break;
			}
			else {
				p.trans(v.x * d / 2.0, v.y * d / 2.0, v.z * d / 2.0);
			}
		}
		return Collision{d, p, hit: true, c};
	}
}