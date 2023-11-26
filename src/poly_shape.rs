use std::fs::{self, File};
use std::io::{BufReader, BufRead};

use rand::seq::SliceRandom;

use crate::point::Point as V3;
use crate::face::Face as F;
use crate::engine_pa::PathtracingObject;

use sdl2::pixels::Color;

pub struct Poly {
    pub m : V3,
    pub x : Vec<F>,
    pub base_color: Color, 
    dc: Collision
}

impl Poly {
    pub fn new(m_ : V3, x_ : Vec<F>) -> Self {
        Poly { 
            m: m_,
            x: x_,
            base_color: Color::RGB(0,0,0),
            dc: Collision{p: m_, hit: false, c: Color::BLUE}
        }
    }  

    pub fn parse_wavefront(f: String) -> Self { 
        let mut vertices : Vec<V3> = Vec::new();
        let mut middle : V3 = V3{x: 0.0, y: 0.0, z: 0.0};
        let mut faces : Vec<F> = Vec::new();

        let reader = BufReader::new(File::open(f).expect("Cannot open file"));
         
        for line in reader.lines() { 
            let mut s = line.expect("");
            let ss = s.split_off(2);
            let mut es = ss.split_whitespace(); 

            if (s.eq("v ")) {
                let x_: f64 = es.next().expect("").parse::<>().unwrap();
                let y_: f64 = es.next().expect("").parse::<>().unwrap();
                let z_: f64 = es.next().expect("").parse::<>().unwrap();
 
                println!("V {}, {}, {} ", x_, y_, z_);

                vertices.push(V3{x: x_, y: y_, z: z_});
                middle.add(V3{x: x_, y: y_, z: z_});
            }
            else if (s.eq("f ")) {
                let ri : usize = es.next().expect("").split("/").next().expect("").parse::<usize>().unwrap() - 1;
                let ai : usize = es.next().expect("").split("/").next().expect("").parse::<usize>().unwrap() - 1;
                let bi : usize = es.next().expect("").split("/").next().expect("").parse::<usize>().unwrap() - 1;

                faces.push(F{r: vertices[ri], a: vertices[ai], b: vertices[bi]});
            }
        }

        middle.mult(1.0 / vertices.len() as f64);

        return Poly{m: middle, x: faces, base_color: Poly::random_color(), dc: Collision{p: middle, hit: false, c: Color::BLUE}};
    }

    pub fn random_color() -> Color {
        let cv = vec![Color::CYAN, Color::BLUE, Color::GREEN, Color::GRAY, Color::YELLOW]; 
        return *cv.choose(&mut rand::thread_rng()).unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct Collision {
    pub p : V3,
    pub hit : bool,
    pub c : Color,
}

impl PathtracingObject for Poly {
    fn d(&self, p: V3) -> f64 {
        return 0.0; //todo
    }
	fn color(&self, p: V3) -> Color {
        return self.base_color;
    }
	fn rot(&mut self, p: V3) {
        for f in self.x.iter_mut() {
            f.rot(p, self.m);
        }
    }
	fn is_colliding(&mut self, p0: V3, p: V3) -> bool {
        return true;
    }

	fn get_collision(&self, p0: V3, p: V3) -> Collision {
        let mut c : Collision = self.dc;
        let mut bd : f64 = f64::MAX; 
        let cv = vec![Color::CYAN, Color::BLUE, Color::GREEN, Color::GRAY, Color::YELLOW];  

        for (i, f) in self.x.iter().enumerate() {
            let bg = f.collides(p0, p); 
            if (bg.0 <= 1.0 && bg.0 >= 0.0 && bg.1 <= 1.0 && bg.1 >= 0.0  && bg.0 + bg.1 <= 1.0) {
                let pc: V3 = V3{
                    x: f.r.x + bg.0 * (f.a.x - f.r.x) + bg.1 * (f.b.x - f.r.x), 
                    y: f.r.y + bg.0 * (f.a.y - f.r.y) + bg.1 * (f.b.y - f.r.y), 
                    z: f.r.z + bg.0 * (f.a.z - f.r.z) + bg.1 * (f.b.z - f.r.z)  
                }; 
                let d : f64 = pc.d(p0); 
                
                if (d < bd) {
                    bd = d;
                    c = Collision{p: pc, hit: true, c: cv[i % 5]};
                }
            }
        }

        return c;
    }
}