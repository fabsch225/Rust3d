use std::sync::Arc;

use sdl2::pixels::Color;

use crate::engine_utils::{Collision, Sphereable};

use crate::engine_pa::PathtracingObject;
use crate::face::{Face as F, UV};
use crate::point::Point as V3;
use crate::poly_shape::Poly;
use crate::polytree::poly_tree_element::PolyTreeElement;

use super::poly_tree_utils::PolyTreeCollisionFeedback;

#[derive(Debug, Clone)]
pub struct PolyTree {
    pub m : V3,
    pub root : PolyTreeElement,
    pub source : Poly,
}

impl PathtracingObject for PolyTree {
    fn clone(&self) -> Box<dyn PathtracingObject + 'static> {
        return Box::new(PolyTree {
            m: self.m,
            root: PolyTree::make_polytree_root(Clone::clone(&self.source)),
            source: Clone::clone(&self.source)
        })
    }
    fn d(&self, p: V3) -> f64 {
        return 0.0; //todo
    }
    fn color(&self, p: V3) -> Color {
        return self.source.base_color;
    }
    
    fn rot(&mut self, r_: V3) {
        self.root.rot(r_, self.source.m);
        self.source.rot(r_);
    }
    fn trans(&mut self, p: V3) { 
        
        self.source.trans(p);
        //self.root = PolyTree::make_polytree_root(Clone::clone(&self.source));

       
        self.root.trans(p);
        

    }
    fn scale(&mut self, p: V3) { 
        self.source.scale(p);
        self.root = PolyTree::make_polytree_root(Clone::clone(&self.source));
    }
    fn is_colliding(&mut self, p0: V3, p: V3) -> bool {
        return true;
    }
    fn get_collision(&self, p0: V3, p: V3) -> Collision {
        let mut ptcf_closest = PolyTreeCollisionFeedback::empty();
        let mut bd : f64 = f64::MAX;
        let ptcf_vec = self.root.get_collision(p0, p);
        if (ptcf_vec.len() > 0) {
            for ptcf in ptcf_vec {
                if ptcf.hit {
                    let d : f64 = ptcf.p.d(p0);
                    if (d < bd) {
                        bd = d;
                        ptcf_closest = ptcf;
                    }
                }
            }
            let mut c : Collision = Collision {d: bd, p: ptcf_closest.p, hit: true, c: self.source.base_color };
            let uv = ptcf_closest.uv;
            let y = (uv.r.0 + ptcf_closest.bg.0 * (uv.a.0 - uv.r.0) + ptcf_closest.bg.1 * (uv.b.0 - uv.r.0));
            let x = 1.0 - (uv.r.1 + ptcf_closest.bg.0 * (uv.a.1 - uv.r.1) + ptcf_closest.bg.1 * (uv.b.1 - uv.r.1));

            let ty = (x * self.source.th as f64) as u32;
            let tx = (y * self.source.tw as f64) as u32;

            let pos = ((tx + ty * self.source.th) * 3) as usize;

            //print!("{} -- ", pos);

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
        
        return Collision{d: bd, p: p0, hit: false, c: Color::RED};
        
    }
}

impl PolyTree {
    pub fn new(p: Poly) -> Box<PolyTree> {
        Box::new(PolyTree {
            m: p.m,
            source: Clone::clone(&p),
            root: PolyTree::make_polytree_root(p)
        })
    }

    pub fn update(&mut self) {
        self.root.calulate_middle();
        self.root.calculate_radius();
    }

    pub fn make_polytree_root<'pd>(p: Poly) -> PolyTreeElement {
        return Self::construct_tree(p.x, p.tm);
    }

    pub fn construct_tree(fs: Vec<F>, uvs: Vec<UV>) -> PolyTreeElement {
        let m_ = PolyTree::get_middle(&fs);
        let r_ = PolyTree::get_radius(&fs);

        if fs.len() < 200 {

            return PolyTreeElement {
                children: Vec::new(),
                faces: fs,
                uvs: uvs,
                m: m_,
                radius: r_,
                leaf: true
            }
        }
        else {
            let mut children : Vec<PolyTreeElement> = Vec::new();
            let (dfsc, duvs) = PolyTree::divide_faces(fs, uvs);
            for i in 0..8 {

                let mut dfsc_ = Vec::new();
                let mut duvs_ = Vec::new();
                for j in 0..dfsc[i].len() {
                    dfsc_.push(dfsc[i][j]);
                    duvs_.push(duvs[i][j]);
                }

                children.push(PolyTree::construct_tree(dfsc_, duvs_));
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
        middle
    }

    pub fn get_radius_from_poly(p : &Poly) -> f64 {  
        0.0
    }

    pub fn get_middle(x : &Vec<F>) -> V3 {
        let mut middle : V3 = V3{x: 0.0, y: 0.0, z: 0.0};
        for i in 0..x.len() {
            middle.add(x[i].m);
        }
        middle.mult(1.0 / x.len() as f64);
        middle
    }

    pub fn get_radius(x : &Vec<F>) -> f64 {
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

    pub fn divide_faces(fs : Vec<F>, uvs: Vec<UV>) -> (Vec<Vec<F>>, Vec<Vec<UV>>) {
        let mut dfsc : Vec<Vec<F>> = Vec::new();
        let mut duvs : Vec<Vec<UV>> = Vec::new();

        for _ in 0..8 {
            dfsc.push(Vec::new());
            duvs.push(Vec::new());
        }

        let mut middle : V3 = PolyTree::get_middle(&fs);
        
        for i in 0..fs.len() {
            let f = fs[i];
            let uv = uvs[i];
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
        assert!(dfsc.len() == duvs.len());
        
        return (dfsc, duvs);
    }
}