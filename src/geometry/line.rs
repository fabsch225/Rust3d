use sdl2::pixels::Color;

use crate::engine::{pathtracing::PathtracingObject, raymarching::RayMarchingObject, utils::{rendering::Sphereable, transformation::Transformable}};

use super::point::Point as V3;

pub struct Line {
    pub s : V3,
    pub e : V3,
    pub m : V3,
    pub thickness : f64,
    pub base_color : Color,
}

impl Line {
    pub fn new(s : V3, e : V3, thickness : f64) -> Line {
        let m = V3{x: (s.x + e.x) / 2.0, y: (s.y + e.y) / 2.0, z: (s.z + e.z) / 2.0};
        Line {
            s,
            e,
            m,
            thickness,
            base_color: Color::BLUE
        }
    }
    //probalby this is somehow wrong...
    //i have to understand the coordinate system 😭 
    pub fn nearest_point_to_(&self, p : V3) -> V3 {
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

    pub fn d_(&self, p : V3) -> f64 {
        p.d(self.nearest_point_to_(p))
    }
}

impl Transformable for Line {
    fn rot(&mut self, r : V3) {
        self.s.rot_by(self.m, r);
        self.e.rot_by(self.m, r);
    }
    fn rot_reverse(&mut self, r : V3) {
        self.s.rot_reverse_by(self.m, r);
        self.e.rot_reverse_by(self.m, r);
    }
    fn translate(&mut self, v : V3) {
        self.s.add(v);
        self.e.add(v);
        self.m.add(v);
    }
    fn scale(&mut self, v : V3) {
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
    
    fn rot_by(&mut self, p : V3, r : V3) {
        self.s.rot_by(p, r);
        self.e.rot_by(p, r);
        self.m = self.e.clone();
        self.m.add(self.s);
        self.m.scale(0.5);
    }
}

impl Sphereable for Line {
    fn get_radius(&self) -> f64 {
        self.s.d(self.m)
    }
    
    fn get_middle(&self) -> V3 {
        return self.m;
    }
}

impl PathtracingObject for Line {
    fn d(&self, p: V3) -> f64 {
        todo!()
    }

    fn color(&self, p : V3) -> Color {
        todo!()
    }

    fn is_colliding(&mut self, p0 : V3, p : V3) -> bool {
        todo!()
    }

    fn get_collision(&self, p0 : V3, p : V3) -> crate::engine::utils::rendering::Collision {
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

            return crate::engine::utils::rendering::Collision {
                d: d_p0.norm(),
                p: self.nearest_point_to_(p0),
                hit: true,
                c: Color::BLUE
            };
        }
        else {
            return crate::engine::utils::rendering::Collision {
                d,
                p: V3::empty(),
                hit: false,
                c: self.base_color
            };
        }
    }

    fn clone(&self) -> Box<dyn PathtracingObject + Send + Sync> {
        return Box::new(Line {
            s: self.s.clone(),
            e: self.e.clone(),
            m: self.m.clone(),
            thickness: self.thickness,
            base_color: self.base_color
        });
    }
}

impl RayMarchingObject for Line {
    fn d(&self, p: V3) -> f64 {
        return self.d_(p);
    }

    fn d_r(&self, p: V3) -> f64 {
        return self.d_(p);
    }

    fn color(&self, p: V3) -> Color {
        //let mut color : Point = Point{x: self.base_color.r as f64, y: self.base_color.g as f64, z: self.base_color.b as f64};
        //color.add(Point { x: self.find_s_index(p) as f64 * 10.0, y: (self.find_s_index(p) as f64 * 10.0), z: (self.find_s_index(p) as f64 * 10.0) });

        return self.base_color; //Color::RGB(color.x as u8,  color.y as u8, color.z as u8); // + self.find_s_index(p) * 10
    }

    fn nearest_point(&self, p: V3) -> V3 {
        return self.nearest_point_to_(p);
    }

    fn clone(&self) -> Box<dyn RayMarchingObject + Send + Sync> {
        return Box::new(Line {
            s: self.s.clone(),
            e: self.e.clone(),
            m: self.m.clone(),
            thickness: self.thickness,
            base_color: self.base_color
        });
    }
}