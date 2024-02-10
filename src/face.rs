use crate::point::Point as V3;

pub struct Face {
    pub r: V3,
    pub a: V3, 
    pub b: V3,
    pub n: V3,
    pub m: V3,
    pub radius: f64
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
        v.subtr(r_);
        let a = v.norm();
        v = m_.clone();
        v.subtr(a_);
        let b = v.norm();
        v = m_.clone();
        v.subtr(b_);
        let c = v.norm();
        return f64::max(f64::max(a, b), c);
    }

    pub fn calculate_middle(r_: V3, a_: V3, b_: V3) -> V3 {
        let mut v : V3 = a_.clone();
        v.add(r_);
        v.add(b_);
        v.mult(1.0 / 3.0);
        return v;
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

        // optimization. slows it down
        
        let mut to_middle = self.m.clone();
        to_middle.subtr(p0);
        let mut p_ = p.clone();
        
        p_.mult(self.radius / p_.norm());
        // let mut m_ = self.m.clone();

        let delta : f64 = to_middle.dt(p_) / p_.norm();

        //p_.mult(to_middle.dt(p_) / p_.norm_sq());
        //p_.add(p0);
        //m_.subtr(p_);
        //let delta : f64 = m_.norm();
        //(no) somehow, this solves objects beeing behind the camera

        if (delta < self.radius) {
            //println!("works here");
            return (-1.0, -1.0);
        }
        

        //gauss

        let (a, b, c) = (p0.x, p0.y, p0.z);
        let (d, e, f) = (p.x, p.y, p.z);
        let (x, y, z) = (self.r.x, self.r.y, self.r.z);

        let n1 = -(self.a.x - x) * e + (self.a.y - y) * d;
        let n2 = -(self.a.y - y) * f + (self.a.z - z) * e; 
        let n3 = -(self.b.x - x) * e + (self.b.y - y) * d;
        let n4 = -(self.b.y - y) * f + (self.b.z - z) * e;

        let o1 = (x - a) * e - (y - b) * d;  
        let o2 = (y - b) * f - (z - c) * e; 

        let beta = (o1 * n4 - o2 * n3) / (n1 * n4 - n2 * n3); 
        let gamma = (o1 - beta * n1) / n3;

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

        self.update();
 	}

    fn update(&mut self) {
        self.n = Face::calculate_norm(self.r, self.a, self.b);
        self.m = Face::calculate_middle(self.r, self.a, self.b);
        self.radius = Face::calculate_radius(self.m, self.r, self.a, self.b);
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

        self.update();
 	}
    
    pub fn trans(&mut self, p: V3) {
    	self.r.trans(p.x, p.y, p.z);
        self.a.trans(p.x, p.y, p.z);
        self.b.trans(p.x, p.y, p.z);

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

#[derive(Copy, Clone)]
pub struct UV {
    pub r: (f64, f64),
    pub a: (f64, f64), 
    pub b: (f64, f64)
}