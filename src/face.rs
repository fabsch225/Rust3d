use crate::point::Point as V3;

pub struct Face {
    pub r: V3,
    pub a: V3, 
    pub b: V3,
    pub c: V3  //Last Point of collision
}

impl Face {
    pub fn new(r_  : V3, a_ : V3, b_ : V3) -> Self {
        Face {
            r : r_,
            a : a_,
            b : b_,
            c : V3{x: 0.0, y: 0.0, z: 0.0}
        }
    }

    pub fn d(p : V3) {

    }

    pub fn collides(&mut self, p0: V3, p: V3) -> bool { 
        let a : f64 = p0.x;
        let b : f64 = p0.y;
        let c : f64 = p0.z;

        let d : f64 = p.x - p0.x;
        let e : f64 = p.y - p0.y;
        let f : f64 = p.z - p0.z;

        
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
 
        println!("{}", beta);
        println!("{}", gamma);

        return true;    
    }
}