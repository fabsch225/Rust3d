use sdl2::pixels::Color as SdlColor;

use crate::engine::{pathtracing::PathtracingObject, raymarching::RayMarchingObject, utils::{rendering::RaySphereable, transformation::Transformable}};
use crate::engine::projection::{Projectable, Projection, Raster};
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
        pc.subtr(self.s);
        v.subtr(self.s);
        let mut lin = v.dt(pc) / v.norm_sq();
        lin = if lin < 0.0 {
                0.0
            } else if lin > 1.0 {
                1.0
            } else {
                lin
            };
        v.mult(lin);
        let mut proj = self.s.clone();
        proj.add(v);
        proj
    }

    pub fn d_(&self, p : Vector3) -> f64 {
        p.d(self.nearest_point_to_(p))
    }
}

impl Transformable for Line {
    fn rot(&mut self, r : Vector3) {
        self.s.rot_by(self.m, r);
        self.e.rot_by(self.m, r);
    }
    fn rot_reverse(&mut self, r : Vector3) {
        self.s.rot_reverse_by(self.m, r);
        self.e.rot_reverse_by(self.m, r);
    }
    fn translate(&mut self, v : Vector3) {
        self.s.add(v);
        self.e.add(v);
        self.m.add(v);
    }
    fn scale(&mut self, v : Vector3) {
        self.s.subtr(self.m);
        self.e.subtr(self.m);
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
    
    fn rot_by(&mut self, p : Vector3, r : Vector3) {
        self.s.rot_by(p, r);
        self.e.rot_by(p, r);
        self.m = self.e.clone();
        self.m.add(self.s);
        self.m.mult(0.5);
    }
}

impl RaySphereable for Line {
    fn get_radius(&self) -> f64 {
        self.s.d(self.m)
    }
    
    fn get_middle(&self) -> Vector3 {
        return self.m;
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

    fn get_collision(&self, p0 : Vector3, p : Vector3) -> crate::engine::utils::rendering::Collision {
        let mut pq = p0.clone();
        pq.subtr(self.s);
        let mut v = self.e.clone();
        v.subtr(self.s);
        let mut n = p.clone();
        n.cross(v);
        let norm_n = n.norm();
        let d = f64::abs(pq.dt(n)) / f64::abs(norm_n);
       
        if (d < self.thickness && self.is_colliding(p0, p) && norm_n > 0.0) {
            let mut d_p0 = self.nearest_point_to_(p0);
            d_p0.subtr(p0);

            return crate::engine::utils::rendering::Collision {
                d: d_p0.norm(),
                p: self.nearest_point_to_(p0),
                hit: true,
                c: SdlColor::BLUE
            };
        }
        else {
            return crate::engine::utils::rendering::Collision {
                d,
                p: Vector3::empty(),
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
    fn d(&self, p: Vector3) -> f64 {
        return self.d_(p);
    }

    fn d_r(&self, p: Vector3) -> f64 {
        return self.d_(p);
    }

    fn color(&self, p: Vector3) -> SdlColor {
        //let mut color : Point = Point{x: self.base_color.r as f64, y: self.base_color.g as f64, z: self.base_color.b as f64};
        //color.add(Point { x: self.find_s_index(p) as f64 * 10.0, y: (self.find_s_index(p) as f64 * 10.0), z: (self.find_s_index(p) as f64 * 10.0) });

        return self.base_color; //SdlColor::RGB(color.x as u8,  color.y as u8, color.z as u8); // + self.find_s_index(p) * 10
    }

    fn nearest_point(&self, p: Vector3) -> Vector3 {
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

pub struct Line2D {
    pub start: Point,
    pub end: Point,
    pub width: f64,
    pub color: Color,
}

impl Line2D {
    pub fn new(start: Point, end: Point, width: f64, color: Color) -> Line2D {
        Line2D {
            start,
            end,
            width,
            color,
        }
    }
    // Helper to convert floating point coordinates to integer grid coordinates
    fn to_grid_coords(&self, width: usize, height: usize) -> ((i32, i32), (i32, i32)) {
        let start_x = self.start.x as i32;
        let start_y = self.start.y as i32;
        let end_x = self.end.x as i32;
        let end_y = self.end.y as i32;
        ((start_x, start_y), (end_x, end_y))
    }

    // Bresenham's line algorithm to rasterize a line
    fn bresenham_line(&self, start: (i32, i32), end: (i32, i32), raster: &mut Raster) {
        let (x0, y0) = start;
        let (x1, y1) = end;

        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x >= 0 && x < raster.width as i32 && y >= 0 && y < raster.pixels.len() as i32 {
                raster.pixels[y as usize][x as usize] = self.color.clone();
            }

            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

impl Projectable for Line2D {
    fn project(&self, mat: &MatrixND) -> Box<&dyn Projection> {
        Box::new(self.clone())
    }
}

impl Projection for Line2D {
       fn rasterize(&self, width: usize, height: usize) -> Raster {
            let (start, end) = self.to_grid_coords(width, height);
            let raster_width = i32::abs(start.0 - end.0) as usize;
            let raster_height = i32::abs(start.1 - end.1) as usize;

            let mut raster = Raster {
                z: 0,
                rec_start: (i32::min(start.0, end.0) as usize, i32::min(start.1, end.1) as usize),
                rec_end: (i32::max(start.0, end.0) as usize, i32::max(start.1, end.1) as usize),
                width,
                height,
                pixels: vec![vec![Color::new(0, 0, 0, 255); raster_width]; raster_height],
            };

            self.bresenham_line((0, 0), (self.end.x as i32 - self.start.x as i32, self.end.y as i32 - self.start.y as i32), &mut raster);

            raster
       }
}

impl Projectable for Line {
    fn project(&self, mat: &MatrixND) -> Box<&dyn Projection> {
        todo!()
    }
}