use std::vec;

use sdl2::pixels::Color;

use crate::engine::pathtracing::{PathtracingObject, PathtracingObjects};
use crate::engine::polytree::poly_tree::PolyTree;
use crate::engine::raymarching::{RayMarchingObject, RayMarchingObjects};
use crate::engine::utils::anker_label::AnkerLabel;
use crate::engine::utils::{rendering::{RenderObjects, Renderable, Collision, Sphereable}, transformation::Transformable};
use crate::geometry::line::Line;
use crate::geometry::quad::Quad;
use crate::geometry::point::Point as V3;

use super::utils::graph_utils::{PolyTreeGraphFactory, WithLabels};

pub struct Graph3D {
    pub content : Box<dyn PathtracingObject + Send + Sync + 'static>,
    pub bounds : Quad,
    pub color : Color,
    pub m : V3,
    pub axis : RayMarchingObjects,
    pub grid : Vec<Line>,
    pub labels : Vec<AnkerLabel>,
}

impl Graph3D {
    pub fn new<T : PolyTreeGraphFactory>(bounds: Quad, f: T, labels : Vec<&str>) -> Graph3D {
        assert_eq!(labels.len(), 3);

        let fg_ = Color::BLUE;
        let bg_ = Color::WHITE;

        let line1 = Line::new(bounds.x[7], bounds.x[6], 0.05);
        let line2 = Line::new(bounds.x[7], bounds.x[4], 0.05);
        let line3 = Line::new(bounds.x[7], bounds.x[3], 0.05);

        let font = include_bytes!("../../demo_assets/fonts/NotoSansMath-Regular.ttf") as &[u8];
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();

        let label1 = AnkerLabel::new(bounds.x[6].x, bounds.x[6].y, bounds.x[6].z, labels[0].to_string(), &font, bg_, fg_);
        let label2 = AnkerLabel::new(bounds.x[4].x, bounds.x[4].y, bounds.x[4].z, labels[1].to_string(), &font, bg_, fg_);
        let label3 = AnkerLabel::new(bounds.x[3].x, bounds.x[3].y, bounds.x[3].z, labels[2].to_string(), &font, bg_, fg_); 
        let mut skel = Vec::new();
        for i in 0..7 {
            for j in 0..7 {
                if j == i {
                    continue;
                }
                skel.push(Line::new(bounds.x[i], bounds.x[j], 0.01));
            }
        }
        let mut axis_ = RayMarchingObjects::new(0.02);
        axis_.add(line1);
        axis_.add(line2);
        axis_.add(line3);
        Graph3D {
            content: f.create_graph(bounds, 0.1),
            bounds,
            m : bounds.m,
            color: Color::WHITE,
            axis: axis_,
            grid: Vec::new(),
            labels: vec![label1, label2, label3],
        }
    }

    pub fn wrapup(old : &PathtracingObjects) -> Self {
        todo!();
    }

}

impl WithLabels for Graph3D {
    fn get_labels(&self) -> &Vec<AnkerLabel> {
        return &self.labels;
    }
}

impl Renderable for Graph3D {
    fn get_collision(&self, p0: V3, p: V3, radius: f64) -> Collision {
        let mut collisions = vec![
            self.content.get_collision(p0, p)
        ];
        
        collisions.push(self.axis.get_collision(p0, p, radius));
        
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

impl Transformable for Graph3D {
    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        return Box::new(self);
    }
    fn rot(&mut self, r_: V3) {
        self.content.rot(r_);
        self.axis.rot(r_); 
        for a in self.grid.iter_mut() {
            a.rot_by(r_, self.m);
        }
        for a in self.labels.iter_mut() {
            a.rot_by(r_, self.m);
        }
    }
    fn translate(&mut self, p: V3) {
        self.content.translate(p);
        self.axis.translate(p);
        for a in self.grid.iter_mut() {
            a.translate(p);
        }
        for a in self.labels.iter_mut() {
            a.translate(p);
        }
    }
    fn scale(&mut self, p: V3) {
        self.content.scale(p);
        self.axis.scale(p);
        for a in self.grid.iter_mut() {
            a.scale(p);
        }
        for a in self.labels.iter_mut() {
            a.scale(p);
        }
    }
}