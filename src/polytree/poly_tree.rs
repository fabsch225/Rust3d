use std::borrow::Borrow;
use std::cell::Cell;
use std::rc::Rc;

use sdl2::pixels::Color;

use crate::engine_pa::PathtracingObject;
use crate::face::{Face as F, UV, CollisionCheckable};
use crate::point::Point as V3;
use crate::poly_shape::{Collision, Poly};
use crate::polytree::poly_tree_element::PolyTreeElement;

pub struct PolyTree<'pt> {
    pub m : V3,
    pub root : PolyTreeElement<'pt>,
    pub source : &'pt Poly,
}

impl<'pt> PathtracingObject for PolyTree<'pt> {
    fn d(&self, p: V3) -> f64 {
        return 0.0; //todo
    }
    fn color(&self, p: V3) -> Color {
        return self.source.base_color;
    }
    fn rot(&mut self, r_: V3) {
        //self.source.rot(r_);
    }
    fn trans(&mut self, p: V3) { 
        //self.source.trans(p);
    }
    fn scale(&mut self, p: V3) { 
        //self.source.scale(p);
    }
    fn is_colliding(&mut self, p0: V3, p: V3) -> bool {
        return self.root.get_collision(p0, p).hit;
    }
    fn get_collision(&self, p0: V3, p: V3) -> Collision {
        let ptcf = self.root.get_collision(p0, p);
        
        if (ptcf.hit) {
            let mut c : Collision = Collision { p: ptcf.p, hit: true, c: self.source.base_color };
            let uv = ptcf.uv;
            let y = ptcf.p.y;
            let x = ptcf.p.x;

            let ty = (x * self.source.th as f64) as u32;
            let tx = (y * self.source.tw as f64) as u32;

            let pos = ((tx + ty * self.source.th) * 3) as usize;

            if pos >= self.source.tf.len() {
                c.c = Color::RED;
            }
            else {
                let r = self.source.tf[pos]; 
                let g = self.source.tf[pos + 1];
                let b = self.source.tf[pos + 2];

                c.c = Color::RGB(r, g, b);
            }

            return c;
        }
        else {
            return Collision{p: p0, hit: false, c: Color::RED};
        }
    }
}

impl<'pt> PolyTree<'pt> {
    pub fn new(p: &'pt Poly) -> Self {
        PolyTree {
            m: p.m,
            source: p,
            root: PolyTree::make_polytree_element(&p.x, &p.tm)
        }
    }

    pub fn make_polytree_element<'pd>(fs: &'pd [F], uvs: &'pd [UV]) -> PolyTreeElement<'pd> {

        let mut fsvec = Vec::new();
        let mut uvvec = Vec::new();
        for i in 0..fs.len() {
            fsvec.push(&fs[i]);
            uvvec.push(&uvs[i]);
        }


        if fs.len() < 8 {
            let m_ = PolyTree::get_middle(&fsvec);
            let r_ = PolyTree::get_radius(&fsvec);

            return PolyTreeElement {
                children: Vec::new(),
                faces: fsvec,
                uvs: uvvec,
                m: m_,
                radius: r_,
                leaf: true
            }
        }
        else {
            let m_ = PolyTree::get_middle(&fsvec);
            let r_ = PolyTree::get_radius(&fsvec);

            let mut children : Vec<Option<Box<PolyTreeElement<'pd>>>> = Vec::new();
            let (dfsc, duvs) = PolyTree::divide_faces(&fsvec, &uvvec);
            for i in 0..8 {

                let mut dfsc_ = Vec::new();
                let mut duvs_ = Vec::new();
                for j in 0..dfsc[i].len() {
                    dfsc_.push(dfsc[i][j]);
                    duvs_.push(duvs[i][j]);
                }

                children.push(Option::Some(Box::new(PolyTree::make_polytree_element(fs, uvs))));
            }

            return PolyTreeElement {
                children: children,
                faces: Vec::new(),
                uvs: Vec::new(),
                m: m_,
                radius: r_,
                leaf: false
            }
        }
    }


    pub fn get_middle_from_poly(p : &Poly) -> V3 {
        let mut middle : V3 = V3{x: 0.0, y: 0.0, z: 0.0};
        for i in 0..p.x.len() {
            middle.add(p.x[i].m);
        }
        middle.mult(1.0 / p.x.len() as f64);
        return middle;
    }

    pub fn get_radius_from_poly(p : &Poly) -> f64 {  
        return 0.0;
    }

    pub fn get_middle(x : &Vec<&F>) -> V3 {
        let mut middle : V3 = V3{x: 0.0, y: 0.0, z: 0.0};
        for i in 0..x.len() {
            middle.add(x[i].m);
        }
        middle.mult(1.0 / x.len() as f64);
        return middle;
    }

    pub fn get_radius(x : &Vec<&F>) -> f64 {
        let mut r : f64 = 0.0;
        for i in 0..x.len() {
            let d = x[i].m.d(PolyTree::get_middle(x));
            let r_ = x[i].get_radius();
            if (d + r_ > r) {
                r = d + r_;
            }
        }

        r
    }

    pub fn divide_faces<'p>(fs : &'p Vec<&'p F>, uvs: &'p Vec<&'p UV>) -> (Vec<Vec<&'p F>>, Vec<Vec<&'p UV>>) {
        let mut dfsc : Vec<Vec<&F>> = Vec::new();
        let mut duvs : Vec<Vec<&UV>> = Vec::new();
        for _ in 0..8 {
            dfsc.push(Vec::new());
            duvs.push(Vec::new());
        }

        let mut middle : V3 = PolyTree::get_middle(fs);
        
        for i in 0..fs.len() {
            let f = &fs[i];
            let uv = &uvs[i];
            if f.m.x < middle.x {
                if f.m.y < middle.y {
                    if f.m.z < middle.z {
                        dfsc[0].push(f);
                        duvs[0].push(uv);
                    }
                    else {
                        dfsc[1].push(f);
                        duvs[1].push(uv);
                    }
                }
                else {
                    if f.m.z < middle.z {
                        dfsc[2].push(f);
                        duvs[2].push(uv);
                    }
                    else {
                        dfsc[3].push(f);
                        duvs[3].push(uv);
                    }
                }
            }   
            else {
                if f.m.y < middle.y {
                    if f.m.z < middle.z {
                        dfsc[4].push(f);
                        duvs[4].push(uv);
                    }
                    else {
                        dfsc[5].push(f);
                        duvs[5].push(uv);
                    }
                }
                else {
                    if f.m.z < middle.z {
                        dfsc[6].push(f);
                        duvs[6].push(uv);
                    }
                    else {
                        dfsc[7].push(f);
                        duvs[7].push(uv);
                    }
                }
            }
        }

        
        return (dfsc, duvs);
    }
}