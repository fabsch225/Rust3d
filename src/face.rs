use crate::point::Point as V3;

pub struct Face {
    pub r: V3,
    pub a: V3, 
    pub b: V3,
    pub n: V3,
}

impl Face {
    pub fn new(r_  : V3, a_ : V3, b_ : V3) -> Self {
        Face {
            r : r_,
            a : a_,
            b : b_,
            n : Face::calculate_norm(r_, a_, b_)
        }
    }

    pub fn d(_p : V3) {

    }

    pub fn calculate_norm(r_: V3, a_: V3, b_: V3) -> V3 {
        let mut v1 : V3 = a_.clone();
        let mut v2 : V3 = b_.clone();
        v1.subtr(r_);
        v2.subtr(r_);
        v1.cross(v2);
        return v1;
    }

    pub fn collides(&self, p0: V3, p: V3) -> (f64, f64) { 


        let (a, b, c) = (p0.x, p0.y, p0.z);
        let (d, e, f) = (p.x, p.y, p.z);

        let (x, y, z) = (self.r.x, self.r.y, self.r.z);
        let (x1, y1, z1) = (self.a.x - x, self.a.y - y, self.a.z - z);
        let (x2, y2, z2) = (self.b.x - x, self.b.y - y, self.b.z - z);

        let beta = ((x - a) * e - (y - b) * d) * (-y2 * f + z2 * e) / ((-x1 * e + y1 * d) * (-y2 * f + z2 * e) - (-y1 * f + z1 * e) * (-x2 * e + y2 * d));
        let gamma = ((x - a) * e - (y - b) * d - beta * (-x1 * e + y1 * d)) / (-x2 * e + y2 * d);

        return (beta, gamma);
    }

    pub fn rot_reverse(&mut self, r_: V3, p: V3) {
        self.r.subtr(p);
        self.a.subtr(p);
        self.b.subtr(p);

        self.r.rot_reverse(r_);
        self.a.rot_reverse(r_);
        self.b.rot_reverse(r_);

        self.r.add(p);
        self.a.add(p);
        self.b.add(p);
 	}
    
    pub fn rot(&mut self, r_: V3, p: V3) {
        self.r.subtr(p);
        self.a.subtr(p);
        self.b.subtr(p);

        self.r.rot(r_);
        self.a.rot(r_);
        self.b.rot(r_);

        self.r.add(p);
        self.a.add(p);
        self.b.add(p);
 	}
    
    pub fn trans(&mut self, p: V3) {
    	self.r.trans(p.x, p.y, p.z);
        self.a.trans(p.x, p.y, p.z);
        self.b.trans(p.x, p.y, p.z);
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
    }
}

#[derive(Copy, Clone)]
pub struct UV {
    pub r: (f64, f64),
    pub a: (f64, f64), 
    pub b: (f64, f64)
}