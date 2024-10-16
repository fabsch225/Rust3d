use std::sync::{mpsc, Arc};
use std::thread;
use sdl2::pixels::Color;

use crate::engine::utils::{rendering::{RayRenderScene, RayRenderable, Collision, RaySphereable}, transformation::Transformable};
use crate::engine::pathtracing::PathtracingObject;
use crate::geometry::face::{Face as F, UV};
use crate::geometry::vector3::Vector3 as V3;
use crate::geometry::simplex3d::Simplex3D;
use crate::engine::polytree::poly_tree_element::PolyTreeElement;

use super::poly_tree_utils::PolyTreeCollisionFeedback;

#[derive(Debug, Clone)]
pub struct PolyTree {
    pub m : V3,
    pub root : PolyTreeElement,
    pub source : Simplex3D,
}

impl Transformable for PolyTree {
    fn rot_reverse(&mut self, r_: V3) {
        self.root.rot_reverse(r_, self.source.m);
        self.source.rot_reverse(r_);
    }
    fn rot(&mut self, r_: V3) {
        self.root.rot(r_, self.source.m);
        self.source.rot(r_);
    }
    fn rot_by(&mut self, p : V3, r : V3) {
        self.source.rot_by(p, r);
        self.root.rot_by(p, r);
    }
    fn translate(&mut self, p: V3) { 
        
        self.source.translate(p);
        //self.root = PolyTree::make_polytree_root(Clone::clone(&self.source));

        self.root.trans(p);
    }
    fn scale(&mut self, p: V3) { 
        self.source.scale(p);
        self.root = PolyTree::make_polytree_root(Clone::clone(&self.source));
    }

    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        return Box::new(self);
    }
}

impl PathtracingObject for PolyTree {
    fn d(&self, p: V3) -> f64 {
        return 0.0; //todo
    }
    fn color(&self, p: V3) -> Color {
        return self.source.base_color;
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
    fn clone(&self) -> Box<dyn PathtracingObject + Send + Sync + 'static> {
        return Box::new(PolyTree {
            m: self.m,
            root: Clone::clone(&self.root),
            source: Clone::clone(&self.source)
        })
    }
}

impl PolyTree {
    pub fn new(p: Simplex3D) -> Box<PolyTree> {
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

    pub fn make_polytree_root<'pd>(p: Simplex3D) -> PolyTreeElement {
        return Self::construct_tree(p.x, p.tm, true);
    }

    //ToDo Cache this, load from file
    //ToDo multithreading
    pub fn construct_tree(fs: Vec<F>, uvs: Vec<UV>, use_threads: bool) -> PolyTreeElement {
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
            if (use_threads) {
                let (tx, rx) = mpsc::channel();
                let dfsc = Arc::new(dfsc);
                let duvs = Arc::new(duvs);

                for i in 0..8 {
                    let tx = tx.clone();
                    let dfsc = Arc::clone(&dfsc);
                    let duvs = Arc::clone(&duvs);

                    thread::spawn(move || {
                        let mut dfsc_ = Vec::new();
                        let mut duvs_ = Vec::new();

                        for j in 0..dfsc[i].len() {
                            dfsc_.push(dfsc[i][j]);
                            duvs_.push(duvs[i][j]);
                        }

                        let child = PolyTree::construct_tree(dfsc_, duvs_, false);
                        tx.send(child).expect("Failed to send the result");
                    });
                }
                let mut i = 0;
                for child in rx.iter().take(8) {
                    i += 1;
                    children.push(child);
                }
            } else {
                for i in 0..8 {
                    let mut dfsc_ = Vec::new();
                    let mut duvs_ = Vec::new();
                    for j in 0..dfsc[i].len() {
                        dfsc_.push(dfsc[i][j]);
                        duvs_.push(duvs[i][j]);
                    }

                    children.push(PolyTree::construct_tree(dfsc_, duvs_, false));
                }
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

    pub fn goto(&mut self, p: V3) {
        let mut p_ = p;
        p_.subtr(self.m);
        self.translate(p_);
    }

    pub fn get_middle_from_poly(p : &Simplex3D) -> V3 {
        let mut middle : V3 = V3{x: 0.0, y: 0.0, z: 0.0};
        for i in 0..p.x.len() {
            middle.add(p.x[i].m);
        }
        middle.mult(1.0 / p.x.len() as f64);
        middle
    }

    pub fn get_radius_from_poly(p : &Simplex3D) -> f64 {
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
        let m = PolyTree::get_middle(x);
        for i in 0..x.len() {
            let d = x[i].m.d(m);
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

        (dfsc, duvs)
    }
}