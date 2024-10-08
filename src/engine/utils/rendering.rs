use std::sync::{Arc, RwLock};

use crate::geometry::face::UV;
use crate::geometry::point::Point as V3;

use sdl2::pixels::Color;
use crate::engine::utils::transformation::Transformable;
use crate::engine::pathtracing::PathtracingObject;
use crate::engine::raymarching::RayMarchingObject;

pub trait PathtracingObjectMultiThreading: PathtracingObject + Send + Sync + Transformable {}

pub trait RayMarchingObjectMultiThreading: RayMarchingObject + Send + Sync + Transformable {}

pub trait Textured {
    fn get_texture(&self) -> Vec<u8>;
    fn get_uv_map(&self) -> Vec<UV>;
}

#[derive(Copy, Clone)]
pub struct Collision {
    pub d: f64,
    pub p: V3,
    pub hit: bool,
    pub c: Color,
}

impl Collision {
    pub fn empty() -> Self {
        Collision {
            d: 0.0,
            p: V3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            hit: false,
            c: Color::RGB(0, 0, 0),
        }
    }
}

pub trait Sphereable {
    fn get_radius(&self) -> f64;
    fn get_middle(&self) -> V3;

    //ToDo optimizing this is critical
    fn is_colliding(&self, p0: V3, p: V3) -> bool {
        //is it behind me?
        let mut m = self.get_middle().clone();
        m.subtr(p0);
        let proj = m.dt(p);
        if (proj < 0.0) {
            return false;
        } else {
            //is it too far away from the ray?
            m.cross(p);

            if (m.norm() > self.get_radius()) {
                return false;
            } else {
                return true;
            }
        }
    }
}


pub trait Renderable : Send + Sync {
    fn get_collision(&self, p0: V3, p: V3, radius: f64) -> Collision;
}

pub struct RenderObjects {
    pub objects: Vec<Box<dyn Renderable>>
}

impl RenderObjects {
    pub fn new() -> Self {
        RenderObjects {
            objects: Vec::new()
        }
    }

    pub fn wrap(&mut self, obj: Box<dyn Renderable>) {
        self.objects.push(obj);
    }

    pub fn read(&self) -> &dyn Renderable {
        return self;
    }
}

impl Renderable for RenderObjects {
    fn get_collision(&self, p0: V3, p: V3, radius: f64) -> Collision {
        let mut c: Collision = Collision::empty();
        let mut bd: f64 = f64::MAX;
        let mut i: usize = 0;
        let mut bg: (f64, f64) = (0.0, 0.0);

        for po in self.objects.iter() {
            let c_ = po.get_collision(p0, p, radius);
            if (c_.hit) {
                let mut p2 = c_.p.clone();
                p2.subtr(p0);
                let d = p2.norm();
                if (d != c_.d && c_.hit) {
                    //println!("distance is {}", c_.d);
                }
                //assert_eq!(d, c_.d);
                if (c_.d < bd) {
                    //println!("distance is {}", c_.d);
                    c = c_;
                    bd = c_.d;
                }
            }
        }
        return c;
    }
}
