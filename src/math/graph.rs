use sdl2::pixels::Color;

use crate::engine::pathtracing::PathtracingObject;
use crate::engine::polytree::poly_tree::PolyTree;
use crate::engine::raymarching::RayMarchingObject;
use crate::engine::utils::anker_label::AnkerLabel;
use crate::engine::utils::{rendering::{RenderObjects, Renderable, Collision, Sphereable}, transformation::Transformable};
use crate::geometry::line::Line;
use crate::geometry::quad::Quad;
use crate::geometry::point::Point as V3;

use super::utils::graph_utils::PolyTreeGraphFactory;

pub struct Graph {
    pub content : Box<dyn PathtracingObject + Send + Sync + 'static>,
    pub bounds : Quad,
    pub color : Color,
    pub axis : Vec<Line>,
    pub grid : Vec<Line>,
    pub labels : Vec<AnkerLabel>,
}

impl Graph {
    pub fn new<T : PolyTreeGraphFactory>(bounds: Quad, f: T) -> Graph {
        let l1 = Line::new(bounds.x[5], bounds.x[2], 0.01);
        let l2 = Line::new(bounds.x[5], bounds.x[1], 0.01);
        let l3 = Line::new(bounds.x[5], bounds.x[4], 0.01);
        Graph {
            content: f.create_graph(bounds, 0.1),
            bounds,
            color: Color::WHITE,
            axis: vec![l1],
            grid: Vec::new(),
            labels: Vec::new(),
        }
    }
}

impl Renderable for Graph {
    fn get_collision(&self, p0: V3, p: V3, radius: f64) -> Collision {
        let mut collisions = vec![
            self.content.get_collision(p0, p)
        ];
        for a in self.axis.iter() {
            collisions.push(a.get_collision(p0, p));
        }
        for a in self.grid.iter() {
            collisions.push(a.get_collision(p0, p));
        }
        
        let mut best_collision = Collision::empty();
        best_collision.d = f64::INFINITY;
        for c in collisions {
            if c.d < best_collision.d && c.hit {
                best_collision = c;
            }
        }

        return best_collision;
    }
}

impl Transformable for Graph {
    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        return Box::new(self);
    }
    fn rot_reverse(&mut self, r_: V3) {
        self.content.rot_reverse(r_);
        for a in self.axis.iter_mut() {
            a.rot_reverse(r_);
        }
        for a in self.grid.iter_mut() {
            a.rot_reverse(r_);
        }
        for a in self.labels.iter_mut() {
            a.rot_reverse(r_);
        }
    }
    fn rot(&mut self, r_: V3) {
        self.content.rot(r_);
        for a in self.axis.iter_mut() {
            a.rot(r_);
        }
        for a in self.grid.iter_mut() {
            a.rot(r_);
        }
        for a in self.labels.iter_mut() {
            a.rot(r_);
        }
    }
    fn translate(&mut self, p: V3) {
        self.content.translate(p);
        for a in self.axis.iter_mut() {
            a.translate(p);
        }
        for a in self.grid.iter_mut() {
            a.translate(p);
        }
        for a in self.labels.iter_mut() {
            a.translate(p);
        }
    }
    fn scale(&mut self, p: V3) {
        self.content.scale(p);
        for a in self.axis.iter_mut() {
            a.scale(p);
        }
        for a in self.grid.iter_mut() {
            a.scale(p);
        }
        for a in self.labels.iter_mut() {
            a.scale(p);
        }
    }
}