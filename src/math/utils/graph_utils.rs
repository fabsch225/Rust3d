use sdl2::pixels::Color;

use crate::{engine::polytree::poly_tree::PolyTree, geometry::{face::{Face, UV}, point::Point, poly_shape::Poly, quad::Quad}, math::functions::{FunctionR2ToR, FunctionRToR}};

pub trait PolyTreeGraphFactory {
    fn create_graph(&self, bounds : Quad, delta : f64) -> Box<PolyTree>;
}

impl PolyTreeGraphFactory for FunctionR2ToR {
    fn create_graph(&self, bounds : Quad, delta : f64) -> Box<PolyTree> {
        let mut faces: Vec<Face> = Vec::new();
        let mut uvs: Vec<UV> = Vec::new();
        let mins = bounds.mins();
        let startx = mins[0];
        let starty = mins[1];
        let endx = mins[3];
        let endy = mins[4];
        let minz = mins[2];
        let maxz = mins[5];

        let mut x = startx;
        let mut y = starty;

        let minx = startx;
        let miny = starty;
        let eval = |x, y| {
            self.eval(x - minx, y - miny)
        };

        while x < endx {
            y = starty;
            while y < endy {
                let p = Point::new(x, y, minz + eval(x, y));
                let p1 = Point::new(x + delta, y, minz + eval(x + delta, y));
                let p2 = Point::new(x, y + delta, minz + eval(x, y + delta));
                let p3 = Point::new(x + delta, y + delta, minz + eval(x + delta, y + delta));
                faces.push(Face::new(p, p1, p2));
                uvs.push(UV::empty());
                //faces.push(Face::new(p3, p1, p2));
                //uvs.push(UV::empty());
                y += delta;
            }
            x += delta;
        }
        let p = Poly::new_textured(Point::new(mins[0], mins[1], mins[2]), faces, uvs, vec![255, 255, 255, 255], 0, 0);
        PolyTree::new(p)
    }
}

