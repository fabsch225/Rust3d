use sdl2::pixels::Color as SdlColor;

use crate::engine::{pathtracing::PathtracingObject, raymarching::RayMarchingObject, utils::{rendering::RaySphereable, transformation::Transformable}};
use crate::engine::projection::projection::{Projectable, Projection};
use crate::engine::utils::rendering::Collision;
use crate::engine::utils::virtual_canvas::Color;
use crate::geometry::point::Point;
use crate::math::matrix::MatrixND;
use super::vector3::Vector3;

pub struct Line {
    pub s : Vector3,
    pub e : Vector3,
    pub m : Vector3,
    pub thickness : f64,
    pub base_color : SdlColor,
}

impl Line {
    pub fn new(s : Vector3, e : Vector3, thickness : f64) -> Line {
        let m = Vector3{x: (s.x + e.x) / 2.0, y: (s.y + e.y) / 2.0, z: (s.z + e.z) / 2.0};
        Line {
            s,
            e,
            m,
            thickness,
            base_color: SdlColor::BLUE
        }
    }

    pub fn nearest_point_to_(&self, p : Vector3) -> Vector3 {
        let mut v = self.e.clone();
        let mut pc = p.clone();
        pc.subtract(self.s);
        v.subtract(self.s);
        let mut lin = v.dt(pc) / v.norm_sq();
        lin = if lin < 0.0 {
                0.0
            } else if lin > 1.0 {
                1.0
            } else {
                lin
            };
        v.scale(lin);
        let mut proj = self.s.clone();
        proj.add(v);
        proj
    }

    pub fn d_(&self, p : Vector3) -> f64 {
        p.d(self.nearest_point_to_(p))
    }
}

impl Transformable for Line {
    fn rot_reverse(&mut self, r : Vector3) {
        self.s.rot_reverse_by(self.m, r);
        self.e.rot_reverse_by(self.m, r);
    }
    fn rot(&mut self, r : Vector3) {
        self.s.rot_by(self.m, r);
        self.e.rot_by(self.m, r);
    }
    fn rot_by(&mut self, p : Vector3, r : Vector3) {
        self.s.rot_by(p, r);
        self.e.rot_by(p, r);
        self.m = self.e.clone();
        self.m.add(self.s);
        self.m.scale(0.5);
    }
    fn translate(&mut self, v : Vector3) {
        self.s.add(v);
        self.e.add(v);
        self.m.add(v);
    }
    fn scale(&mut self, v : Vector3) {
        self.s.subtract(self.m);
        self.e.subtract(self.m);
        self.s.x = self.s.x * v.x;
        self.s.y = self.s.y * v.y;
        self.s.z = self.s.z * v.z;
        self.e.x = self.e.x * v.x;
        self.e.y = self.e.y * v.y;
        self.e.z = self.e.z * v.z;
        self.s.add(self.m);
        self.e.add(self.m);
    }

    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        todo!()
    }
}

impl RaySphereable for Line {
    fn get_radius(&self) -> f64 {
        self.s.d(self.m)
    }
    
    fn get_middle(&self) -> Vector3 {
        self.m
    }
}

impl PathtracingObject for Line {
    fn d(&self, p: Vector3) -> f64 {
        todo!()
    }

    fn color(&self, p : Vector3) -> SdlColor {
        todo!()
    }

    fn is_colliding(&mut self, p0 : Vector3, p : Vector3) -> bool {
        todo!()
    }

    fn get_collision(&self, p0 : Vector3, p : Vector3) -> Collision {
        let mut pq = p0.clone();
        pq.subtract(self.s);
        let mut v = self.e.clone();
        v.subtract(self.s);
        let mut n = p.clone();
        n.cross(v);
        let norm_n = n.norm();
        let d = f64::abs(pq.dt(n)) / f64::abs(norm_n);
       
        if (d < self.thickness && self.is_colliding(p0, p) && norm_n > 0.0) {
            let mut d_p0 = self.nearest_point_to_(p0);
            d_p0.subtract(p0);

            Collision {
                d: d_p0.norm(),
                p: self.nearest_point_to_(p0),
                hit: true,
                c: SdlColor::BLUE
            }
        }
        else {
            Collision {
                d,
                p: Vector3::empty(),
                hit: false,
                c: self.base_color
            }
        }
    }

    fn clone(&self) -> Box<dyn PathtracingObject + Send + Sync> {
        //this is necessary so the dynamic trait is preserved
        Box::new(Line {
            s: self.s.clone(),
            e: self.e.clone(),
            m: self.m.clone(),
            thickness: self.thickness,
            base_color: self.base_color
        })
    }
}

impl RayMarchingObject for Line {
    fn d(&self, p: Vector3) -> f64 {
        return self.d_(p);
    }

    fn d_r(&self, p: Vector3) -> f64 {
        return self.d_(p);
    }

    fn color(&self, p: Vector3) -> SdlColor {
        //let mut color : Point = Point{x: self.base_color.r as f64, y: self.base_color.g as f64, z: self.base_color.b as f64};
        //color.add(Point { x: self.find_s_index(p) as f64 * 10.0, y: (self.find_s_index(p) as f64 * 10.0), z: (self.find_s_index(p) as f64 * 10.0) });

        self.base_color //SdlColor::RGB(color.x as u8,  color.y as u8, color.z as u8); // + self.find_s_index(p) * 10
    }

    fn nearest_point(&self, p: Vector3) -> Vector3 {
        return self.nearest_point_to_(p);
    }

    fn clone(&self) -> Box<dyn RayMarchingObject + Send + Sync> {
        //this is necessary so the dynamic trait is preserved
        Box::new(Line {
            s: self.s.clone(),
            e: self.e.clone(),
            m: self.m.clone(),
            thickness: self.thickness,
            base_color: self.base_color
        })
    }
}

impl Projectable for Line {
    fn project(&self, mat: &MatrixND) -> Box<&dyn Projection> {
        todo!()
    }
}