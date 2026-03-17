use crate::math::vector::NVector;
use crate::engine::projection::projection::{Projectable, Projection};
use crate::engine::projection::raster::Raster;
use crate::engine::utils::virtual_canvas::Color;
use crate::geometry::d2::line2d::Line2D;
use crate::geometry::point::Point;
use crate::math::matrix::MatrixND;

pub struct NLine {
    pub a: NVector,
    pub b: NVector,
    pub width: f64,
    pub color: Color,
    pub scale: f64,
}

impl NLine {
    pub fn new(a: NVector, b: NVector, width: f64, color: Color, scale: f64) -> Self {
        assert_eq!(a.n, b.n);
        NLine { a, b, width, color, scale }
    }

    fn perspective_project_to_2d(v: &NVector) -> (f64, f64) {
        assert!(v.n >= 2);

        let mut coords = v.x.clone();
        for dim in (2..v.n).rev() {
            let d = 3.0 + dim as f64;
            let denom = d - coords[dim];
            let factor = if denom.abs() < 1e-6 { 1.0 } else { d / denom };

            for i in 0..dim {
                coords[i] *= factor;
            }
        }

        (coords[0], coords[1])
    }

    fn to_line2d(&self, width: usize, height: usize) -> Line2D {
        let (ax, ay) = Self::perspective_project_to_2d(&self.a);
        let (bx, by) = Self::perspective_project_to_2d(&self.b);

        let start = Point {
            x: (width as f64 * 0.5) + ax * self.scale,
            y: (height as f64 * 0.5) - ay * self.scale,
        };

        let end = Point {
            x: (width as f64 * 0.5) + bx * self.scale,
            y: (height as f64 * 0.5) - by * self.scale,
        };

        Line2D::new(start, end, self.width, self.color)
    }
}

impl Projectable for NLine {
    fn project(&self, _mat: &MatrixND) -> Box<&dyn Projection> {
        Box::new(self as &dyn Projection)
    }
}

impl Projection for NLine {
    fn rasterize(&self, width: usize, height: usize) -> Raster {
        self.to_line2d(width, height).rasterize(width, height)
    }
}