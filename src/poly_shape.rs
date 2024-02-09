use std::fs::{self, File};
use std::io::{BufReader, BufRead};

use image::io::Reader as ImageReader;
use image::{Pixels, GenericImageView};
 
use rand::seq::SliceRandom;

use crate::point::Point as V3;
use crate::face::{Face as F, UV};
use crate::engine_pa::PathtracingObject;

use sdl2::pixels::Color;

pub struct Poly {
    pub m : V3,
    pub x : Vec<F>,
    pub tm : Vec<UV>,
    pub tf: Vec<u8>,
    pub tw: u32, 
    pub th: u32,
    pub base_color: Color, 
    pub has_t: bool
}

impl Poly {
    pub fn new(m_ : V3, x_ : Vec<F>) -> Self {
        Poly { 
            m: m_,
            x: x_,
            tm:  Vec::new(),
            tf: Vec::new(),
            tw: 0, 
            th: 0,
            base_color: Color::RGB(0,0,0),
            has_t: false
        }
    }    

    pub fn new_textured(m_ : V3, x_ : Vec<F>, tm_ : Vec<UV>, tf_ : Vec<u8>, tw_ : u32, th_ : u32) -> Self {
        Poly { 
            m: m_,
            x: x_,
            tm:  tm_,  
            tf: tf_,
            tw: tw_, 
            th: th_,
            base_color: Color::RGB(0,0,0),
            has_t: true
        }
    }  

    pub fn parse_wavefront(f: String, tf: String) -> Self { 
        let mut vertices : Vec<V3> = Vec::new(); 
        let mut middle : V3 = V3{x: 0.0, y: 0.0, z: 0.0};

        let mut uvs : Vec<(f64, f64)> = Vec::new();

        let mut faces : Vec<F> = Vec::new();
        let mut texture_map : Vec<UV> = Vec::new();

        let reader = BufReader::new(File::open(f).expect("Cannot open file"));
         
        for line in reader.lines() { 
            let mut s = line.expect("");
            let ss = s.split_off(2);
            let mut es = ss.split_whitespace(); 

            if (s.eq("v ")) {
                let x_: f64 = es.next().expect("").parse::<>().unwrap();
                let y_: f64 = es.next().expect("").parse::<>().unwrap();
                let z_: f64 = es.next().expect("").parse::<>().unwrap();
 
                //println!("V {}, {}, {} ", x_, y_, z_);

                vertices.push(V3{x: x_, y: y_, z: z_});
                middle.add(V3{x: x_, y: y_, z: z_});
            } 
            else if (s.eq("vt")) {
                let x_: f64 = es.next().expect("").parse::<>().unwrap();
                let y_: f64 = es.next().expect("").parse::<>().unwrap();
            
                uvs.push((x_, y_));
            }
            else if (s.eq("f ")) {
                let mut _1 = es.next().expect("").split("/");
                let mut _2 = es.next().expect("").split("/");
                let mut _3 = es.next().expect("").split("/");

                let ri : usize = _1.next().expect("").parse::<usize>().unwrap() - 1;
                let ai : usize = _2.next().expect("").parse::<usize>().unwrap() - 1;
                let bi : usize = _3.next().expect("").parse::<usize>().unwrap() - 1;

                let tri : usize = _1.next().expect("").parse::<usize>().unwrap() - 1;
                let tai : usize = _2.next().expect("").parse::<usize>().unwrap() - 1;
                let tbi : usize = _3.next().expect("").parse::<usize>().unwrap() - 1;

                faces.push(F::new(vertices[ri], vertices[ai], vertices[bi]));
                texture_map.push(UV{r: uvs[tri], a: uvs[tai], b: uvs[tbi]});
            }
        }

        middle.mult(1.0 / vertices.len() as f64);

        let mut img = image::open(tf).unwrap();

        let img_width = img.dimensions().0;
        let img_height = img.dimensions().1;

        let rgb: Vec<u8> = img.to_rgb8().to_vec();
        println!("Texture Loaded: w{}, h{}, total{}", img_width, img_height, rgb.len());

        return Self::new_textured(middle, faces, texture_map, rgb, img_width, img_height);
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

    fn trans(&mut self, p: V3) { 
        for f in self.x.iter_mut() {
            f.trans(p);
        }
        self.m.trans(p.x, p.y, p.z);
    }

    fn scale(&mut self, p: V3) { 
        for f in self.x.iter_mut() {
            f.scale_by(p, self.m);
        }
        self.m.trans(p.x, p.y, p.z);
    }

	fn is_colliding(&mut self, p0: V3, p: V3) -> bool {
        return true;
    }

	fn get_collision(&self, p0: V3, p: V3) -> Collision {
        let mut c : Collision = Collision { p: (p0), hit: (false), c: (self.base_color) };
        let mut bd : f64 = f64::MAX; 
        let mut i : usize = 0;
        let mut bg : (f64, f64) = (0.0, 0.0);
        let cv = vec![Color::CYAN, Color::BLUE, Color::GREEN, Color::GRAY, Color::YELLOW];  

        for (i_, f) in self.x.iter().enumerate() {
            let bg_ = f.collides(p0, p); 
            if (bg_.0 <= 1.0 && bg_.0 >= 0.0 && bg_.1 <= 1.0 && bg_.1 >= 0.0  && bg_.0 + bg_.1 <= 1.0) {
                let pc: V3 = V3{
                    x: f.r.x + bg_.0 * (f.a.x - f.r.x) + bg_.1 * (f.b.x - f.r.x), 
                    y: f.r.y + bg_.0 * (f.a.y - f.r.y) + bg_.1 * (f.b.y - f.r.y), 
                    z: f.r.z + bg_.0 * (f.a.z - f.r.z) + bg_.1 * (f.b.z - f.r.z)  
                }; 
                let d : f64 = pc.d(p0); 
                
                if (d < bd) {
                    bg = bg_;
                    bd = d; 
                    i = i_;
                    c = Collision{p: pc, hit: true, c: cv[i_ % 5]};
                }
            }
        }

        if (c.hit) {   
            let uv = self.tm[i];
            let y = (uv.r.0 + bg.0 * (uv.a.0 - uv.r.0) + bg.1 * (uv.b.0 - uv.r.0));
            let x = 1.0 - (uv.r.1 + bg.0 * (uv.a.1 - uv.r.1) + bg.1 * (uv.b.1 - uv.r.1));

            //println!("{}, {}, {}, {}, {}, {}, {}", uv.r.1, (uv.a.1 - uv.r.1), (uv.b.1 - uv.r.1), bg.0, bg.1, x, y);

            let ty = (x * self.th as f64) as u32;
            let tx = (y * self.tw as f64) as u32;

            let pos = ((tx + ty * self.th) * 3) as usize;

            if pos >= self.tf.len() {
                c.c = Color::RED;
            }
            else {
                let r = self.tf[pos]; 
                let g = self.tf[pos + 1];
                let b = self.tf[pos + 2];

                c.c = Color::RGB(r, g, b);

                //println!("{}, {}, {}, {}, {}", tx, ty, r, g, b);
            }
        }

        return c;
    }
}