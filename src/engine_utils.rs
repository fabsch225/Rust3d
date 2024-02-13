use crate::point::Point as V3;
use crate::face::UV;

use sdl2::pixels::Color;


pub trait Textured {
    fn get_texture(&self) -> Vec<u8>;
    fn get_uv_map(&self) -> Vec<UV>;
}

#[derive(Copy, Clone)]
pub struct Collision {
    pub d: f64,
    pub p : V3,
    pub hit : bool,
    pub c : Color,
}

impl Collision {
    pub fn empty() -> Self {
        Collision {
            d: 0.0,
            p: V3{x: 0.0, y: 0.0, z: 0.0},
            hit: false,
            c: Color::RGB(0,0,0)
        }
    }
}

pub trait Sphereable {
    fn get_radius(&self) -> f64;
    fn get_middle(&self) -> V3;

    fn is_colliding(&self, p0: V3, p: V3) -> bool {
        //is it behind me?
        let mut to_m = self.get_middle().clone();
        to_m.subtr(p0);
        let proj = to_m.dt(p);
        if (proj < 0.0) {
            return false
        }
        else {
            //is it too far away from the ray?
            let mut m_ = self.get_middle().clone();
            m_.subtr(p0);
            m_.cross(p); 
            
            if (m_.norm() > self.get_radius()) {
                return false;
            }
            else {
                return true;
            }
        }
    }
}