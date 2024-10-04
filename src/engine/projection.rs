//ToDo this will be more generalized

use crate::geometry::point::Point;
use crate::math::matrix::NMatrix;

pub struct ProjectiveScene {
    pub objects: Vec<Box<dyn Projectable>>
}

pub trait Projectable {
    fn project(&self, &NMatrix) -> dyn Projection;
}

pub trait Projection {
    fn rasterize(&self) -> Raster;
}

pub struct Raster {
    pub z: i32,
    pub rec_start: Point,
    pub rec_end: Point,
    pub width: i32,
    pub pixels: Vec<(u8, u8, u8, u8)>
}