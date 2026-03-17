use sdl2::pixels::Color;
use crate::engine::lighting::Material;
use crate::engine::pathtracing::RaytracingObject;
use crate::engine::raymarching::RayMarchingObject;
use crate::engine::utils::rendering::Collision;
use crate::engine::utils::rendering::RaySphereable;
use crate::engine::utils::{rendering::{RayRenderScene, RayRenderable}, transformation::Transformable};
use crate::geometry::vector3::Vector3;

#[derive(Copy, Clone)]
pub struct Sphere {
	m: Vector3,
    r: f64,
    mat: Material,
    disco: bool,
    facet_scale: f64,
    mirror: bool,
    reflectivity: f64,
}

impl Sphere {
    pub fn new(p: Vector3, r_: f64, mat_: Material) -> Self {
        Sphere {
            m: p,
            r: r_,
            mat: mat_,
            disco: false,
            facet_scale: 14.0,
            mirror: false,
            reflectivity: 0.0,
        }
    }

    pub fn disco(p: Vector3, r_: f64) -> Self {
        Sphere {
            m: p,
            r: r_,
            mat: Material::new(Color::RGB(210, 210, 210), 1.0),
            disco: true,
            facet_scale: 18.0,
            mirror: false,
            reflectivity: 0.0,
        }
    }

    pub fn mirror(p: Vector3, r_: f64) -> Self {
        Sphere {
            m: p,
            r: r_,
            mat: Material::new(Color::RGB(210, 210, 220), 1.0),
            disco: false,
            facet_scale: 14.0,
            mirror: true,
            reflectivity: 0.9,
        }
    }

    pub fn d_(self, p : Vector3) -> f64 {
        return self.m.d(p) - self.r;
    }

    pub fn nearest_point_to(self, p : Vector3) -> Vector3 {
        let mut v : Vector3 = self.m.clone();
        let mut res : Vector3 = self.m.clone();
        v.subtract(p);
        v.normalize();
        v.scale(self.r);
        res.add(v);

        return res;
    }
}

impl RaytracingObject for Sphere {
    fn d(&self, p: Vector3) -> f64 {
        self.m.d(p) - self.r
    }

    fn color(&self, _p: Vector3) -> Color {
        self.mat.color
    }

    fn is_colliding(&mut self, p0: Vector3, p: Vector3) -> bool {
        self.get_collision(p0, p).hit
    }

    fn get_collision(&self, p0: Vector3, p: Vector3) -> Collision {
        self.get_collision_with_normal(p0, p).0
    }

    fn get_collision_with_normal(&self, p0: Vector3, p: Vector3) -> (Collision, Option<Vector3>) {
        let mut oc = p0;
        oc.subtract(self.m);

        let a = p.dt(p);
        let b = 2.0 * oc.dt(p);
        let c = oc.dt(oc) - self.r * self.r;

        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return (Collision::empty(), None);
        }

        let sqrt_disc = disc.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        let t = if t1 > 1e-6 {
            t1
        } else if t2 > 1e-6 {
            t2
        } else {
            return (Collision::empty(), None);
        };

        let mut hit = p;
        hit.scale(t);
        hit.add(p0);

        let mut n = hit;
        n.subtract(self.m);
        n.normalize();

        let mut color = self.mat.color;
        if self.disco {
            let theta = f64::atan2(n.z, n.x);
            let phi = f64::acos(f64::clamp(n.y, -1.0, 1.0));
            let ti = f64::floor((theta + std::f64::consts::PI) * self.facet_scale) as i64;
            let pi = f64::floor(phi * self.facet_scale) as i64;
            if ((ti + pi) & 1) == 0 {
                color = Color::RGB(235, 235, 245);
            } else {
                color = Color::RGB(90, 90, 105);
            }
        } else if self.mirror {
            color = Color::RGB(220, 220, 235);
        }

        (
            Collision {
                d: t,
                p: hit,
                hit: true,
                c: color,
            },
            Some(n),
        )
    }

    fn clone(&self) -> Box<dyn RaytracingObject + Send + Sync> {
        Box::new(*self)
    }

    fn reflectivity(&self, _p: Vector3) -> f64 {
        self.reflectivity
    }
}

impl Transformable for Sphere {
    fn rot_reverse(&mut self, p: Vector3) {}
    fn rot(&mut self, p: Vector3) {}

    fn rot_by(&mut self, p : Vector3, r : Vector3) {
        self.m.rot_by(p, r);
    }

    fn translate(&mut self, p: Vector3) {
    	self.m.translate(p.x, p.y, p.z);
    }

    fn scale(&mut self, p : Vector3) {
        self.r *= p.x;
    }

    fn transform(&mut self) -> Box<&mut dyn Transformable> {
        return Box::new(self);
    }
}

impl RaySphereable for Sphere {
    fn get_radius(&self) -> f64 {
        return self.r;
    }

    fn get_middle(&self) -> Vector3 {
        return self.m.clone();
    }
}

impl RayMarchingObject for Sphere {
	fn sdf(&self, p : Vector3) -> f64 {
		return self.d_(p);
	}
    fn get_material(&self) -> &Material {
        &self.mat
    }
    fn clone(&self) -> Box<dyn RayMarchingObject + Send + Sync> {
        Box::new(Sphere {
            m: self.m,
            r: self.r,
            mat: self.mat,
            disco: self.disco,
            facet_scale: self.facet_scale,
            mirror: self.mirror,
            reflectivity: self.reflectivity,
        })
    }
}