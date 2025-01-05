use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
use crate::engine::lighting::{Light, Material};
use crate::engine::utils::{rendering::{RayRenderScene, RayRenderable, Collision}, transformation::Transformable};
use crate::geometry::sphere::Sphere;
use crate::geometry::quad::Quad;
use crate::geometry::vector3::Vector3 as V3;
use crate::geometry::face::Face;

pub trait RayMarchingObject : Transformable {
    fn sdf(&self, p: V3) -> f64;
	fn color(&self, p: V3) -> Color;
	fn clone(&self) -> Box<dyn RayMarchingObject + Send + Sync>;
}

pub struct RayMarchingScene {
    pub objects: Vec<Box<dyn RayMarchingObject + Send + Sync>>,
	pub lights: Vec<Light>,
	pub materials: Vec<Material>,
	pub epsilon: f64,
}

impl RayMarchingScene {
	pub fn new(epsilon: f64) -> Self {
		RayMarchingScene {
			objects: Vec::new(),
			materials: Vec::new(),
			lights: Vec::new(),
			epsilon,
		}
	}

	pub fn wrapup(old : &RayMarchingScene) -> Self {
        let mut objects_vec: Vec<Box<dyn RayMarchingObject + Send + Sync>> = Vec::new();
        for i in 0..old.objects.len() {
            objects_vec.push(old.objects[i].clone());
        }
        RayMarchingScene {
			epsilon: old.epsilon,
            objects: objects_vec,
			materials: old.materials.clone(),
			lights: old.lights.clone(),
		}
    }

	pub fn get(&mut self, i: usize) -> &mut Box<dyn RayMarchingObject + 'static + Send + Sync>{
		&mut self.objects[i]
	}

	pub fn add(&mut self, obj: impl RayMarchingObject + 'static + Send + Sync) {
		self.objects.push(Box::new(obj));
	}

	pub fn add_light(&mut self, light: Light) {
		self.lights.push(light);
	}

	pub fn add_material(&mut self, material: Material) {
		self.materials.push(material);
	}

    pub fn nearest_distance(&self, p : V3) -> f64{
		let mut result : f64 = f64::MAX;

        for component in self.objects.iter() {
			let cd = component.sdf(p);
			if (cd < result) {
				result = cd;
			}
        }

		result
    }

	pub fn get_normal(&self, p: V3) -> V3 {
		let e = V3 { x: 0.05, y: 0.0, z: 0.0 };
		let d = self.nearest_distance(p);
		let mut p1 = p.clone();
		p1.add( V3 { x: e.x, y: e.y, z: e.y });
		let mut p2 = p.clone();
		p2.add( V3 { x: e.y, y: e.x, z: e.y });
		let mut p3 = p.clone();
		p3.add( V3 { x: e.y, y: e.y, z: e.x });
		let mut n = V3 {
			x: self.nearest_distance(p1) - d,
			y: self.nearest_distance(p2) - d,
			z: self.nearest_distance(p3) - d,
		};
		n.normalize();
		n
	}

	pub fn diffuse_lighting(&self, p: V3, material: &Material) -> Color {
		let n = self.get_normal(p);
		let mut r = 0.0;
		let mut g = 0.0;
		let mut b = 0.0;

		for light in &self.lights {
			let mut light_dir = light.position.clone();
			light_dir.subtract(p);
			light_dir.normalize();
			let intensity = f64::max(0.0, n.dt(light_dir)) * light.intensity;
			r += light.color.r as f64 * intensity;
			g += light.color.g as f64 * intensity;
			b += light.color.b as f64 * intensity;
		}
		Color::RGB(
			(r * material.diffuse) as u8,
			(g * material.diffuse) as u8,
			(b * material.diffuse) as u8
		)
	}

	pub fn current_color(&self, p: V3) -> Color {
		let mut result_color = Color::RGB(10, 0, 0);
		let mut bd = f64::MAX;

		for (i, component) in self.objects.iter().enumerate() {
			let cd = component.sdf(p);
			if cd < bd {
				let material = &self.materials[i];
				result_color = self.diffuse_lighting(p, material);
				bd = cd;
			}
		}

		result_color
	}
}

impl Transformable for RayMarchingScene {
	fn rot_reverse(&mut self, r_: V3) {
		for component in self.objects.iter_mut() {
			component.rot_reverse(r_);
		}
	}

	fn rot(&mut self, r_: V3) {
		for component in self.objects.iter_mut() {
			component.rot(r_);
		}
	}

	fn rot_by(&mut self, p : V3, r : V3) {
		for component in self.objects.iter_mut() {
			component.rot_by(p, r);
		}
	}

	fn translate(&mut self, p_: V3) {
		for component in self.objects.iter_mut() {
			component.translate(p_);
		}
	}

	fn scale(&mut self, p : V3) {
		todo!()
	}

	fn transform(&mut self) -> Box<&mut dyn Transformable> {
		return Box::new(self);
	}

}

impl RayRenderable for RayMarchingScene {
	fn get_collision(&self, p0 : V3, v : V3, radius : f64) -> Collision {
		let mut p : V3 = p0;
		let mut d : f64 = 0.0;
		let mut c = Color::RGB(51, 51, 51);
		loop {
			d = self.nearest_distance(p);
			if (d < self.epsilon) {
				c = self.current_color(p);
				let d = p.d(p0);
				return Collision{d, p, hit: true, c};
			}
			else if (p.d(p0) > radius) {
				c = Color::RGB(51, 51, 51);
				return Collision{d: 0.0, p, hit: false, c};
			}
			else {
				p.translate(v.x * d / 2.0, v.y * d / 2.0, v.z * d / 2.0);
			}
		}
	}
}