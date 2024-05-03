use crate::geometry::point::Point as V3;

pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64
pub const TWO_PI: f64 = 6.28318530717958647692528676655900576_f64; // 6.2831853071795862f64 //copilot did this, maybe not exactly correct

pub trait Transformable {
    fn rot_reverse(&mut self, r : V3) {
        self.rot(V3{x: TWO_PI - r.x, y: TWO_PI - r.y, z: TWO_PI - r.z});
    }
    fn rot(&mut self, r : V3);
    fn rot_by(&mut self, r : V3, p : V3) {
        let mut minus_p = p.clone();
        minus_p.mult(-1.0);
        self.translate(p);
        self.rot(r);
        self.translate(minus_p);
    }
    fn translate(&mut self, p : V3);
    fn scale(&mut self, p : V3);
    fn transform(&mut self) -> Box<&mut dyn Transformable>;
}