use crate::point::Point as V3;
use crate::face::UV;

use std::rc::Rc;

pub struct PolyTreeCollisionFeedback<'a> {
    pub hit: bool,
    pub p: V3,
    pub uv: &'a UV,
    pub bg : (f64, f64),
}

impl PolyTreeCollisionFeedback<'_> {
    pub fn empty () -> Self {
        PolyTreeCollisionFeedback{hit: false, p: V3{x: 0.0, y: 0.0, z: 0.0}, uv: &UV{r: (0.0, 0.0), a: (0.0, 0.0), b: (0.0, 0.0)}, bg: (0.0, 0.0)}
    }
}