use sdl2::pixels::Color;

use crate::engine::raymarching::RayMarchingObject;
use crate::geometry::point::Point;

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

    pub fn nearest_point_to(self, p : Point) -> Point {
        let mut v : Point = self.m.clone();
        let mut res : Point = self.m.clone();
        v.subtr(p);
        v.normalize();
        v.mult(self.r);
        res.add(v);

        return res;
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

    fn nearest_point(&self, p: Point) -> Point {
        return self.nearest_point_to(p)
    }
}