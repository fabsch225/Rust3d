pub mod cube;
pub mod point;

use cube::Cube;
use point::Point;

pub struct Camera {
	pub v: [Point; 3],
	pub zoom: f64,
	pub x: Point,
	pub rx: f64,
	pub ry: f64,
	pub rz: f64,
}

impl Camera {
	 pub fn new(p: Point, rx_: f64, ry_: f64, rz_: f64) -> Self {
		let mut v_ : [Point; 3] = [
	    		Point{x: 1.0, y: -0.5, z: -0.5},
	    		Point{x: 1.0, y: 0.5, z: -0.5},
	    		Point{x: 1.0, y: -0.5, z: 0.5}
	    	];

		 for i in 0..3 {
			v_[i].rot(Point{x: rx_, y: ry_, z: rz_});
			v_[i].trans(p.x, p.y, p.z);
		}
    	
        Camera {
	        v: v_,
        	x: p,
            rx: rx_,
            ry: ry_,
            rz: rz_,
            zoom: 1.0
        }
    }

}



