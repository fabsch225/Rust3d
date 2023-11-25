use crate::point::Point as V3;
use crate::face::Face as F;
use crate::engine_pa::PathtracingObject;

use sdl2::pixels::Color;

pub struct Poly {
    pub m : V3,
    pub c : V3,
    pub x : Vec<F>,
    pub base_color: Color, 
    dc: Collision
}

impl Poly {
    pub fn new(m_ : V3, x_ : Vec<F>) -> Self {
        Poly { 
            m: m_,
            x: x_,
            base_color: Color::RGB(0,0,0),
            c: V3{x: 0.0, y: 0.0, z: 0.0},
            dc: Collision{p: m_, hit: false}
        }
    }
}

#[derive(Copy, Clone)]
pub struct Collision {
    pub p : V3,
    pub hit : bool
}

impl PathtracingObject for Poly {
    fn d(&self, p: V3) -> f64 {
        return 0.0; //todo
    }
	fn color(&self, p: V3) -> Color {
        return self.base_color;
    }
	fn rot(&mut self, p: V3) {
        for f in self.x.iter_mut() {
            f.rot(p);
        }
    }
	fn is_colliding(&mut self, p0: V3, p: V3) -> bool {
        return true;
    }

	fn get_collision(&self, p0: V3, p: V3) -> Collision {
        let mut c : Collision = self.dc;
        let mut bd : f64 = f64::MAX; 

        for f in self.x.iter() {
            let cc = f.collides(p0, p); 
            if (cc.hit && cc.p.d(p0) < bd) {
                bd = cc.p.d(p0); 
                c = cc;
            }
        }

        return c;
    }
}