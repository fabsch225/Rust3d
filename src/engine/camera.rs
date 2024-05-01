use std::{borrow::Borrow, sync::{Arc, Mutex}};

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::{geometry::point::Point as V3, math::utils::graph_utils::WithLabels};

use crate::engine::utils::{rendering::{RenderObjects, Renderable}, transformation::Transformable};

use super::utils::{anker_label::AnkerLabel, rendering::Sphereable, renderung_ui::UiElement};

#[derive(Copy, Clone)]
pub struct Camera {
	pub v: [V3; 3],
	pub zoom: f64,
	pub x: V3,
	pub rx: f64,
	pub ry: f64,
	pub rz: f64,
	pub epsilon: f64,
	pub view_distance: f64,
}

impl<'a> Camera {
	pub fn new(p: V3, rx_: f64, ry_: f64, rz_: f64) -> Self {
		let mut v_ : [V3; 3] = [
	    		V3{x: 1.0, y: -0.5, z: -0.5},
	    		V3{x: 1.0, y: 0.5, z: -0.5},
	    		V3{x: 1.0, y: -0.5, z: 0.5}
	    	];

		for i in 0..3 {
			v_[i].rot(V3{x: rx_, y: ry_, z: rz_});
			v_[i].trans(p.x, p.y, p.z);
		}
    	
        Camera {
	        v: v_,
        	x: p,
            rx: rx_,
            ry: ry_,
            rz: rz_,
            zoom: 1.0,
			epsilon: 0.8f64,
			view_distance: 100.0,
        }
    }

	pub fn rot(&mut self, p : V3) {
		for i in 0..3 {
			self.v[i].subtr(self.x);
			self.v[i].rot(p);
			self.v[i].add(self.x);
		}
	}

    fn get_ray_vec(&self, j: usize, i : usize, w: usize, h : usize) -> V3 {
        let vxp : f64 = j as f64 / w as f64;
        let vyp : f64 = i as f64 / h as f64;
        
        let b : V3 = V3{x: self.v[0].x - self.x.x, y: self.v[0].y - self.x.y, z: self.v[0].z - self.x.z};

        let mut v : V3 = V3{
            x: b.x,
            y: b.y + (self.v[1].y - self.v[0].y) * vyp + (self.v[2].y - self.v[0].y) * vxp,
            z: b.z + (self.v[1].z - self.v[0].z) * vyp + (self.v[2].z - self.v[0].z) * vxp
        };

		v.norm();
		v
    }

	pub fn draw_modulus(&self, p: &Vec<Color>, canvas : &mut Canvas<Window>, index: usize, n : usize, w: usize, h : usize) {
		let mut pos : usize = 0;

		for j in 0..w {
			if (j % n == index) {
				for i in 0..h {
					let c = p[pos];
					pos = pos + 1;
					canvas.set_draw_color(c);
			
					canvas.draw_point(Point::new((j) as i32, (i) as i32));
				}
			}
		}
	}

	pub fn render_modulus<R : Renderable>(&self, obj: &R, w: usize, h : usize, index : usize, n : usize) -> Vec<Color> {
		let mut pos : usize = 0;
		let mut pixels: Vec<Color> = Vec::new();

		for j in 0..w {
			if (j % n == index) {
				for i in 0..h {
					let v = self.get_ray_vec(j, i, w, h);
					let c = obj.get_collision(self.x, v, 100.0);
					pixels.push(c.c);
				}
			}
		}

		pixels
	}	

	pub fn render_modulus_multi<R : Renderable>(&self, obj: Arc<R>, w: usize, h : usize, index : usize, n : usize) -> Vec<Color> {
		let mut pos : usize = 0;
		let mut pixels: Vec<Color> = Vec::new();

		for j in 0..w {
			if (j % n == index) {
				for i in 0..h {
					let v = self.get_ray_vec(j, i, w, h);
					let c = obj.get_collision(self.x, v, 100.0);
					pixels.push(c.c);
				}
			}
		}

		pixels
	}	

	pub fn draw_section(&self, p: &Vec<Color>, canvas : &mut Canvas<Window>, i1: usize, j1 : usize, i2: usize, j2 : usize) {
		let mut pos : usize = 0;

		for i in i1..i2 {
			for j in j1..j2 {
				let c = p[pos];
				pos = pos + 1;
				if (c.a != 0) {
				canvas.set_draw_color(c);
        
        		canvas.draw_point(Point::new((i) as i32, (j) as i32));
				}
			}
		}	
	}

	pub fn render_section(&self, j1: usize, i1 : usize, j2: usize, i2 : usize, obj: &dyn Renderable, w: usize, h : usize) -> Vec<Color> {
		let mut section: Vec<Color> = Vec::new();

		for i in i1..i2 {
			for j in j1..j2 {
				let v = self.get_ray_vec(j, i, w, h);
				let c = obj.get_collision(self.x, v, 100.0);
				if (c.hit) {
					section.push(c.c);
				}
				else {
					section.push(Color::RGBA(0, 0, 0, 0));
				}
			}
		}

		return section;
	}

    pub fn render_pixel_at(&self, j: usize, i : usize, canvas : &mut Canvas<Window>, obj: &dyn Renderable, w: usize, h : usize,) {
        let v = self.get_ray_vec(j, i, w, h);
        let mut c = obj.get_collision(self.x, v, 100.0);
    
        canvas.set_draw_color(c.c);
        
        canvas.draw_point(Point::new(j as i32, i as i32));
	}

	pub fn render_anker_labels<W : WithLabels>(&self, a : &W, canvas : &mut Canvas<Window>, w: usize, h : usize) {
		let labels = a.get_labels();

		for l in labels.iter() {
			self.render_anker_label(l, canvas, w, h);
		}
	}

	pub fn render_anker_label(&self, a : &AnkerLabel, canvas : &mut Canvas<Window>, w: usize, h : usize) {
		let mut pos : usize = 0;

		for i in 0..h {
			for j in 0..w {
				let v = self.get_ray_vec(j, i, w, h);
				if (a.is_colliding(self.x, v)) {
					a.render(canvas, i as i32, j as i32);
					return;
				}
			}
		}
	}
}