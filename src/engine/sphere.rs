use sdl2::pixels::Color;

use crate::engine::point::Point;
use crate::engine::RayMarchingObject;

#[derive(Copy, Clone)]
pub struct Sphere {
	m: Point,
    r: f64,
    base_color: Color
}

impl Sphere {
    pub fn new(p: Point, r_: f64, c: Color) -> Self {
        Sphere {
            m: p,
            r: r_,
            base_color: c
        }
    }

    pub fn rot_reverse(&mut self, p:Point) {}

    pub fn rot(&mut self, p:Point) {}

    pub fn trans(&mut self, p: Point) {
    	self.m.trans(p.x, p.y, p.z);
    }

    pub fn d_(self, p : Point) -> f64 {
        return self.m.d(p) - self.r;
    }
}

impl RayMarchingObject for Sphere {
	fn d(&self, p : Point) -> f64 {
		return self.d_(p);
	}

    fn d_r(&self, p : Point) -> f64 {
		return self.d_(p);
	}

	fn color(&self, p : Point) -> Color {
		return self.base_color; // + self.find_s_index(p) * 10
	}

	fn rot(&mut self, p : Point) {
		return self.rot(p);
	}
}