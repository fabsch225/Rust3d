use crate::engine::{pathtracing::PathtracingObject, utils::{rendering::{Collision, RayRenderScene, RayRenderable, RaySphereable}, transformation::Transformable}};
use crate::geometry::vector3::Vector3 as V3;

#[derive(Debug, Clone, Copy)]
pub struct Face {
    pub r: V3,
    pub a: V3, 
    pub b: V3,
    pub n: V3,
    pub m: V3,
    pub radius: f64
}

impl RaySphereable for Face {
    fn get_radius(&self) -> f64 {
        return self.radius;
    }

    fn get_middle(&self) -> V3 {
        return self.m;
    }
}

impl Face {
    pub fn new(r_  : V3, a_ : V3, b_ : V3) -> Self {
        Face {
            r : r_,
            a : a_,
            b : b_,
            n : Face::calculate_norm(r_, a_, b_),
            m: Face::calculate_middle(r_, a_, b_),
            radius: Face::calculate_radius(Face::calculate_middle(r_, a_, b_), r_, a_, b_)
        }
    }

    pub fn d(_p : V3) {

    }

    pub fn calculate_radius(m_: V3, r_: V3, a_: V3, b_: V3) -> f64 {
        let mut v : V3 = m_.clone();
        v.subtract(r_);
        let a = v.norm();
        v = m_.clone();
        v.subtract(a_);
        let b = v.norm();
        v = m_.clone();
        v.subtract(b_);
        let c = v.norm();
        return f64::max(f64::max(a, b), c);
    }

    pub fn calculate_middle(r_: V3, a_: V3, b_: V3) -> V3 {
        let mut v : V3 = a_.clone();
        v.add(r_);
        v.add(b_);
        v.scale(1.0 / 3.0);
        return v;
    }

    pub fn calculate_norm(r_: V3, a_: V3, b_: V3) -> V3 {
        let mut v1 : V3 = a_.clone();
        let mut v2 : V3 = b_.clone();
        v1.subtract(r_);
        v2.subtract(r_);
        v1.cross(v2);
        return v1;
    }

    
    pub fn get_beta_gamma(&self, p0: V3, p: V3) -> (f64, f64) {   

        //print!("{} {} {} {} {} {} ", p0.x, p0.y, p0.z, p.x, p.y, p.z);

        //gauss (with early returns)

        let (a, b, c) = (p0.x, p0.y, p0.z);
        let (d, e, f) = (p.x, p.y, p.z);
        let (x, y, z) = (self.r.x, self.r.y, self.r.z);

        //if n3 == 0 -> early return
        let n3 = -(self.b.x - x) * e + (self.b.y - y) * d;
        if (n3 == 0.) {
            return (-1., -1.);
        }

        let n1 = -(self.a.x - x) * e + (self.a.y - y) * d;
        let n2 = -(self.a.y - y) * f + (self.a.z - z) * e;

        let n4 = -(self.b.y - y) * f + (self.b.z - z) * e;

        let n1n4n2n3_pre = n1 * n4 - n2 * n3;
        //if (n1 * n4 - n2 * n3) == 0 -> early return
        if (n1n4n2n3_pre == 0.) {
            return (-1., -1.);
        }

        let o1 = (x - a) * e - (y - b) * d;  
        let o2 = (y - b) * f - (z - c) * e; 

        let beta = (o1 * n4 - o2 * n3) / n1n4n2n3_pre;
        let gamma = (o1 - beta * n1) / n3;

        (beta, gamma)
    } 

    pub fn rot_reverse(&mut self, r_: V3, p: V3) {
        self.r.subtract(p);
        self.a.subtract(p);
        self.b.subtract(p);

        self.r.rot_reverse(r_);
        self.a.rot_reverse(r_);
        self.b.rot_reverse(r_);

        self.r.add(p);
        self.a.add(p);
        self.b.add(p);

        self.update();
 	}

    fn update(&mut self) {
        self.n = Face::calculate_norm(self.r, self.a, self.b);
        self.m = Face::calculate_middle(self.r, self.a, self.b);
        self.radius = Face::calculate_radius(self.m, self.r, self.a, self.b);
    }

    pub fn rot(&mut self, r_: V3, p: V3) {
        self.r.rot_by(p, r_);
        self.a.rot_by(p, r_);
        self.b.rot_by(p, r_);
        self.m.rot_by(p, r_);
        self.update();
 	}
    
    pub fn trans(&mut self, p: V3) {
    	self.r.translate(p.x, p.y, p.z);
        self.a.translate(p.x, p.y, p.z);
        self.b.translate(p.x, p.y, p.z);

        self.update();
    }

    pub fn scale_by(&mut self, p: V3, m: V3) {
        let ax = (self.a.x - m.x) * p.x;
        let ay = (self.a.y - m.y) * p.y;
        let az = (self.a.z - m.z) * p.z;

        self.a.x = m.x + ax;
        self.a.y = m.y + ay;
        self.a.z = m.z + az;

        let bx = (self.b.x - m.x) * p.x;
        let by = (self.b.y - m.y) * p.y;
        let bz = (self.b.z - m.z) * p.z;

        self.b.x = m.x + bx;
        self.b.y = m.y + by;
        self.b.z = m.z + bz;

        let rx = (self.r.x - m.x) * p.x;
        let ry = (self.r.y - m.y) * p.y;
        let rz = (self.r.z - m.z) * p.z;

        self.r.x = m.x + rx;
        self.r.y = m.y + ry;
        self.r.z = m.z + rz;

        self.update();
    }
}

#[derive(Debug, Copy, Clone)]
pub struct UV {
    pub r: (f64, f64),
    pub a: (f64, f64), 
    pub b: (f64, f64)
}

impl UV {
    pub fn empty() -> Self {
        UV {
            r: (0.0, 0.0),
            a: (0.0, 0.0),
            b: (0.0, 0.0)
        }
    }
}