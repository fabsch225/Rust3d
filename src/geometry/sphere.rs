use sdl2::pixels::Color;

use crate::engine::raymarching::RayMarchingObject;
use crate::engine::utils::rendering::RaySphereable;
use crate::engine::utils::{rendering::{RayRenderScene, RayRenderable}, transformation::Transformable};
use crate::geometry::vector3::Vector3;

#[derive(Copy, Clone)]
pub struct Sphere {
	m: Vector3,
    r: f64,
    base_color: Color
}

impl Sphere {
    pub fn new(p: Vector3, r_: f64, c: Color) -> Self {
        Sphere {
            m: p,
            r: r_,
            base_color: c
        }
    }

    pub fn d_(self, p : Vector3) -> f64 {
        return self.m.d(p) - self.r;
    }

    pub fn nearest_point_to(self, p : Vector3) -> Vector3 {
        let mut v : Vector3 = self.m.clone();
        let mut res : Vector3 = self.m.clone();
        v.subtract(p);
        v.normalize();
        v.scale(self.r);
        res.add(v);

        return res;
    }
}

impl Transformable for Sphere {
    fn rot_reverse(&mut self, p: Vector3) {}
    fn rot(&mut self, p: Vector3) {}

    fn rot_by(&mut self, p : Vector3, r : Vector3) {
        self.m.rot_by(p, r);
    }

    fn translate(&mut self, p: Vector3) {
    	self.m.translate(p.x, p.y, p.z);
    }

    fn scale(&mut self, p : Vector3) {
        self.r *= p.x;
    }

    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        return Box::new(self);
    }
}

impl RaySphereable for Sphere {
    fn get_radius(&self) -> f64 {
        return self.r;
    }

    fn get_middle(&self) -> Vector3 {
        return self.m.clone();
    }
}

impl RayMarchingObject for Sphere {
	fn d(&self, p : Vector3) -> f64 {
		return self.d_(p);
	}

    fn d_r(&self, p : Vector3) -> f64 {
		return self.d_(p);
	}

	fn color(&self, p : Vector3) -> Color {
		return self.base_color; // + self.find_s_index(p) * 10
	}

    fn nearest_point(&self, p: Vector3) -> Vector3 {
        return self.nearest_point_to(p)
    }

    fn clone(&self) -> Box<dyn RayMarchingObject + Send + Sync> {
        return Box::new(Sphere {
            m: self.m,
            r: self.r,
            base_color: self.base_color
        });
    }
}