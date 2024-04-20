use crate::geometry::point::Point as V3;

pub trait Transformable {
    fn rot_reverse(&mut self, r : V3);
    fn rot(&mut self, r : V3);
    fn translate(&mut self, p : V3);
    fn scale(&mut self, p : V3);
    fn transform(&mut self) -> Box<&mut dyn Transformable>;
}