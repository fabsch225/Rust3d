use crate::point::Point as V3;
use crate::poly_shape::Collision;

pub struct Face {
    pub r: V3,
    pub a: V3, 
    pub b: V3,
}

impl Face {
    pub fn new(r_  : V3, a_ : V3, b_ : V3) -> Self {
        Face {
            r : r_,
            a : a_,
            b : b_
        }
    }

    pub fn d(p : V3) {

    }

    pub fn collides(&self, p0: V3, p: V3) -> (f64, f64) { 
        let a : f64 = p0.x;
        let b : f64 = p0.y;
        let c : f64 = p0.z;

        let d : f64 = p.x;// - p0.x;
        let e : f64 = p.y;// - p0.y;
        let f : f64 = p.z;// - p0.z;

        
        let x : f64 = self.r.x; 
        let y : f64 = self.r.y;
        let z : f64 = self.r.z;

        let x1 : f64 = self.a.x - x;
        let y1 : f64 = self.a.y - y;
        let z1 : f64 = self.a.z - z;

        let x2 : f64 = self.b.x - x;
        let y2 : f64 = self.b.y - y;
        let z2 : f64 = self.b.z - z; 

        let n1 : f64 = -x1 * e + y1 * d;
        let n2 : f64 = -y1 * f + z1 * e; 
        let n3 : f64 = -x2 * e + y2 * d;
        let n4 : f64 = -y2 * f + z2 * e;

        let o1 : f64 = (x - a) * e - (y - b) * d;  
        let o2 : f64 = (y - b) * f - (z - c) * e; 

        
        let beta : f64 = (o1 * n4 - o2 * n3) / (n1 * n4 - n2 * n3); 
        let gamma : f64 = (o1 - beta * n1) / n3;
 
        //let c = V3{x: x + beta * x1 + gamma * x2, y: y + beta * y1 + gamma * y2, z: z + beta * z1 + gamma * z2};
        //let hit_ : bool = beta <= 1.0 && beta >= 0.0 && gamma <= 1.0 && gamma >= 0.0  && gamma + beta <= 1.0;  
        //f64::sqrt(beta * beta + gamma * gamma) < 1.0
        //println!("{}", beta);
        //println!("{}", gamma);

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
}