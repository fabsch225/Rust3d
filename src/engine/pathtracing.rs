use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::path::Path;
use std::rc::Rc;

use crate::engine::utils::Collision;
use crate::geometry::point::Point as V3;

use super::utils::{Renderable, Transformable};

pub trait PathtracingObject : Transformable {
    fn d(&self, p: V3) -> f64;
    fn color(&self, p : V3) -> Color;
    fn is_colliding(&mut self, p0 : V3, p : V3) -> bool; //Todo
    fn get_collision(&self, p0 : V3, p : V3) -> Collision;
    fn clone(&self) -> Box<dyn PathtracingObject + Send + Sync>;
}

pub struct PathtracingObjects {
    pub objects: Vec<Box<dyn PathtracingObject + Send + Sync>>,
}

impl PathtracingObjects {
    pub fn new() -> Self {
        PathtracingObjects {
            objects: Vec::new(),
        }
    }

    pub fn wrapup(old : &PathtracingObjects) -> Self {
        let mut objects_vec: Vec<Box<dyn PathtracingObject + Send + Sync>> = Vec::new();
        for i in 0..old.objects.len() {
            objects_vec.push(old.objects[i].clone());
        }
        PathtracingObjects {
            objects: objects_vec,
        }
    }

    pub fn get(&mut self, i : usize) -> &mut Box<dyn PathtracingObject + 'static + Send + Sync> {
        return &mut self.objects[i];
    }

    pub fn remove(&mut self, i : usize) {
        self.objects.remove(i);
    }

    pub fn remove_and_clone(&mut self, i : usize) -> Box<dyn PathtracingObject> {
        let obj = self.objects[i].clone();
        self.objects.remove(i);
        obj
    }

    pub fn add(&mut self, obj : impl PathtracingObject + 'static + Send + Sync) {
        self.objects.push(Box::new(obj));
    }
}

impl Renderable for PathtracingObjects {
	fn get_collision(&self, p0 : V3, p : V3, radius : f64) -> Collision {
		let mut c: Collision = Collision::empty();
		let mut bd: f64 = f64::MAX;
		let mut i: usize = 0;
		let mut bg: (f64, f64) = (0.0, 0.0);

		for po in self.objects.iter() {
			let c_ = po.get_collision(p0, p);
			if (c_.hit) {
				if (c_.d < bd) {
					c = c_;
					bd = c_.d;
				}
			}
		}
		return c;
	}
}
