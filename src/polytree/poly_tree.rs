use std::borrow::Borrow;
use std::rc::Rc;

use sdl2::pixels::Color;

use crate::engine_pa::PathtracingObject;
use crate::face::{Face as F, UV, CollisionCheckable};
use crate::point::Point as V3;
use crate::poly_shape::{Collision, Poly};
use crate::polytree::poly_tree_element::PolyTreeElement;

pub struct PolyTree<'pd> {
    pub m : V3,
    pub root : PolyTreeElement<'pd>,
    pub source : &'pd Poly,
}

impl PathtracingObject for PolyTree<'_> {
    fn d(&self, p: V3) -> f64 {
        return 0.0; //todo
    }
    fn color(&self, p: V3) -> Color {
        return self.source.base_color;
    }
    fn rot(&mut self, r_: V3) {
        self.root.rot(r_, self.m);
    }
    fn trans(&mut self, p: V3) { 
    }
    fn scale(&mut self, p: V3) { 
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

impl PolyTree<'_> {
    pub fn new<'pd>(p: Poly) -> Self {
        
        let face_refs: Vec<&F> = p.x.iter().collect();
        let uv_refs: Vec<&UV> = p.tm.iter().collect();

        PolyTree { 
            m: p.m,
            root: PolyTree::make_polytree_element( &face_refs, &uv_refs),
            source: &p
        }
    }

    pub fn make_polytree_element<'pd>(fs: &'pd  Vec<&F>, uvs: &'pd  Vec<&UV>) -> PolyTreeElement<'pd> {
        if fs.len() < 8 {
            return PolyTreeElement {
                children: Vec::new(),
                faces: fs,
                uvs: uvs,
                m: PolyTree::get_middle(fs),
                radius: PolyTree::get_radius(fs),
                leaf: true
            }
        }
        else {
            let mut children : Vec<PolyTreeElement> = Vec::new();
            let (dfsc, duvs) = PolyTree::divide_faces(fs, uvs);
            for i in 0..8 {
                children.push(PolyTree::make_polytree_element(&dfsc[i], &duvs[i]));
            }
            return PolyTreeElement {
                children: children,
                faces: &Vec::new(),
                uvs: &Vec::new(),
                m: PolyTree::get_middle(fs),
                radius: PolyTree::get_radius(fs),
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
        return 0.0;
    }

    pub fn divide_faces<'f, 'u>(fs : &'f Vec<&F>, uvs: &'u Vec<&UV>) -> (Vec<Vec<&'f F>>, Vec<Vec<&'u UV>>) {
        let mut dfsc : Vec<Vec<&'f F>> = Vec::new();
        let mut duvs : Vec<Vec<&'u UV>> = Vec::new();
        for i in 0..8 {
            dfsc.push(Vec::new());
            duvs.push(Vec::new());
        }

        let mut middle : V3 = PolyTree::get_middle(fs);
        
        for i in 0..fs.len() {
            let f = fs[i];
            let uv = uvs[i];
            if f.m.x < middle.x {
                if f.m.y < middle.y {
                    if f.m.z < middle.z {
                        dfsc[0].push(&f);
                        duvs[0].push(&uv);
                    }
                    else {
                        dfsc[1].push(&f);
                        duvs[1].push(&uv);
                    }
                }
                else {
                    if f.m.z < middle.z {
                        dfsc[2].push(&f);
                        duvs[2].push(&uv);
                    }
                    else {
                        dfsc[3].push(&f);
                        duvs[3].push(&uv);
                    }
                }
            }   
            else {
                if f.m.y < middle.y {
                    if f.m.z < middle.z {
                        dfsc[4].push(&f);
                        duvs[4].push(&uv);
                    }
                    else {
                        dfsc[5].push(&f);
                        duvs[5].push(&uv);
                    }
                }
                else {
                    if f.m.z < middle.z {
                        dfsc[6].push(&f);
                        duvs[6].push(&uv);
                    }
                    else {
                        dfsc[7].push(&f);
                        duvs[7].push(&uv);
                    }
                }
            }
        }
        return (dfsc, duvs);
    }
}