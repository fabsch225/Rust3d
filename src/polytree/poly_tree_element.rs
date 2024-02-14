use sdl2::pixels::Color;

use crate::engine_pa::PathtracingObject;
use crate::engine_utils::Sphereable;
use crate::face::{Face as F, UV};
use crate::point::Point as V3;
use crate::poly_shape::Poly;
use crate::polytree::poly_tree_utils::PolyTreeCollisionFeedback;

use super::poly_tree::PolyTree;

#[derive(Debug, Clone)]
pub struct PolyTreeElement {
    pub children: Vec<PolyTreeElement>,
    pub faces: Vec<F>,
    pub uvs: Vec<UV>,
    pub m: V3,
    pub radius: f64,
    pub leaf: bool,
}

impl Sphereable for PolyTreeElement {
    fn get_radius(&self) -> f64 {
        return self.radius;
    }

    fn get_middle(&self) -> V3 {
        return self.m;
    }
}

impl PolyTreeElement {
    pub fn empty() -> Self {
        PolyTreeElement {
            children: Vec::new(),
            faces: Vec::new(),
            uvs: Vec::new(),
            m: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 0.0,
            leaf: false,
        }
    }
    pub fn scale_by(&mut self, p: V3, p0: V3) {
        if (self.leaf) {
            for i in 0..self.faces.len() {
                self.faces[i].scale_by(p, p0);
            }
        } else {
            for i in 0..self.children.len() {
                self.children[i].scale_by(p, p0);
            }
            //todo: get radius from chidlren
        }
    }
    pub fn rot(&mut self, r_: V3, p0: V3) {
        self.m.rot_by(p0, r_);
        if (self.leaf) {
           
            for i in 0..self.faces.len() {
                self.faces[i].rot(r_, p0);
            }
        } else {
            for i in 0..self.children.len() {
                self.children[i].rot(r_, p0);
            }
        }
    }
    pub fn rot_reverse(&mut self, r_: V3, p0: V3) {
        if (self.leaf) {
            for i in 0..self.faces.len() {
                self.faces[i].rot_reverse(r_, p0);
            }
            self.m.rot_reverse_by(r_, p0);
        } else {
            for i in 0..self.children.len() {
                self.children[i].rot_reverse(r_, p0);
            }
            self.m.rot_reverse_by(r_, p0);
        }
    }
    pub fn trans(&mut self, p: V3) {
        if (self.leaf) {
            for i in 0..self.faces.len() {
                self.faces[i].trans(p);
            }
            self.m.trans(p.x, p.y, p.z);
        } else {
            for i in 0..self.children.len() {
                self.children[i].trans(p);
            }
            self.m.trans(p.x, p.y, p.z);
        }
    }
    pub fn calulate_middle(&mut self) -> V3 {
        if (self.leaf) {
            let mid = PolyTree::get_middle(&self.faces);
            self.m = mid;
            mid
        } else {
            let mut mid: V3 = V3::empty();
            for i in 0..self.children.len() {
                mid.add(self.children[i].calulate_middle());
            }
            mid.mult(1.0 / self.children.len() as f64);
            self.m = mid;
            mid
        }
    }
    pub fn calculate_radius(&mut self) -> f64 {
        if self.leaf {
            let radius = PolyTree::get_radius(&self.faces);
            self.radius = radius;
            radius
        } else {
            let mut radius = 0.0;
            for i in 0..self.children.len() {
                radius += self.children[i].calculate_radius();
            }
            radius /= self.children.len() as f64;
            self.radius = radius;
            radius
        }
    }
    pub fn get_collision(&self, p0: V3, p: V3) -> Vec<PolyTreeCollisionFeedback> {
        if (self.leaf) {
            let mut bd: f64 = f64::MAX;
            let mut i_: usize = 0;
            let mut pc: V3 = V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let mut bg: (f64, f64) = (0.0, 0.0);
            let mut did_hit = false;

            for i in 0..self.faces.len() {
                if (self.faces[i].is_colliding(p0, p)) {
                    let bg_ = self.faces[i].get_beta_gamma(p0, p);
                    //print!("hit it {} {}", bg_.0, bg_.1);
                    if (bg_.0 <= 1.0
                        && bg_.0 >= 0.0
                        && bg_.1 <= 1.0
                        && bg_.1 >= 0.0
                        && bg_.0 + bg_.1 <= 1.0)
                    {
                        let pc_: V3 = V3 {
                            x: self.faces[i].r.x
                                + bg_.0 * (self.faces[i].a.x - self.faces[i].r.x)
                                + bg_.1 * (self.faces[i].b.x - self.faces[i].r.x),
                            y: self.faces[i].r.y
                                + bg_.0 * (self.faces[i].a.y - self.faces[i].r.y)
                                + bg_.1 * (self.faces[i].b.y - self.faces[i].r.y),
                            z: self.faces[i].r.z
                                + bg_.0 * (self.faces[i].a.z - self.faces[i].r.z)
                                + bg_.1 * (self.faces[i].b.z - self.faces[i].r.z),
                        };
                        let d: f64 = pc_.d(p0);

                        if (d < bd) {
                            pc = pc_;
                            bg = bg_;
                            bd = d;
                            i_ = i;
                            did_hit = true;
                        }
                    }
                }
            }
            if (did_hit) {
                return vec![PolyTreeCollisionFeedback {
                    hit: true,
                    p: pc,
                    uv: &self.uvs[i_],
                    bg,
                }];
            } else {
                return Vec::new();
            }
        } else {
            let mut ptcf_vec: Vec<PolyTreeCollisionFeedback> = Vec::new();
            let mut bd: f64 = f64::MAX;

            for i in 0..8 {
                let pt = &self.children[i];
                if pt.is_colliding(p0, p) {
                    let ptcf_vec1 = pt.get_collision(p0, p);
                    for j in 0..ptcf_vec1.len() {
                        if (ptcf_vec1[j].hit) {
                            ptcf_vec.push(ptcf_vec1[j]);
                        }
                    }
                }
            }

            return ptcf_vec;
        }
    }
}
