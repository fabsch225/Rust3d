use sdl2::pixels::Color;

use crate::engine::raymarching::RayMarchingObject;
use crate::engine::utils::{rendering::{RenderObjects, Renderable}, transformation::Transformable};
use crate::geometry::point::Point;

#[derive(Copy, Clone, Debug)]
pub struct Quad {
    pub x: [Point; 8],
    s: [Point; 6],
    pub r: f64,
    pub r_outer: f64,
    pub m: Point,
    rx: f64,
    ry: f64,
    rz: f64,
    base_color: Color,
}

impl Quad {
    pub fn new(p: Point, sides: Point, c: Color) -> Self {
        let half_sides: Point = Point {
            x: sides.x / 2.0,
            y: sides.y / 2.0,
            z: sides.z / 2.0,
        };

        let a = sides.clone().norm();

        Quad {
            x: [
                Point {
                    x: p.x - half_sides.x,
                    y: p.y + half_sides.y,
                    z: p.z - half_sides.z,
                },
                Point {
                    x: p.x + half_sides.x,
                    y: p.y + half_sides.y,
                    z: p.z - half_sides.z,
                },
                Point {
                    x: p.x + half_sides.x,
                    y: p.y - half_sides.y,
                    z: p.z - half_sides.z,
                },
                Point {
                    x: p.x - half_sides.x,
                    y: p.y - half_sides.y,
                    z: p.z - half_sides.z,
                },
                Point {
                    x: p.x - half_sides.x,
                    y: p.y + half_sides.y,
                    z: p.z + half_sides.z,
                },
                Point {
                    x: p.x + half_sides.x,
                    y: p.y + half_sides.y,
                    z: p.z + half_sides.z,
                },
                Point {
                    x: p.x + half_sides.x,
                    y: p.y - half_sides.y,
                    z: p.z + half_sides.z
                },
                Point {
                    x: p.x - half_sides.x,
                    y: p.y - half_sides.y,
                    z: p.z + half_sides.z,
                },
            ],
            s: [
                Point {
                    x: p.x - half_sides.x,
                    y: p.y,
                    z: p.z,
                },
                Point {
                    x: p.x + half_sides.x,
                    y: p.y,
                    z: p.z,
                },
                Point {
                    x: p.x,
                    y: p.y - half_sides.y,
                    z: p.z,
                },
                Point {
                    x: p.x,
                    y: p.y + half_sides.y,
                    z: p.z,
                },
                Point {
                    x: p.x,
                    y: p.y,
                    z: p.z - half_sides.z,
                },
                Point {
                    x: p.x,
                    y: p.y,
                    z: p.z + half_sides.z,
                },
            ],
            m: p,
            r: a,
            r_outer: f64::sqrt(a * a * a * 3f64),
            rx: 0.0,
            ry: 0.0,
            rz: 0.0,
            base_color: c,
        }
    }

    pub fn cube(p: Point, a: f64, c: Color) -> Self {
        let half_a: f64 = a / 2.0;

        Quad {
            x: [
                Point {
                    x: p.x - half_a,
                    y: p.y + half_a,
                    z: p.z - half_a,
                },
                Point {
                    x: p.x + half_a,
                    y: p.y + half_a,
                    z: p.z - half_a,
                },
                Point {
                    x: p.x + half_a,
                    y: p.y - half_a,
                    z: p.z - half_a,
                },
                Point {
                    x: p.x - half_a,
                    y: p.y - half_a,
                    z: p.z - half_a,
                },
                Point {
                    x: p.x - half_a,
                    y: p.y + half_a,
                    z: p.z + half_a,
                },
                Point {
                    x: p.x + half_a,
                    y: p.y + half_a,
                    z: p.z + half_a,
                },
                Point {
                    x: p.x + half_a,
                    y: p.y - half_a,
                    z: p.z + half_a,
                },
                Point {
                    x: p.x - half_a,
                    y: p.y - half_a,
                    z: p.z + half_a,
                },
            ],
            s: [
                Point {
                    x: p.x - half_a,
                    y: p.y,
                    z: p.z,
                },
                Point {
                    x: p.x + half_a,
                    y: p.y,
                    z: p.z,
                },
                Point {
                    x: p.x,
                    y: p.y - half_a,
                    z: p.z,
                },
                Point {
                    x: p.x,
                    y: p.y + half_a,
                    z: p.z,
                },
                Point {
                    x: p.x,
                    y: p.y,
                    z: p.z - half_a,
                },
                Point {
                    x: p.x,
                    y: p.y,
                    z: p.z + half_a,
                },
            ],
            m: p,
            r: a,
            r_outer: f64::sqrt(a * a * a * 3f64),
            rx: 0.0,
            ry: 0.0,
            rz: 0.0,
            base_color: c,
        }
    }

    pub fn has_point(self, p: Point) -> u32 {
        if true {
            return 5;
        } else {
            return 0;
        }
    }

    pub fn print_points(&mut self) {
        for i in 0..8 {
            println!("{}", self.x[i].x.to_string());
            println!("{}", self.x[i].y.to_string());
            println!("{}", self.x[i].z.to_string());
        }
    }

    pub fn find_s_index(self, p: Point) -> u32 {
        let mut min_d: Point = self.s[0];
        let mut result: u32 = 1;
        for i in 0..6 {
            if (p.d(min_d) > p.d(self.s[i])) {
                min_d = self.s[i].clone();
                result = i as u32 + 1;
            }
        }
        return result;
    }

    pub fn mins(self) -> [f64; 6] {
        let mut result: [f64; 6] = [
            self.x[3].x,
            self.x[3].y,
            self.x[3].z,
            self.x[5].x,
            self.x[5].y,
            self.x[5].z,
        ];
        for i in 1..8 {
            if (self.x[i].x < result[0]) {
                result[0] = self.x[i].x;
            }
            if (self.x[i].y < result[1]) {
                result[1] = self.x[i].y;
            }
            if (self.x[i].z < result[2]) {
                result[2] = self.x[i].z;
            }
            if (self.x[i].x > result[3]) {
                result[3] = self.x[i].x;
            }
            if (self.x[i].y > result[4]) {
                result[4] = self.x[i].y;
            }
            if (self.x[i].z > result[5]) {
                result[5] = self.x[i].z;
            }
        }
        return result;
    }

    pub fn nearest_point_to(self, p: Point) -> Point {
        let mut pc: Point = p.clone();

        let mut o: Point = self.x[3];
        let mut x: Point = self.x[2];
        let mut y: Point = self.x[0];
        let mut z: Point = self.x[7];

        pc.subtr(o);

        x.subtr(o);
        y.subtr(o);
        z.subtr(o);

        let mut tx: f64 = pc.dt(x) / x.norm_sq();
        let mut ty: f64 = pc.dt(y) / y.norm_sq();
        let mut tz: f64 = pc.dt(z) / z.norm_sq();

        tx = if tx < 0.0 {
				0.0
			} else if tx > 1.0 {
				1.0
			} else {
				tx
			};
        ty = if ty < 0.0 {
				0.0
			} else if ty > 1.0 {
				1.0
			} else {
				ty
			};
        tz = if tz < 0.0 {
				0.0
			} else if tz > 1.0 {
				1.0
			} else {
				tz
			};

        x.mult(tx);
        y.mult(ty);
        z.mult(tz);

        o.add(x);
        o.add(y);
        o.add(z);

        return o;
    }

    pub fn d_(self, p: Point) -> f64 {
        if p.d(self.m) > self.r_outer {
            //doesnt work i think
            //println!("{}", self.r_outer);
            return p.d(self.m);
        } else {
            return self.nearest_point_to(p).d(p);
        }
    }

    pub fn d_rounded(self, p: Point) -> f64 {
        //trash
        let mut pc: Point = p.clone();

        let mut o: Point = self.x[3];
        let mut x: Point = self.x[2];
        let mut y: Point = self.x[0];
        let mut z: Point = self.x[7];

        pc.subtr(o);

        x.subtr(o);
        y.subtr(o);
        z.subtr(o);

        let mut tx: f64 = pc.dt(x) / x.norm_sq();
        let mut ty: f64 = pc.dt(y) / y.norm_sq();
        let mut tz: f64 = pc.dt(z) / z.norm_sq();

        tx = if tx < 0.0 {
            0.0
        } else if tx > 1.0 {
            1.0
        } else {
            tx
        };
        ty = if ty < 0.0 {
            0.0
        } else if ty > 1.0 {
            1.0
        } else {
            ty
        };
        tz = if tz < 0.0 {
            0.0
        } else if tz > 1.0 {
            1.0
        } else {
            tz
        };

        x.mult(tx);
        y.mult(ty);
        z.mult(tz);

        o.add(x);
        o.add(y);
        o.add(z);

        return o.d(p);
    }
}

impl Transformable for Quad {
    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        return Box::new(self);
    }
    fn rot_reverse(&mut self, p: Point) {
        let cm: Point = self.m.clone();

        self.rx -= p.x;
        self.ry -= p.y;
        self.rz -= p.z;

        for i in 0..8 {
            self.x[i].subtr(self.m);
            self.x[i].rot_reverse(p);
            self.x[i].add(self.m);
        }

        for i in 0..6 {
            self.s[i].subtr(self.m);
            self.s[i].rot_reverse(p);
            self.s[i].add(self.m);
        }
    }

    fn rot(&mut self, p: Point) {
        let cm: Point = self.m.clone();

        self.rx += p.x;
        self.ry += p.y;
        self.rz += p.z;

        for i in 0..8 {
            self.x[i].subtr(self.m);
            self.x[i].rot(p);
            self.x[i].add(self.m);
        }

        for i in 0..6 {
            self.s[i].subtr(self.m);
            self.s[i].rot(p);
            self.s[i].add(self.m);
        }
    }

    fn translate(&mut self, p: Point) {
        self.m.trans(p.x, p.y, p.z);

        for i in 0..8 {
            self.x[i].trans(p.x, p.y, p.z);
        }
        for i in 0..6 {
            self.s[i].trans(p.x, p.y, p.z);
        }
    }

    fn scale(&mut self, p: Point) {
        for i in 0..8 {
            self.x[i].subtr(self.m);
            self.x[i].x *= p.x;
            self.x[i].y *= p.y;
            self.x[i].z *= p.z;
            self.x[i].add(self.m);
        }

        for i in 0..6 {
            self.s[i].subtr(self.m);
            self.s[i].x *= p.x;
            self.s[i].y *= p.y;
            self.s[i].z *= p.z;
            self.s[i].add(self.m);
        }
    }
}

impl RayMarchingObject for Quad {
    fn d(&self, p: Point) -> f64 {
        return self.d_(p);
    }

    fn d_r(&self, p: Point) -> f64 {
        return self.d_rounded(p);
    }

    fn color(&self, p: Point) -> Color {
        //let mut color : Point = Point{x: self.base_color.r as f64, y: self.base_color.g as f64, z: self.base_color.b as f64};
        //color.add(Point { x: self.find_s_index(p) as f64 * 10.0, y: (self.find_s_index(p) as f64 * 10.0), z: (self.find_s_index(p) as f64 * 10.0) });

        return self.base_color; //Color::RGB(color.x as u8,  color.y as u8, color.z as u8); // + self.find_s_index(p) * 10
    }

    fn nearest_point(&self, p: Point) -> Point {
        return self.nearest_point_to(p);
    }

    fn clone(&self) -> Box<dyn RayMarchingObject + Send + Sync> {
        return Box::new(Quad {
            x: self.x,
            s: self.s,
            r: self.r,
            r_outer: self.r_outer,
            m: self.m,
            rx: self.rx,
            ry: self.ry,
            rz: self.rz,
            base_color: self.base_color,
        });
    }
}
