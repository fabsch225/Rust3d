use sdl2::pixels::Color;

use std::rc::Rc;

use crate::engine_pa::PathtracingObject;
use crate::face::{Face as F, UV, CollisionCheckable};
use crate::point::Point as V3;
use crate::poly_shape::{Collision, Poly};
use crate::polytree::poly_tree_utils::PolyTreeCollisionFeedback;

pub struct PolyTreeElement<'p> {
    pub children: Vec<PolyTreeElement<'p>>,
    pub faces: &'p Vec<&'p F>,
    pub uvs: &'p Vec<&'p UV>,
    pub m: V3,
    pub radius: f64,
    pub leaf: bool
}

impl CollisionCheckable for PolyTreeElement<'_> {
    fn get_radius(&self) -> f64 {
        return self.radius;
    }

    fn get_middle(&self) -> V3 {
        return self.m;
    }
}

impl PolyTreeElement<'_> {
    pub fn get_collision(&self, p0: V3, p: V3) -> PolyTreeCollisionFeedback {
        if (self.leaf) {
            let mut bd : f64 = f64::MAX; 
            let mut i_ : usize = 0;
            let mut p : V3 = V3{x: 0.0, y: 0.0, z: 0.0};
            let mut bg : (f64, f64) = (0.0, 0.0);

            for i in 0..self.faces.len() {
                if (self.faces[i].is_colliding(p0, p)) {
                    let bg_ = self.faces[i].get_beta_gamma(p0, p); 
                    if (bg_.0 <= 1.0 && bg_.0 >= 0.0 && bg_.1 <= 1.0 && bg_.1 >= 0.0  && bg_.0 + bg_.1 <= 1.0) {
                        let pc: V3 = V3{
                            x: self.faces[i].r.x + bg_.0 * (self.faces[i].a.x - self.faces[i].r.x) + bg_.1 * (self.faces[i].b.x - self.faces[i].r.x), 
                            y: self.faces[i].r.y + bg_.0 * (self.faces[i].a.y - self.faces[i].r.y) + bg_.1 * (self.faces[i].b.y - self.faces[i].r.y), 
                            z: self.faces[i].r.z + bg_.0 * (self.faces[i].a.z - self.faces[i].r.z) + bg_.1 * (self.faces[i].b.z - self.faces[i].r.z)  
                        }; 
                        let d : f64 = pc.d(p0); 

                        if (d < bd) {
                            p = pc;
                            bg = bg_;
                            bd = d; 
                            i_ = i;
                        }
                    }
                }
            }

            return PolyTreeCollisionFeedback{hit: true, p, uv: self.uvs[i_], bg};
        }   
        else {
            let mut ptcf : PolyTreeCollisionFeedback = PolyTreeCollisionFeedback::empty();
            let mut bd : f64 = f64::MAX;

            for i in 0..8 {
                let pt = self.children[i];
                if pt.is_colliding(p0,p) {
                    let ptcf_ = pt.get_collision(p0, p);
                    if (ptcf_.hit) {
                        let d : f64 = ptcf_.p.d(p0);
                        if (d < bd) {
                            bd = d;
                            ptcf = ptcf_;
                        }
                    }
                }
            }

            return ptcf;
        }
    }

    pub fn rot(&mut self, r_: V3, p: V3) {
        if (self.leaf) {
            for i in 0..self.faces.len() {
                self.faces[i].rot(r_, p);
            }
        }
        else {
            for i in 0..self.children.len() {
                self.children[i].rot(r_, p);
            }
        }   
    }
    pub fn rot_reverse(&mut self, r_: V3, p: V3) {
        if (self.leaf) {
            for i in 0..self.faces.len() {
                self.faces[i].rot_reverse(r_, p);
            }
        }
        else {
            for i in 0..self.children.len() {
                self.children[i].rot_reverse(r_, p);
            }
        }   
    }
    pub fn trans(&mut self, p: V3) { 
        if (self.leaf) {
            for i in 0..self.faces.len() {
                self.faces[i].trans(p);
            }
        }
        else {
            for i in 0..self.children.len() {
                self.children[i].trans(p);
            }
        }
    }
}